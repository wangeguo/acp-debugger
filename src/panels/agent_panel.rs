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
use gpui_component::{button::Button, v_flex, ActiveTheme as _, IconName, Sizable as _};

#[derive(IntoElement)]
pub struct AgentPanel;

impl AgentPanel {
    fn header(&self, cx: &App) -> impl IntoElement {
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
            .child(Button::new("new-thread").rounded_lg().outline().child(IconName::Plus))
    }

    fn prompt_input(&self, cx: &App) -> impl IntoElement {
        div()
            .flex_shrink_0()
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .p_3()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        // Input area - first row
                        div()
                            .px_3()
                            .py_2()
                            .bg(cx.theme().input)
                            .rounded_md()
                            .border_1()
                            .border_color(cx.theme().border)
                            .child(div().text_color(cx.theme().muted_foreground).child(
                                "Message the Custom Agent — @ to include context, / for commands",
                            )),
                    )
                    .child(
                        // Bottom row - left icons and right buttons
                        div()
                            .flex()
                            .flex_shrink_0()
                            .items_center()
                            .justify_between()
                            .child(
                                // Left side icons
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .child(
                                        Button::new("attach")
                                            .small()
                                            .rounded_md()
                                            .child(IconName::Asterisk),
                                    )
                                    .child(
                                        Button::new("context")
                                            .small()
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
                                        Button::new("model-selector").small().rounded_md().child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_1()
                                                .child("Default")
                                                .child(IconName::ChevronDown),
                                        ),
                                    )
                                    .child(
                                        Button::new("mode-selector").small().rounded_md().child(
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
                                            .small()
                                            .rounded_md()
                                            .child(IconName::ArrowRight),
                                    ),
                            ),
                    ),
            )
    }
}

impl RenderOnce for AgentPanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .size_full()
            .overflow_hidden()
            .child(self.header(cx))
            .child(
                v_flex().id("scrollable-agent-container").flex_1().overflow_y_scroll().p_4().child(
                    div()
                        .size_full()
                        .flex()
                        .justify_center()
                        .items_center()
                        .text_xl()
                        .child("Agent Panel"),
                ),
            )
            .child(self.prompt_input(cx))
    }
}
