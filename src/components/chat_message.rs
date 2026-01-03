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
use gpui_component::ActiveTheme as _;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
    System,
}

#[derive(IntoElement)]
pub struct ChatMessage {
    content: SharedString,
    role: Role,
}

impl ChatMessage {
    pub fn new(content: impl Into<SharedString>, role: Role) -> Self {
        Self {
            content: content.into(),
            role,
        }
    }
}

impl RenderOnce for ChatMessage {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (bg_color, border_color) = match self.role {
            Role::User => (cx.theme().colors.secondary, cx.theme().colors.border),
            Role::Assistant => (cx.theme().colors.accent, cx.theme().colors.border),
            Role::System => (cx.theme().colors.muted, cx.theme().colors.border),
        };

        div()
            .flex_shrink_0()
            .px_2()
            .py_1()
            .mb_2()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded_md()
            .min_w_0()
            .overflow_hidden()
            .text_sm()
            .text_color(cx.theme().colors.foreground)
            .child(self.content)
    }
}
