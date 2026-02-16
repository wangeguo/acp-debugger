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

use serde::{Deserialize, Serialize};

pub type AgentId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AuthMethod {
    None,
    ApiKey { key: String },
    Bearer { token: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: AgentId,
    pub name: String,
    pub endpoint: String,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub auth: AuthMethod,
    pub timeout_ms: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

pub struct AgentRegistry {
    agents: Vec<AgentConfig>,
    active_agent_id: Option<AgentId>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            active_agent_id: None,
        }
    }

    pub fn agents(&self) -> &[AgentConfig] {
        &self.agents
    }

    pub fn active_agent_id(&self) -> Option<&AgentId> {
        self.active_agent_id.as_ref()
    }

    pub fn active_agent(&self) -> Option<&AgentConfig> {
        self.active_agent_id
            .as_ref()
            .and_then(|id| self.agents.iter().find(|a| &a.id == id))
    }

    pub fn add_agent(&mut self, config: AgentConfig) {
        self.agents.push(config);
    }

    pub fn remove_agent(&mut self, id: &str) {
        self.agents.retain(|a| a.id != id);
        if self.active_agent_id.as_deref() == Some(id) {
            self.active_agent_id = None;
        }
    }

    pub fn set_active(&mut self, id: Option<AgentId>) {
        self.active_agent_id = id;
    }
}
