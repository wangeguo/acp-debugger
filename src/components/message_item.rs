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
use gpui_component::{ActiveTheme as _, StyledExt};

#[derive(IntoElement)]
pub struct MessageItem {
    title: SharedString,
    json_content: SharedString,
    is_response: bool,
}

impl MessageItem {
    pub fn new(
        title: impl Into<SharedString>,
        json_content: impl Into<SharedString>,
        is_response: bool,
    ) -> Self {
        Self { title: title.into(), json_content: json_content.into(), is_response }
    }

    fn arrow_icon(&self) -> &'static str {
        if self.is_response {
            "↑" // Response: coming up from agent
        } else {
            "↓" // Request: going down to agent
        }
    }

    fn arrow_color(&self) -> Hsla {
        if self.is_response {
            rgb(0xa3be8c).into() // Green for response
        } else {
            rgb(0x5e81ac).into() // Blue for request
        }
    }
}

impl RenderOnce for MessageItem {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let arrow = self.arrow_icon();
        let arrow_color = self.arrow_color();

        div()
            .flex()
            .flex_col()
            .bg(cx.theme().tab_bar)
            .border_1()
            .border_color(cx.theme().border)
            .rounded_md()
            .px_2()
            .py_1p5()
            .mb_1()
            // Header row: arrow + title
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_1p5()
                    .child(div().text_sm().font_semibold().text_color(arrow_color).child(arrow))
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(self.title.clone()),
                    ),
            )
            // JSON content area
            .child(
                div()
                    .mt_1p5()
                    .px_2()
                    .py_1()
                    .bg(cx.theme().secondary)
                    .rounded(px(4.))
                    .overflow_x_hidden()
                    .child(
                        div()
                            .text_sm()
                            .font_family("monospace")
                            .text_color(cx.theme().secondary_foreground)
                            .overflow_hidden()
                            .text_ellipsis()
                            .child(self.json_content.clone()),
                    ),
            )
    }
}
