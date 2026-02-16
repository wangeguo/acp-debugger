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

use gpui::*;
use serde_json::Value;

use super::{connection::AcpConnection, error::ClientError};

/// Session management methods on AcpConnection.
impl AcpConnection {
    /// Create a new session.
    ///
    /// Sends `session/new` and returns the session ID.
    pub fn new_session(
        &mut self,
        cwd: &str,
        cx: &mut Context<Self>,
    ) -> Task<Result<String, ClientError>> {
        let params = serde_json::json!({ "cwd": cwd });
        let task = self.send_request("session/new", params, cx);

        cx.spawn(async move |_, _| {
            let result = task.await?;
            result
                .get("sessionId")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
                .ok_or_else(|| ClientError::ParseError("missing sessionId in response".into()))
        })
    }

    /// Send a prompt to an existing session.
    ///
    /// Session updates arrive as `ConnectionEvent::SessionNotification`.
    pub fn prompt(
        &mut self,
        session_id: &str,
        messages: Vec<Value>,
        cx: &mut Context<Self>,
    ) -> Task<Result<Value, ClientError>> {
        let params = serde_json::json!({
            "sessionId": session_id,
            "prompt": messages,
        });
        self.send_request("session/prompt", params, cx)
    }

    /// Cancel an ongoing prompt in a session (notification, no response).
    pub fn cancel(&mut self, session_id: &str, cx: &mut Context<Self>) -> Result<(), ClientError> {
        let params = serde_json::json!({ "sessionId": session_id });
        self.send_notification("session/cancel", params, cx)
    }

    /// Send an authenticate request.
    pub fn authenticate(
        &mut self,
        method: &str,
        credentials: Value,
        cx: &mut Context<Self>,
    ) -> Task<Result<Value, ClientError>> {
        let params = serde_json::json!({
            "method": method,
            "credentials": credentials,
        });
        self.send_request("authenticate", params, cx)
    }
}
