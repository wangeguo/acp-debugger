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

use std::path::PathBuf;

use crate::models::{AppSettings, SettingsStore};

/// Persists application settings to a JSON file at
/// `~/.config/acp-debugger/settings.json`.
pub struct FileSettingsStore {
    path: PathBuf,
}

impl FileSettingsStore {
    pub fn new() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let path = PathBuf::from(home).join(".config").join("acp-debugger").join("settings.json");
        Self { path }
    }
}

impl SettingsStore for FileSettingsStore {
    fn load(&self) -> anyhow::Result<AppSettings> {
        if !self.path.exists() {
            return Ok(AppSettings::default());
        }
        let content = std::fs::read_to_string(&self.path)?;
        let settings = serde_json::from_str(&content)?;
        Ok(settings)
    }

    fn save(&self, settings: &AppSettings) -> anyhow::Result<()> {
        if let Some(parent) = self.path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(settings)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }
}
