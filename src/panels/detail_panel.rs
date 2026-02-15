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
use gpui_component::{
    accordion::Accordion, description_list::DescriptionList, h_flex, tag::Tag, text::TextView,
    v_flex, ActiveTheme as _, Sizable as _, StyledExt as _,
};

use crate::models::{AcpMessage, MessageType};

/// A panel that displays detailed information for a single ACP message.
#[derive(IntoElement)]
pub struct DetailPanel {
    message: AcpMessage,
}

impl DetailPanel {
    pub fn new(message: AcpMessage) -> Self {
        Self { message }
    }

    fn message_type_tag(message_type: MessageType) -> Tag {
        match message_type {
            MessageType::Request => Tag::info().child("Request"),
            MessageType::Response => Tag::success().child("Response"),
            MessageType::Notification => Tag::warning().child("Notification"),
            MessageType::Error => Tag::danger().child("Error"),
        }
    }

    fn direction_tag(is_response: bool) -> Tag {
        if is_response {
            Tag::secondary().outline().child("\u{2191} Incoming")
        } else {
            Tag::secondary().outline().child("\u{2193} Outgoing")
        }
    }
}

impl RenderOnce for DetailPanel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let msg = &self.message;

        // Build overview description list
        let mut overview = DescriptionList::new().columns(1).small();

        if let Some(method) = &msg.method {
            overview = overview.item("Method", method.to_string(), 1);
        }

        if let Some(id) = &msg.id {
            overview = overview.item("ID", id.to_string(), 1);
        }

        overview = overview.item("Type", msg.message_type.label().to_string(), 1).item(
            "Direction",
            msg.direction_label().to_string(),
            1,
        );

        if let Some(version) = &msg.jsonrpc_version {
            overview = overview.item("Protocol", version.to_string(), 1);
        }

        // Build payload TextView (params or result)
        let payload_section = msg.payload_json().map(|payload| {
            let section_title: &str = if msg.params.is_some() { "Params" } else { "Result" };
            let md = format!("```json\n{}\n```", payload);
            let text_view = TextView::markdown("payload-json", md, window, cx);
            (section_title, text_view)
        });

        // Build error section
        let error_section = msg.error.as_ref().map(|error| {
            let mut md = format!("**Code:** `{}`\n\n**Message:** {}\n", error.code, error.message);
            if let Some(data) = &error.data {
                if let Ok(pretty) = serde_json::to_string_pretty(data) {
                    md.push_str(&format!("\n```json\n{}\n```", pretty));
                }
            }
            TextView::markdown("error-details", md, window, cx)
        });

        // Build raw payload TextView
        let raw_json = msg.pretty_json();
        let raw_md = format!("```json\n{}\n```", raw_json);
        let raw_text_view = TextView::markdown("raw-json", raw_md, window, cx);

        // Build accordion
        let mut accordion = Accordion::new("detail-sections")
            .multiple(true)
            .bordered(true)
            .small()
            // Section 1: Overview (always present, open)
            .item(|item| item.open(true).title("Overview").child(overview));

        // Section 2: Params/Result (conditional)
        if let Some((title, text_view)) = payload_section {
            accordion = accordion.item(|item| item.open(true).title(title).child(text_view));
        }

        // Section 3: Error Details (conditional)
        if let Some(text_view) = error_section {
            accordion =
                accordion.item(|item| item.open(true).title("Error Details").child(text_view));
        }

        // Section 4: Raw Payload (always present, collapsed)
        accordion =
            accordion.item(|item| item.open(false).title("Raw Payload").child(raw_text_view));

        v_flex()
            .id("detail-panel")
            .size_full()
            .overflow_y_scroll()
            .gap_3()
            .p_3()
            // Header: type tag + direction tag + title
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Self::message_type_tag(msg.message_type))
                    .child(Self::direction_tag(msg.is_response))
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .text_color(cx.theme().foreground)
                            .child(msg.title.clone()),
                    ),
            )
            // Accordion sections
            .child(accordion)
    }
}
