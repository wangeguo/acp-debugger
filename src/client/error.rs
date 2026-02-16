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

use thiserror::Error;

/// Errors that can occur during ACP client operations.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("failed to spawn agent process: {0}")]
    SpawnFailed(#[source] std::io::Error),

    #[error("transport closed")]
    TransportClosed,

    #[error("failed to send message: {0}")]
    SendFailed(String),

    #[error("failed to parse message: {0}")]
    ParseError(String),

    #[error("request timed out (id={0})")]
    Timeout(u64),

    #[error("request cancelled (id={0})")]
    Cancelled(u64),

    #[error("agent returned error: code={code}, message={message}")]
    AgentError { code: i64, message: String, data: Option<serde_json::Value> },

    #[error("not connected")]
    NotConnected,

    #[error("{0}")]
    Other(#[from] anyhow::Error),
}
