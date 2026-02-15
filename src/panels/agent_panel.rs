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

use gpui::{actions, prelude::*, *};
use gpui_component::{
    button::Button,
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme as _, IconName, Sizable as _,
};

use crate::components::{
    ChatMessage, ChatMessageVariant, PlanEntry, PlanEntryStatus, Role, ToolCallKind, ToolCallStatus,
};

actions!(agent_panel, [SendMessage]);

struct ChatMessageData {
    variant: ChatMessageVariant,
}

pub struct AgentPanel {
    input: Entity<InputState>,
    messages: Vec<ChatMessageData>,
    scroll_handle: ScrollHandle,
}

impl AgentPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Message the Custom Agent — @ to include context, / for commands")
                .multi_line(true)
                .auto_grow(3, 10)
                .soft_wrap(true)
        });

        // Pre-load demo data to showcase all message variants
        let messages = vec![
            // User text message
            ChatMessageData {
                variant: ChatMessageVariant::Text {
                    role: Role::User,
                    content: "Can you analyze this code and fix the failing tests?".into(),
                },
            },
            // Agent thinking
            ChatMessageData {
                variant: ChatMessageVariant::Thought {
                    content: "I'll start by reading the project structure to understand the codebase layout. Then I'll look at the test files to identify which tests are failing and why.".into(),
                },
            },
            // Tool call - read (completed, with content)
            ChatMessageData {
                variant: ChatMessageVariant::ToolCall {
                    title: "Read src/main.rs".into(),
                    kind: ToolCallKind::Read,
                    status: ToolCallStatus::Completed,
                    content: Some("fn main() {\n    println!(\"Hello, world!\");\n}".into()),
                },
            },
            // Tool call - search (completed)
            ChatMessageData {
                variant: ChatMessageVariant::ToolCall {
                    title: "Search for test files".into(),
                    kind: ToolCallKind::Search,
                    status: ToolCallStatus::Completed,
                    content: None,
                },
            },
            // Tool call - edit (in_progress)
            ChatMessageData {
                variant: ChatMessageVariant::ToolCall {
                    title: "Edit src/lib.rs".into(),
                    kind: ToolCallKind::Edit,
                    status: ToolCallStatus::InProgress,
                    content: None,
                },
            },
            // Tool call - execute (pending)
            ChatMessageData {
                variant: ChatMessageVariant::ToolCall {
                    title: "Run cargo test".into(),
                    kind: ToolCallKind::Execute,
                    status: ToolCallStatus::Pending,
                    content: None,
                },
            },
            // Tool call - failed
            ChatMessageData {
                variant: ChatMessageVariant::ToolCall {
                    title: "Write /etc/config".into(),
                    kind: ToolCallKind::Edit,
                    status: ToolCallStatus::Failed,
                    content: None,
                },
            },
            // Agent text reply
            ChatMessageData {
                variant: ChatMessageVariant::Text {
                    role: Role::Assistant,
                    content: "I've analyzed the code and found the issue. The test was expecting a return value of `42` but the function returns `0`. I've fixed this in `src/lib.rs`.".into(),
                },
            },
            // Execution plan
            ChatMessageData {
                variant: ChatMessageVariant::Plan {
                    entries: vec![
                        PlanEntry {
                            content: "Read project structure".into(),
                            status: PlanEntryStatus::Completed,
                        },
                        PlanEntry {
                            content: "Analyze failing tests".into(),
                            status: PlanEntryStatus::Completed,
                        },
                        PlanEntry {
                            content: "Fix implementation in lib.rs".into(),
                            status: PlanEntryStatus::InProgress,
                        },
                        PlanEntry {
                            content: "Run tests to verify".into(),
                            status: PlanEntryStatus::Pending,
                        },
                        PlanEntry {
                            content: "Update documentation".into(),
                            status: PlanEntryStatus::Pending,
                        },
                    ],
                },
            },
            // System notification
            ChatMessageData {
                variant: ChatMessageVariant::System {
                    content: "Mode changed to \"Accept Edits\"".into(),
                },
            },
        ];

        Self { input, messages, scroll_handle: ScrollHandle::new() }
    }

    pub fn init(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("cmd-enter", SendMessage, None)]);
    }

    fn send_message(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.input.read(cx).value();
        let content = content.trim();
        if content.is_empty() {
            return;
        }

        self.messages.push(ChatMessageData {
            variant: ChatMessageVariant::Text {
                role: Role::User,
                content: content.to_string().into(),
            },
        });

        self.input.update(cx, |state, cx| {
            state.set_value("", window, cx);
        });

        // Scroll to bottom after adding new message
        let item_count = self.messages.len();
        self.scroll_handle.scroll_to_item(item_count.saturating_sub(1));

        cx.notify();
    }
}

impl AgentPanel {
    fn header(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .px_3()
            .py_2()
            .border_b_1()
            .border_color(cx.theme().title_bar_border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(IconName::SquareTerminal)
                    .child("New Thread"),
            )
            .child(Button::new("new-thread").rounded_lg().outline().icon(IconName::Plus))
    }

    fn prompt_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap_2()
            .border_t_1()
            .border_color(cx.theme().border)
            .p_3()
            .child(
                // Input area - first row
                div()
                    .flex()
                    .bg(cx.theme().input)
                    .rounded_md()
                    .border_1()
                    .border_color(cx.theme().border)
                    .child(Input::new(&self.input).appearance(false)),
            )
            .child(
                // Bottom row - left icons and right buttons
                h_flex()
                    .justify_between()
                    .child(
                        // Left side icons
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new("attach")
                                    .xsmall()
                                    .rounded_md()
                                    .child(IconName::Asterisk),
                            )
                            .child(
                                Button::new("context")
                                    .xsmall()
                                    .rounded_md()
                                    .child(IconName::CircleUser),
                            ),
                    )
                    .child(
                        // Right side dropdowns and send button
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Button::new("model-selector").xsmall().rounded_md().child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child("Default")
                                        .child(IconName::ChevronDown),
                                ),
                            )
                            .child(
                                Button::new("mode-selector").xsmall().rounded_md().child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child("Default (recommended)")
                                        .child(IconName::ChevronDown),
                                ),
                            )
                            .child(
                                Button::new("send")
                                    .xsmall()
                                    .rounded_md()
                                    .child(IconName::ArrowRight)
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.send_message(window, cx);
                                    })),
                            ),
                    ),
            )
    }
}

impl Render for AgentPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_hidden()
            .on_action(cx.listener(|this, _: &SendMessage, window, cx| {
                this.send_message(window, cx);
            }))
            .child(self.header(cx))
            .child(
                v_flex()
                    .id("scrollable-agent-container")
                    .flex_1()
                    .overflow_y_scroll()
                    .p_4()
                    .children(self.messages.iter().map(|msg| {
                        let variant = msg.variant.clone();
                        match variant {
                            ChatMessageVariant::Text { role, content } => {
                                ChatMessage::text(role, content)
                            }
                            ChatMessageVariant::Thought { content } => {
                                ChatMessage::thought(content)
                            }
                            ChatMessageVariant::ToolCall { title, kind, status, content } => {
                                if let Some(c) = content {
                                    ChatMessage::tool_call_with_content(title, kind, status, c)
                                } else {
                                    ChatMessage::tool_call(title, kind, status)
                                }
                            }
                            ChatMessageVariant::Plan { entries } => ChatMessage::plan(entries),
                            ChatMessageVariant::System { content } => ChatMessage::system(content),
                        }
                    }))
                    .track_scroll(&self.scroll_handle),
            )
            .child(self.prompt_input(cx))
    }
}
