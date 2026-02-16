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

#![allow(dead_code)]

use super::agent::AgentId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueueItemStatus {
    Pending,
    InFlight,
    Success,
    Failed,
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    pub id: u64,
    pub agent_id: AgentId,
    pub method: String,
    pub request_json: String,
    pub response_json: Option<String>,
    pub status: QueueItemStatus,
    pub created_at: std::time::Instant,
    pub completed_at: Option<std::time::Instant>,
    pub error_message: Option<String>,
}
