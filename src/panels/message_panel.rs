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
use gpui_component::{scroll::ScrollableElement, v_flex, WindowExt};

use crate::{components::MessageItem, panels::DetailPanel};

#[derive(IntoElement)]
pub struct MessagePanel;

impl MessagePanel {
    fn message(title: impl Into<SharedString>, is_response: bool) -> impl IntoElement {
        div()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                window.open_sheet(cx, |sheet, _, _| {
                    sheet
                        .size_full()
                        .margin_top(px(0.))
                        .size(px(400.))
                        .title("Message details")
                        .child(DetailPanel)
                })
            })
            .child(MessageItem::new(title, is_response))
    }
}

impl RenderOnce for MessagePanel {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex()
            .id("scrollable-messages-container")
            .size_full()
            .overflow_y_scrollbar()
            .p_4()
            .child(Self::message("initialize", false))
            .child(Self::message("initialize response", true))
            .child(Self::message("authenticate", false))
            .child(Self::message("authenticate response", true))
            .child(Self::message("session/new", false))
            .child(Self::message("session/new response", true))
            .child(Self::message("tools/list", false))
            .child(Self::message("tools/list response", true))
            .child(Self::message("prompts/get", false))
            .child(Self::message("prompts/get response", true))
            .child(Self::message("resources/read", false))
            .child(Self::message("resources/read response", true))
            .child(Self::message("completion/complete", false))
            .child(Self::message("completion/complete response", true))
            .child(Self::message("logging/setLevel", false))
            .child(Self::message("logging/setLevel response", true))
            .child(Self::message("sampling/createMessage", false))
            .child(Self::message("sampling/createMessage response", true))
            .child(Self::message("roots/list", false))
            .child(Self::message("roots/list response", true))
    }
}
