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

use gpui::{prelude::*, *};
use gpui_component::{
    button::Button,
    h_flex,
    input::{Input, InputState},
    v_flex, ActiveTheme as _, IconName, Sizable as _,
};

pub struct AgentPanel {
    input: Entity<InputState>,
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

        Self { input }
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

    fn prompt_input(&self, cx: &Context<Self>) -> impl IntoElement {
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
                                    .child(IconName::ArrowRight),
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
