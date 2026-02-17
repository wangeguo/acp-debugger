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

use std::time::{SystemTime, UNIX_EPOCH};

use gpui::{prelude::*, *};
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputState},
    radio::RadioGroup,
    v_flex, ActiveTheme as _, StyledExt as _, WindowExt as _,
};

use crate::models::{AgentConfig, AgentRegistry, AuthMethod};

pub struct AgentForm {
    name_input: Entity<InputState>,
    endpoint_input: Entity<InputState>,
    args_input: Entity<InputState>,
    env_input: Entity<InputState>,
    auth_method_index: Option<usize>,
    auth_credential_input: Entity<InputState>,
    timeout_input: Entity<InputState>,
    error_message: Option<SharedString>,
    registry: Entity<AgentRegistry>,
}

impl AgentForm {
    pub fn new(
        registry: Entity<AgentRegistry>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let name_input = cx.new(|cx| InputState::new(window, cx).placeholder("Agent name"));

        let endpoint_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("/path/to/agent or command"));

        let args_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("One argument per line")
                .multi_line(true)
                .auto_grow(2, 5)
        });

        let env_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("KEY=VALUE (one per line)")
                .multi_line(true)
                .auto_grow(2, 5)
        });

        let auth_credential_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("API key or Bearer token"));

        let timeout_input = cx.new(|cx| InputState::new(window, cx).placeholder("30000"));

        Self {
            name_input,
            endpoint_input,
            args_input,
            env_input,
            auth_method_index: Some(0),
            auth_credential_input,
            timeout_input,
            error_message: None,
            registry,
        }
    }

    fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = self.name_input.read(cx).value().trim().to_string();
        let endpoint = self.endpoint_input.read(cx).value().trim().to_string();

        if name.is_empty() {
            self.error_message = Some("Name is required".into());
            cx.notify();
            return;
        }
        if endpoint.is_empty() {
            self.error_message = Some("Endpoint is required".into());
            cx.notify();
            return;
        }

        let args: Vec<String> = self
            .args_input
            .read(cx)
            .value()
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();

        let env: Vec<(String, String)> = self
            .env_input
            .read(cx)
            .value()
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                let (k, v) = line.split_once('=')?;
                Some((k.trim().to_string(), v.trim().to_string()))
            })
            .collect();

        let credential = self.auth_credential_input.read(cx).value().trim().to_string();
        let auth = match self.auth_method_index {
            Some(1) => AuthMethod::ApiKey { key: credential },
            Some(2) => AuthMethod::Bearer { token: credential },
            _ => AuthMethod::None,
        };

        let timeout_ms: u64 = self.timeout_input.read(cx).value().trim().parse().unwrap_or(30_000);

        let id = generate_agent_id();

        let config = AgentConfig { id: id.clone(), name, endpoint, args, env, auth, timeout_ms };

        self.registry.update(cx, |reg, _| {
            reg.add_agent(config);
            reg.set_active(Some(id));
        });

        self.error_message = None;
        window.close_sheet(cx);
    }
}

impl Render for AgentForm {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let auth_method_index = self.auth_method_index;
        let show_credential = matches!(auth_method_index, Some(1) | Some(2));
        let entity = cx.entity().clone();

        v_flex()
            .id("agent-form")
            .size_full()
            .overflow_y_scroll()
            .gap_4()
            .p_4()
            // Name (required)
            .child(
                v_flex()
                    .gap_1()
                    .child(field_label("Name", true, cx))
                    .child(Input::new(&self.name_input)),
            )
            // Endpoint (required)
            .child(
                v_flex()
                    .gap_1()
                    .child(field_label("Endpoint", true, cx))
                    .child(Input::new(&self.endpoint_input)),
            )
            // Arguments
            .child(
                v_flex()
                    .gap_1()
                    .child(field_label("Arguments", false, cx))
                    .child(Input::new(&self.args_input)),
            )
            // Environment
            .child(
                v_flex()
                    .gap_1()
                    .child(field_label("Environment", false, cx))
                    .child(Input::new(&self.env_input)),
            )
            // Auth Method
            .child(
                v_flex().gap_1().child(field_label("Auth Method", false, cx)).child(
                    RadioGroup::horizontal("auth-method")
                        .selected_index(auth_method_index)
                        .child("None")
                        .child("API Key")
                        .child("Bearer")
                        .on_click(move |ix, _window, cx| {
                            entity.update(cx, |this, cx| {
                                this.auth_method_index = Some(*ix);
                                cx.notify();
                            });
                        }),
                ),
            )
            // Auth credential (conditional)
            .when(show_credential, |el| {
                let label = if auth_method_index == Some(1) { "API Key" } else { "Bearer Token" };
                el.child(
                    v_flex()
                        .gap_1()
                        .child(field_label(label, false, cx))
                        .child(Input::new(&self.auth_credential_input)),
                )
            })
            // Timeout
            .child(
                v_flex()
                    .gap_1()
                    .child(field_label("Timeout (ms)", false, cx))
                    .child(Input::new(&self.timeout_input)),
            )
            // Error message
            .when_some(self.error_message.clone(), |el, msg| {
                el.child(div().text_sm().text_color(cx.theme().danger).child(msg))
            })
            // Buttons
            .child(
                h_flex()
                    .justify_end()
                    .gap_2()
                    .child(Button::new("cancel").label("Cancel").on_click(cx.listener(
                        |_, _, window, cx| {
                            window.close_sheet(cx);
                        },
                    )))
                    .child(Button::new("create").primary().label("Create Agent").on_click(
                        cx.listener(|this, _, window, cx| {
                            this.submit(window, cx);
                        }),
                    )),
            )
    }
}

fn field_label(text: &str, required: bool, cx: &Context<AgentForm>) -> impl IntoElement {
    let el = h_flex().gap_1().child(
        div().text_sm().font_semibold().text_color(cx.theme().foreground).child(text.to_string()),
    );
    if required {
        el.child(div().text_sm().text_color(cx.theme().danger).child("*"))
    } else {
        el
    }
}

fn generate_agent_id() -> String {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos();
    format!("agent-{:x}", ts)
}
