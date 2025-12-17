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
use gpui_component::v_flex;

#[derive(IntoElement)]
pub struct DetailPanel;

impl RenderOnce for DetailPanel {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        v_flex().gap_2().child(ModelSection).child(ModeSection)
    }
}

#[derive(IntoElement)]
struct ModelSection;

impl RenderOnce for ModelSection {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .child("Models")
            .child("Current Model: Sonnet")
            .child("Available Models:")
            .child(
                div()
                    .text_sm()
                    .ml_2()
                    .child("• Sonnet - Best of everyday tasks")
                    .child("• Opus - Most capable for complex work")
                    .child("• Haiku - Fastest for quick answers"),
            )
    }
}

#[derive(IntoElement)]
struct ModeSection;

impl RenderOnce for ModeSection {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .child("Modes")
            .child("Current Mode: Default")
            .child("Available Modes:")
            .child(
                div()
                    .text_sm()
                    .ml_2()
                    .child("• Default - Standard behavior")
                    .child("• Accept Edits - Auto-accept file edits")
                    .child("• Plan - Planning mode, no actual tool execution")
                    .child("• Don't Ask - Don't prompt for confirmations")
                    .child("• Bypass Permissions - Bypass all permission checks"),
            )
    }
}
