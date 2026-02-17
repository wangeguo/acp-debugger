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
    menu::{DropdownMenu as _, PopupMenuItem},
    IconName, Sizable as _, WindowExt as _,
};

use crate::{components::AgentForm, models::AgentRegistry};

pub struct AgentSwitcher {
    registry: Entity<AgentRegistry>,
}

impl AgentSwitcher {
    pub fn new(registry: Entity<AgentRegistry>, cx: &mut Context<Self>) -> Self {
        cx.observe(&registry, |_, _, cx| cx.notify()).detach();
        Self { registry }
    }
}

impl Render for AgentSwitcher {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let registry = self.registry.clone();
        let active_name: SharedString = self
            .registry
            .read(cx)
            .active_agent()
            .map(|a| SharedString::from(a.name.clone()))
            .unwrap_or_else(|| "Select Agent".into());

        Button::new("agent-switcher")
            .xsmall()
            .rounded_lg()
            .child(
                div().flex().items_center().gap_1().child(active_name).child(IconName::ChevronDown),
            )
            .dropdown_menu({
                let registry = registry.clone();
                move |menu, _window, cx| {
                    let agents = registry.read(cx).agents().to_vec();
                    let active_id = registry.read(cx).active_agent_id().cloned();

                    let mut menu = menu;

                    if agents.is_empty() {
                        menu = menu.item(PopupMenuItem::new("No agents configured").disabled(true));
                    } else {
                        for agent in &agents {
                            let is_active = active_id.as_ref() == Some(&agent.id);
                            let agent_id = agent.id.clone();
                            let reg = registry.clone();
                            menu = menu.item(
                                PopupMenuItem::new(agent.name.clone()).checked(is_active).on_click(
                                    move |_, _, cx| {
                                        reg.update(cx, |r, _| {
                                            r.set_active(Some(agent_id.clone()));
                                        });
                                    },
                                ),
                            );
                        }
                    }

                    let reg_for_form = registry.clone();
                    menu.separator().item(
                        PopupMenuItem::new("Add Agent...").icon(IconName::Plus).on_click(
                            move |_, window, cx| {
                                let reg = reg_for_form.clone();
                                let form = cx.new(|cx| AgentForm::new(reg, window, cx));
                                window.open_sheet(cx, move |sheet, _, _| {
                                    sheet.size(px(450.)).title("New Agent").child(form.clone())
                                });
                            },
                        ),
                    )
                }
            })
    }
}
