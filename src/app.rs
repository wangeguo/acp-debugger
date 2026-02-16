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
    resizable::{h_resizable, resizable_panel},
    v_flex, IconName, Root, Sizable as _, TitleBar, WindowExt as _,
};

use crate::{
    components::AgentForm,
    models::AgentRegistry,
    panels::{AgentPanel, MessagePanel},
};

pub struct AcpDebugger {
    agent_panel: Entity<AgentPanel>,
    agent_registry: Entity<AgentRegistry>,
}

impl AcpDebugger {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let agent_panel = cx.new(|cx| AgentPanel::new(window, cx));
        let agent_registry = cx.new(|_| AgentRegistry::new());
        Self { agent_panel, agent_registry }
    }
}

impl Render for AcpDebugger {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);
        let registry = self.agent_registry.clone();

        v_flex()
            .size_full()
            .child(TitleBar::new().text_xs().child("ACP Debugger").child(
                div().flex().items_center().gap_1().mr(px(9.0)).child("Custom Agent").child(
                    Button::new("add-agent").xsmall().rounded_lg().icon(IconName::Plus).on_click(
                        move |_, window, cx| {
                            let reg = registry.clone();
                            let form = cx.new(|cx| AgentForm::new(reg, window, cx));
                            window.open_sheet(cx, move |sheet, _, _| {
                                sheet.size(px(450.)).title("New Agent").child(form.clone())
                            });
                        },
                    ),
                ),
            ))
            .child(
                div().flex_1().w_full().overflow_hidden().child(
                    h_resizable("layout")
                        .child(resizable_panel().size(px(400.)).child(self.agent_panel.clone()))
                        .child(resizable_panel().child(MessagePanel)),
                ),
            )
            .children(sheet_layer)
    }
}
