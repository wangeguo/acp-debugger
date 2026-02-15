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

use gpui::SharedString;
use serde_json::Value;

/// The type of an ACP message, determined by JSON-RPC 2.0 fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Request,
    Response,
    Notification,
    Error,
}

impl MessageType {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Request => "Request",
            Self::Response => "Response",
            Self::Notification => "Notification",
            Self::Error => "Error",
        }
    }
}

/// ACP error info extracted from a JSON-RPC error response.
#[derive(Debug, Clone)]
pub struct AcpError {
    pub code: i64,
    pub message: SharedString,
    pub data: Option<Value>,
}

/// Parsed ACP message with extracted metadata.
#[derive(Debug, Clone)]
pub struct AcpMessage {
    pub title: SharedString,
    pub raw_json: SharedString,
    pub is_response: bool,
    pub message_type: MessageType,
    pub jsonrpc_version: Option<SharedString>,
    pub id: Option<Value>,
    pub method: Option<SharedString>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<AcpError>,
}

impl AcpMessage {
    /// Parse a raw JSON string into an AcpMessage.
    pub fn parse(
        title: impl Into<SharedString>,
        raw_json: impl Into<SharedString>,
        is_response: bool,
    ) -> Self {
        let title = title.into();
        let raw_json = raw_json.into();

        let parsed: Value = serde_json::from_str(&raw_json).unwrap_or(Value::Null);

        let jsonrpc_version = parsed
            .get("jsonrpc")
            .and_then(|v| v.as_str())
            .map(|s| SharedString::from(s.to_string()));

        let id = parsed.get("id").cloned();
        let method = parsed
            .get("method")
            .and_then(|v| v.as_str())
            .map(|s| SharedString::from(s.to_string()));
        let params = parsed.get("params").cloned();
        let result = parsed.get("result").cloned();

        let error = parsed.get("error").and_then(|e| {
            Some(AcpError {
                code: e.get("code")?.as_i64()?,
                message: SharedString::from(e.get("message")?.as_str()?.to_string()),
                data: e.get("data").cloned(),
            })
        });

        let message_type = if error.is_some() {
            MessageType::Error
        } else if result.is_some() {
            MessageType::Response
        } else if id.is_some() && method.is_some() {
            MessageType::Request
        } else if method.is_some() {
            MessageType::Notification
        } else {
            MessageType::Response
        };

        Self {
            title,
            raw_json,
            is_response,
            message_type,
            jsonrpc_version,
            id,
            method,
            params,
            result,
            error,
        }
    }

    /// Get the main payload (params or result) as pretty-printed JSON.
    pub fn payload_json(&self) -> Option<String> {
        let value = self
            .params
            .as_ref()
            .or(self.result.as_ref())
            .or(self.error.as_ref().and_then(|e| e.data.as_ref()));

        value.and_then(|v| serde_json::to_string_pretty(v).ok())
    }

    /// Get the full raw JSON pretty-printed.
    pub fn pretty_json(&self) -> String {
        serde_json::from_str::<Value>(&self.raw_json)
            .ok()
            .and_then(|v| serde_json::to_string_pretty(&v).ok())
            .unwrap_or_else(|| self.raw_json.to_string())
    }

    /// Get the direction label.
    pub fn direction_label(&self) -> &'static str {
        if self.is_response {
            "\u{2191} Incoming"
        } else {
            "\u{2193} Outgoing"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_request() {
        let msg = AcpMessage::parse(
            "initialize",
            r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{}}"#,
            false,
        );
        assert_eq!(msg.message_type, MessageType::Request);
        assert_eq!(msg.method.as_ref().map(|s| s.as_ref()), Some("initialize"));
        assert!(msg.id.is_some());
        assert!(msg.params.is_some());
        assert!(!msg.is_response);
    }

    #[test]
    fn test_parse_response() {
        let msg = AcpMessage::parse(
            "initialize",
            r#"{"jsonrpc":"2.0","id":0,"result":{"protocolVersion":1}}"#,
            true,
        );
        assert_eq!(msg.message_type, MessageType::Response);
        assert!(msg.result.is_some());
        assert!(msg.is_response);
    }

    #[test]
    fn test_parse_notification() {
        let msg = AcpMessage::parse(
            "session/update",
            r#"{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"abc"}}"#,
            true,
        );
        assert_eq!(msg.message_type, MessageType::Notification);
        assert!(msg.id.is_none());
        assert!(msg.method.is_some());
    }

    #[test]
    fn test_parse_error() {
        let msg = AcpMessage::parse(
            "initialize",
            r#"{"jsonrpc":"2.0","id":0,"error":{"code":-32600,"message":"Invalid Request"}}"#,
            true,
        );
        assert_eq!(msg.message_type, MessageType::Error);
        assert!(msg.error.is_some());
        let err = msg.error.unwrap();
        assert_eq!(err.code, -32600);
        assert_eq!(err.message.as_ref(), "Invalid Request");
    }

    #[test]
    fn test_pretty_json() {
        let msg = AcpMessage::parse("test", r#"{"jsonrpc":"2.0","id":0}"#, false);
        let pretty = msg.pretty_json();
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("jsonrpc"));
    }
}
