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
use gpui_component::StyledExt;

#[derive(IntoElement)]
pub struct MessageItem {
    title: SharedString,
    is_response: bool,
}

impl MessageItem {
    pub fn new(title: impl Into<SharedString>, is_response: bool) -> Self {
        Self { title: title.into(), is_response }
    }

    fn get_bg_color(&self) -> Hsla {
        if self.is_response {
            rgb(0xa3be8c).into() // Green for response
        } else {
            rgb(0xd8dee9).into() // Blue for request
        }
    }
}

impl RenderOnce for MessageItem {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        div().flex().flex_col().bg(self.get_bg_color()).rounded_md().p_3().m_2().child(
            div().text_sm().font_semibold().text_color(rgb(0x2e3440)).child(self.title.clone()),
        )
    }
}
