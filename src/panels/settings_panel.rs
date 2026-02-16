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

use gpui::{prelude::*, *};
use gpui_component::{
    button::Button,
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme as _, StyledExt as _,
};

use crate::models::{
    file_settings_store::FileSettingsStore, AppSettings, LogLevel, SettingsStore, ThemePreference,
};

/// Settings panel for configuring application preferences.
pub struct SettingsPanel {
    settings: AppSettings,
    store: FileSettingsStore,
    timeout_input: Entity<InputState>,
}

impl SettingsPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let store = FileSettingsStore::new();
        let settings = store.load().unwrap_or_default();
        let timeout_str = settings.default_timeout_ms.to_string();

        let timeout_input = cx.new(|cx| {
            let mut input = InputState::new(window, cx).placeholder("30000");
            input.set_value(&timeout_str, window, cx);
            input
        });

        Self { settings, store, timeout_input }
    }

    fn set_theme(&mut self, theme: ThemePreference, cx: &mut Context<Self>) {
        self.settings.theme = theme;
        cx.notify();
    }

    fn set_log_level(&mut self, level: LogLevel, cx: &mut Context<Self>) {
        self.settings.log_level = level;
        cx.notify();
    }

    fn save_settings(&mut self, cx: &mut Context<Self>) {
        let timeout_str = self.timeout_input.read(cx).value();
        if let Ok(timeout) = timeout_str.trim().parse::<u64>() {
            self.settings.default_timeout_ms = timeout;
        }
        let _ = self.store.save(&self.settings);
        cx.notify();
    }
}

impl SettingsPanel {
    fn theme_button(
        &self,
        id: &'static str,
        label: &'static str,
        value: ThemePreference,
        cx: &mut Context<Self>,
    ) -> Button {
        let btn = Button::new(id).compact().rounded_md();
        let btn = if self.settings.theme == value { btn } else { btn.outline() };
        btn.label(label).on_click(cx.listener(move |this, _, _, cx| {
            this.set_theme(value, cx);
        }))
    }

    fn log_level_button(
        &self,
        id: &'static str,
        label: &'static str,
        value: LogLevel,
        cx: &mut Context<Self>,
    ) -> Button {
        let btn = Button::new(id).compact().rounded_md();
        let btn = if self.settings.log_level == value { btn } else { btn.outline() };
        btn.label(label).on_click(cx.listener(move |this, _, _, cx| {
            this.set_log_level(value, cx);
        }))
    }
}

impl Render for SettingsPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .id("settings-panel")
            .size_full()
            .overflow_y_scroll()
            .p_4()
            .gap_6()
            // Header
            .child(
                div()
                    .text_base()
                    .font_semibold()
                    .text_color(cx.theme().foreground)
                    .child("Settings"),
            )
            // Theme section
            .child(
                v_flex().gap_2().child(div().text_sm().font_semibold().child("Theme")).child(
                    h_flex()
                        .gap_2()
                        .child(self.theme_button(
                            "theme-system",
                            "System",
                            ThemePreference::System,
                            cx,
                        ))
                        .child(self.theme_button(
                            "theme-light",
                            "Light",
                            ThemePreference::Light,
                            cx,
                        ))
                        .child(self.theme_button("theme-dark", "Dark", ThemePreference::Dark, cx)),
                ),
            )
            // Default timeout section
            .child(
                v_flex()
                    .gap_2()
                    .child(div().text_sm().font_semibold().child("Default Timeout (ms)"))
                    .child(Input::new(&self.timeout_input)),
            )
            // Log level section
            .child(
                v_flex().gap_2().child(div().text_sm().font_semibold().child("Log Level")).child(
                    h_flex()
                        .gap_2()
                        .child(self.log_level_button("log-error", "Error", LogLevel::Error, cx))
                        .child(self.log_level_button("log-warn", "Warn", LogLevel::Warn, cx))
                        .child(self.log_level_button("log-info", "Info", LogLevel::Info, cx))
                        .child(self.log_level_button("log-debug", "Debug", LogLevel::Debug, cx))
                        .child(self.log_level_button("log-trace", "Trace", LogLevel::Trace, cx)),
                ),
            )
            // Save button
            .child(h_flex().justify_end().child(
                Button::new("save-settings").compact().label("Save").on_click(cx.listener(
                    |this, _, _, cx| {
                        this.save_settings(cx);
                    },
                )),
            ))
    }
}
