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
    resizable::{h_resizable, resizable_panel},
    v_flex, Root, TitleBar,
};

use crate::{
    components::AgentSwitcher,
    models::AgentRegistry,
    panels::{AgentPanel, MessagePanel},
};

pub struct AcpDebugger {
    agent_panel: Entity<AgentPanel>,
    agent_switcher: Entity<AgentSwitcher>,
}

impl AcpDebugger {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let agent_registry = cx.new(|_| AgentRegistry::new());
        let agent_panel = cx.new(|cx| AgentPanel::new(window, cx));
        let agent_switcher = cx.new(|cx| AgentSwitcher::new(agent_registry, cx));
        Self { agent_panel, agent_switcher }
    }
}

impl Render for AcpDebugger {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sheet_layer = Root::render_sheet_layer(window, cx);

        v_flex()
            .size_full()
            .child(
                TitleBar::new().text_xs().child("ACP Debugger").child(
                    div().flex().items_center().mr(px(9.0)).child(self.agent_switcher.clone()),
                ),
            )
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
