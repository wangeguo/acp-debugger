// Copyright (c) wangeguo. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::HashMap;

use futures::{channel::oneshot, StreamExt};
use gpui::*;
use serde_json::Value;

use crate::models::{AcpMessage, AgentConfig, AgentStatus};

use super::{
    error::ClientError,
    transport::{StdioTransport, TransportMessage},
};

/// Events emitted by AcpConnection for UI subscribers.
#[derive(Debug, Clone)]
pub enum ConnectionEvent {
    /// Connection state changed.
    StateChanged(AgentStatus),

    /// A raw JSON-RPC message was sent or received (for debugger UI).
    MessageCaptured { message: Box<AcpMessage>, is_incoming: bool },

    /// A session notification received from the agent.
    SessionNotification { session_id: String, method: String, params: Value },
}

struct PendingRequest {
    #[allow(dead_code)]
    method: String,
    sender: oneshot::Sender<Result<Value, ClientError>>,
}

/// GPUI Entity managing the ACP client connection lifecycle.
///
/// Handles subprocess spawning, JSON-RPC message dispatch,
/// and request-response correlation. Emits events for every
/// captured message so the debugger UI can display them.
pub struct AcpConnection {
    status: AgentStatus,
    config: Option<AgentConfig>,
    transport: Option<StdioTransport>,
    next_request_id: u64,
    pending_requests: HashMap<u64, PendingRequest>,
    _message_pump_task: Option<Task<()>>,
}

impl EventEmitter<ConnectionEvent> for AcpConnection {}

impl AcpConnection {
    /// Create a new disconnected connection.
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            status: AgentStatus::Disconnected,
            config: None,
            transport: None,
            next_request_id: 0,
            pending_requests: HashMap::new(),
            _message_pump_task: None,
        }
    }

    /// Current connection status.
    pub fn status(&self) -> AgentStatus {
        self.status
    }

    /// Connect to an agent using the given config.
    ///
    /// Spawns the child process, starts the message pump,
    /// and initiates the ACP handshake (initialize).
    pub fn connect(&mut self, config: AgentConfig, cx: &mut Context<Self>) {
        if self.status == AgentStatus::Connected || self.status == AgentStatus::Connecting {
            return;
        }

        self.set_status(AgentStatus::Connecting, cx);
        self.config = Some(config.clone());

        let mut transport = match StdioTransport::spawn(&config) {
            Ok(t) => t,
            Err(_) => {
                self.set_status(AgentStatus::Error, cx);
                return;
            }
        };

        let incoming_rx = transport.take_incoming();
        self.transport = Some(transport);

        if let Some(incoming_rx) = incoming_rx {
            self.start_message_pump(incoming_rx, cx);
        }

        self.send_initialize(cx);
    }

    /// Disconnect from the agent.
    pub fn disconnect(&mut self, cx: &mut Context<Self>) {
        if let Some(transport) = self.transport.as_mut() {
            transport.kill();
        }
        self.transport = None;
        self.config = None;
        self._message_pump_task = None;

        for (id, pending) in self.pending_requests.drain() {
            let _ = pending.sender.send(Err(ClientError::Cancelled(id)));
        }

        self.set_status(AgentStatus::Disconnected, cx);
    }

    /// Send a JSON-RPC request and return a Task resolving to the response.
    pub fn send_request(
        &mut self,
        method: &str,
        params: Value,
        cx: &mut Context<Self>,
    ) -> Task<Result<Value, ClientError>> {
        let id = self.next_id();
        let request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params,
        });
        let json_str = serde_json::to_string(&request).unwrap();

        // Capture outgoing message for debugger UI
        let method_owned = method.to_string();
        let acp_msg = AcpMessage::parse(&method_owned, &json_str, false);
        cx.emit(ConnectionEvent::MessageCaptured {
            message: Box::new(acp_msg),
            is_incoming: false,
        });

        let (tx, rx) = oneshot::channel();
        self.pending_requests.insert(id, PendingRequest { method: method_owned, sender: tx });

        if let Some(transport) = &self.transport {
            if let Err(e) = transport.send(json_str) {
                self.pending_requests.remove(&id);
                return cx.spawn(async move |_, _| Err(e));
            }
        } else {
            self.pending_requests.remove(&id);
            return cx.spawn(async move |_, _| Err(ClientError::NotConnected));
        }

        cx.spawn(async move |_, _| rx.await.map_err(|_| ClientError::TransportClosed)?)
    }

    /// Send a JSON-RPC notification (no response expected).
    pub fn send_notification(
        &mut self,
        method: &str,
        params: Value,
        cx: &mut Context<Self>,
    ) -> Result<(), ClientError> {
        let notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": method,
            "params": params,
        });
        let json_str = serde_json::to_string(&notification).unwrap();

        let method_owned = method.to_string();
        let acp_msg = AcpMessage::parse(&method_owned, &json_str, false);
        cx.emit(ConnectionEvent::MessageCaptured {
            message: Box::new(acp_msg),
            is_incoming: false,
        });

        if let Some(transport) = &self.transport {
            transport.send(json_str)
        } else {
            Err(ClientError::NotConnected)
        }
    }
}

// Internal methods
impl AcpConnection {
    fn next_id(&mut self) -> u64 {
        let id = self.next_request_id;
        self.next_request_id += 1;
        id
    }

    fn set_status(&mut self, status: AgentStatus, cx: &mut Context<Self>) {
        self.status = status;
        cx.emit(ConnectionEvent::StateChanged(status));
        cx.notify();
    }

    /// Start the async message pump that reads from the transport
    /// incoming channel and dispatches messages.
    fn start_message_pump(
        &mut self,
        mut incoming_rx: futures::channel::mpsc::UnboundedReceiver<TransportMessage>,
        cx: &mut Context<Self>,
    ) {
        self._message_pump_task = Some(cx.spawn(async move |this, cx| {
            while let Some(msg) = incoming_rx.next().await {
                let Some(entity) = this.upgrade() else {
                    break;
                };
                let Ok(should_continue) =
                    cx.update_entity(&entity, |conn, cx| conn.handle_transport_message(msg, cx))
                else {
                    break;
                };
                if !should_continue {
                    break;
                }
            }

            // Transport closed: update status
            if let Some(entity) = this.upgrade() {
                let _ = cx.update_entity(&entity, |conn, cx| {
                    if conn.status != AgentStatus::Disconnected {
                        conn.set_status(AgentStatus::Disconnected, cx);
                    }
                });
            }
        }));
    }

    /// Handle a single transport message.
    /// Returns true to continue pumping, false to stop.
    fn handle_transport_message(&mut self, msg: TransportMessage, cx: &mut Context<Self>) -> bool {
        match msg {
            TransportMessage::Json(value) => {
                let raw = serde_json::to_string(&value).unwrap_or_default();
                let title =
                    value.get("method").and_then(|v| v.as_str()).unwrap_or("response").to_string();
                let is_response = value.get("result").is_some() || value.get("error").is_some();

                // Capture every incoming message for debugger
                let acp_msg = AcpMessage::parse(&title, &raw, true);
                cx.emit(ConnectionEvent::MessageCaptured {
                    message: Box::new(acp_msg),
                    is_incoming: true,
                });

                // Dispatch response to pending request
                if is_response {
                    if let Some(id) = value.get("id").and_then(|v| v.as_u64()) {
                        self.handle_response(id, &value);
                    } else {
                        log::warn!(
                            "Received JSON-RPC response with missing or non-u64 id: {}",
                            raw
                        );
                    }
                }

                // Dispatch notification
                if !is_response {
                    if let Some(method) = value.get("method").and_then(|v| v.as_str()) {
                        let params = value.get("params").cloned().unwrap_or(Value::Null);
                        self.handle_notification(method, &params, cx);
                    }
                }

                true
            }
            TransportMessage::ParseError { .. } => true,
            TransportMessage::Closed => false,
        }
    }

    fn handle_response(&mut self, id: u64, value: &Value) {
        if let Some(pending) = self.pending_requests.remove(&id) {
            if let Some(error) = value.get("error") {
                let code = error.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
                let message = error
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("unknown error")
                    .to_string();
                let data = error.get("data").cloned();
                let _ = pending.sender.send(Err(ClientError::AgentError { code, message, data }));
            } else if let Some(result) = value.get("result") {
                let _ = pending.sender.send(Ok(result.clone()));
            } else {
                let _ = pending.sender.send(Ok(Value::Null));
            }
        }
    }

    fn handle_notification(&mut self, method: &str, params: &Value, cx: &mut Context<Self>) {
        if let Some(session_id) = params.get("sessionId").and_then(|v| v.as_str()) {
            cx.emit(ConnectionEvent::SessionNotification {
                session_id: session_id.to_string(),
                method: method.to_string(),
                params: params.clone(),
            });
        }
    }

    /// Send the initialize request as part of the handshake.
    fn send_initialize(&mut self, cx: &mut Context<Self>) {
        let params = serde_json::json!({
            "protocolVersion": 1,
            "clientCapabilities": {
                "fs": {
                    "readTextFile": true,
                    "writeTextFile": true
                },
                "terminal": true
            },
            "clientInfo": {
                "name": "acp-debugger",
                "title": "ACP Debugger",
                "version": env!("CARGO_PKG_VERSION")
            }
        });

        let task = self.send_request("initialize", params, cx);

        cx.spawn(async move |this, cx| match task.await {
            Ok(_) => {
                if let Some(entity) = this.upgrade() {
                    let _ = cx.update_entity(&entity, |conn, cx| {
                        conn.set_status(AgentStatus::Connected, cx);
                    });
                }
            }
            Err(_) => {
                if let Some(entity) = this.upgrade() {
                    let _ = cx.update_entity(&entity, |conn, cx| {
                        conn.set_status(AgentStatus::Error, cx);
                    });
                }
            }
        })
        .detach();
    }
}
