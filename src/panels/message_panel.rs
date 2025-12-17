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
use gpui_component::{scroll::ScrollableElement, v_flex};

use crate::components::MessageItem;

#[derive(IntoElement)]
pub struct MessagePanel;

impl RenderOnce for MessagePanel {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .id("scrollable-messages-container")
            .size_full()
            .bg(rgb(0x2e3440))
            .overflow_y_scrollbar()
            .p_4()
            .child(MessageItem::new("initialize", false))
            .child(MessageItem::new("initialize response", true))
            .child(MessageItem::new("authenticate", false))
            .child(MessageItem::new("authenticate response", true))
            .child(MessageItem::new("session/new", false))
            .child(MessageItem::new("session/new response", true))
            .child(MessageItem::new("tools/list", false))
            .child(MessageItem::new("tools/list response", true))
            .child(MessageItem::new("prompts/get", false))
            .child(MessageItem::new("prompts/get response", true))
            .child(MessageItem::new("resources/read", false))
            .child(MessageItem::new("resources/read response", true))
            .child(MessageItem::new("completion/complete", false))
            .child(MessageItem::new("completion/complete response", true))
            .child(MessageItem::new("logging/setLevel", false))
            .child(MessageItem::new("logging/setLevel response", true))
            .child(MessageItem::new("sampling/createMessage", false))
            .child(MessageItem::new("sampling/createMessage response", true))
            .child(MessageItem::new("roots/list", false))
            .child(MessageItem::new("roots/list response", true))
    }
}
