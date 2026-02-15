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
    collapsible::Collapsible, h_flex, v_flex, ActiveTheme as _, IconName, StyledExt,
};

// ── Role ─────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

// ── Tool Call Types ──────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ToolCallKind {
    Read,
    Edit,
    Delete,
    Move,
    Search,
    Execute,
    Think,
    Fetch,
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToolCallStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

// ── Plan Types ───────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlanEntryStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Clone)]
pub struct PlanEntry {
    pub content: SharedString,
    pub status: PlanEntryStatus,
}

// ── Message Variant ──────────────────────────────────────────────────

#[derive(Clone)]
pub enum ChatMessageVariant {
    /// User or Agent text message
    Text { role: Role, content: SharedString },
    /// Agent thinking process (collapsible)
    Thought { content: SharedString },
    /// Tool invocation with kind, status, and optional result
    ToolCall {
        title: SharedString,
        kind: ToolCallKind,
        status: ToolCallStatus,
        content: Option<SharedString>,
    },
    /// Execution plan with progress entries
    Plan { entries: Vec<PlanEntry> },
    /// System notification (mode change, config update, etc.)
    System { content: SharedString },
}

// ── ChatMessage Component ────────────────────────────────────────────

#[derive(IntoElement)]
pub struct ChatMessage {
    variant: ChatMessageVariant,
}

impl ChatMessage {
    pub fn text(role: Role, content: impl Into<SharedString>) -> Self {
        Self { variant: ChatMessageVariant::Text { role, content: content.into() } }
    }

    pub fn thought(content: impl Into<SharedString>) -> Self {
        Self { variant: ChatMessageVariant::Thought { content: content.into() } }
    }

    pub fn tool_call(
        title: impl Into<SharedString>,
        kind: ToolCallKind,
        status: ToolCallStatus,
    ) -> Self {
        Self {
            variant: ChatMessageVariant::ToolCall {
                title: title.into(),
                kind,
                status,
                content: None,
            },
        }
    }

    pub fn tool_call_with_content(
        title: impl Into<SharedString>,
        kind: ToolCallKind,
        status: ToolCallStatus,
        content: impl Into<SharedString>,
    ) -> Self {
        Self {
            variant: ChatMessageVariant::ToolCall {
                title: title.into(),
                kind,
                status,
                content: Some(content.into()),
            },
        }
    }

    pub fn plan(entries: Vec<PlanEntry>) -> Self {
        Self { variant: ChatMessageVariant::Plan { entries } }
    }

    pub fn system(content: impl Into<SharedString>) -> Self {
        Self { variant: ChatMessageVariant::System { content: content.into() } }
    }
}

// ── Color Constants (Nord palette) ───────────────────────────────────

const NORD_GREEN: u32 = 0xa3be8c;
const NORD_BLUE: u32 = 0x5e81ac;
const NORD_YELLOW: u32 = 0xebcb8b;
const NORD_RED: u32 = 0xbf616a;
const NORD_FROST: u32 = 0x81a1c1;

// ── Helper Functions ─────────────────────────────────────────────────

fn kind_icon(kind: &ToolCallKind) -> IconName {
    match kind {
        ToolCallKind::Read => IconName::Eye,
        ToolCallKind::Edit => IconName::Replace,
        ToolCallKind::Delete => IconName::Delete,
        ToolCallKind::Move => IconName::ArrowRight,
        ToolCallKind::Search => IconName::Search,
        ToolCallKind::Execute => IconName::SquareTerminal,
        ToolCallKind::Think => IconName::Asterisk,
        ToolCallKind::Fetch => IconName::ArrowDown,
        ToolCallKind::Other => IconName::Ellipsis,
    }
}

fn status_color(status: &ToolCallStatus) -> Hsla {
    match status {
        ToolCallStatus::Pending => rgb(NORD_FROST).into(),
        ToolCallStatus::InProgress => rgb(NORD_YELLOW).into(),
        ToolCallStatus::Completed => rgb(NORD_GREEN).into(),
        ToolCallStatus::Failed => rgb(NORD_RED).into(),
    }
}

fn status_label(status: &ToolCallStatus) -> &'static str {
    match status {
        ToolCallStatus::Pending => "pending",
        ToolCallStatus::InProgress => "running",
        ToolCallStatus::Completed => "completed",
        ToolCallStatus::Failed => "failed",
    }
}

fn plan_entry_icon(status: &PlanEntryStatus) -> IconName {
    match status {
        PlanEntryStatus::Pending => IconName::Loader,
        PlanEntryStatus::InProgress => IconName::LoaderCircle,
        PlanEntryStatus::Completed => IconName::CircleCheck,
    }
}

fn plan_entry_color(status: &PlanEntryStatus) -> Hsla {
    match status {
        PlanEntryStatus::Pending => rgb(NORD_FROST).into(),
        PlanEntryStatus::InProgress => rgb(NORD_BLUE).into(),
        PlanEntryStatus::Completed => rgb(NORD_GREEN).into(),
    }
}

// ── Variant Renderers ────────────────────────────────────────────────

fn render_text(role: Role, content: SharedString, cx: &App) -> Div {
    let (icon, label, accent, bg_color) = match role {
        Role::User => (IconName::CircleUser, "You", rgb(NORD_GREEN), cx.theme().colors.secondary),
        Role::Assistant => {
            (IconName::SquareTerminal, "Agent", rgb(NORD_BLUE), cx.theme().colors.accent)
        }
    };
    let accent_color: Hsla = accent.into();

    v_flex()
        .flex_shrink_0()
        .mb_2()
        .bg(bg_color)
        .border_1()
        .border_color(cx.theme().colors.border)
        .rounded_md()
        .overflow_hidden()
        // Header
        .child(
            h_flex()
                .items_center()
                .gap_1p5()
                .px_2()
                .py_1()
                .child(div().text_sm().text_color(accent_color).child(icon))
                .child(div().text_xs().font_semibold().text_color(accent_color).child(label)),
        )
        // Content
        .child(
            div()
                .px_2()
                .py_1()
                .text_sm()
                .text_color(cx.theme().colors.foreground)
                .min_w_0()
                .overflow_hidden()
                .child(content),
        )
}

fn render_thought(content: SharedString, cx: &App) -> Div {
    let accent: Hsla = rgb(NORD_YELLOW).into();

    v_flex()
        .flex_shrink_0()
        .mb_2()
        .bg(cx.theme().colors.muted)
        .border_1()
        .border_color(cx.theme().colors.border)
        .rounded_md()
        .overflow_hidden()
        // Header
        .child(
            h_flex()
                .items_center()
                .gap_1p5()
                .px_2()
                .py_1()
                .child(div().text_sm().text_color(accent).child(IconName::Asterisk))
                .child(div().text_xs().font_semibold().text_color(accent).child("Thinking")),
        )
        // Collapsible content
        .child(Collapsible::new().open(false).content(
            div().px_2().py_1().text_xs().text_color(cx.theme().foreground).child(content),
        ))
}

fn render_tool_call(
    title: SharedString,
    kind: ToolCallKind,
    status: ToolCallStatus,
    content: Option<SharedString>,
    cx: &App,
) -> Div {
    let icon = kind_icon(&kind);
    let s_color = status_color(&status);
    let s_label = status_label(&status);

    let mut container = v_flex()
        .flex_shrink_0()
        .mb_2()
        .bg(cx.theme().tab_bar)
        .border_1()
        .border_color(cx.theme().border)
        .rounded_md()
        .overflow_hidden()
        // Header: kind icon + title + status
        .child(
            h_flex()
                .items_center()
                .justify_between()
                .px_2()
                .py_1p5()
                .child(
                    h_flex()
                        .items_center()
                        .gap_1p5()
                        .child(div().text_sm().text_color(s_color).child(icon))
                        .child(
                            div()
                                .text_sm()
                                .font_semibold()
                                .text_color(cx.theme().foreground)
                                .child(title),
                        ),
                )
                .child(
                    h_flex()
                        .items_center()
                        .gap_1()
                        .child(div().w(px(6.)).h(px(6.)).rounded(px(3.)).bg(s_color))
                        .child(div().text_xs().text_color(s_color).child(s_label)),
                ),
        );

    // Optional collapsible content area
    if let Some(result_content) = content {
        container = container.child(
            Collapsible::new().open(false).content(
                div()
                    .mx_2()
                    .mb_1p5()
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
                            .child(result_content),
                    ),
            ),
        );
    }

    container
}

fn render_plan(entries: Vec<PlanEntry>, cx: &App) -> Div {
    let accent: Hsla = rgb(NORD_BLUE).into();

    let mut container = v_flex()
        .flex_shrink_0()
        .mb_2()
        .bg(cx.theme().tab_bar)
        .border_1()
        .border_color(cx.theme().border)
        .rounded_md()
        .overflow_hidden()
        // Header
        .child(
            h_flex()
                .items_center()
                .gap_1p5()
                .px_2()
                .py_1()
                .child(div().text_sm().text_color(accent).child(IconName::Menu))
                .child(div().text_xs().font_semibold().text_color(accent).child("Plan")),
        );

    // Entry list
    for entry in &entries {
        let icon = plan_entry_icon(&entry.status);
        let color = plan_entry_color(&entry.status);
        let status_text = match entry.status {
            PlanEntryStatus::Pending => "pending",
            PlanEntryStatus::InProgress => "in progress",
            PlanEntryStatus::Completed => "completed",
        };

        container = container.child(
            h_flex()
                .items_center()
                .justify_between()
                .px_2()
                .py_1()
                .child(
                    h_flex()
                        .items_center()
                        .gap_1p5()
                        .child(div().text_xs().text_color(color).child(icon))
                        .child(
                            div()
                                .text_sm()
                                .text_color(cx.theme().foreground)
                                .child(entry.content.clone()),
                        ),
                )
                .child(div().text_xs().text_color(color).child(status_text)),
        );
    }

    container
}

fn render_system(content: SharedString, cx: &App) -> Div {
    let accent: Hsla = rgb(NORD_YELLOW).into();

    v_flex()
        .flex_shrink_0()
        .mb_2()
        .bg(cx.theme().colors.muted)
        .border_1()
        .border_color(cx.theme().colors.border)
        .rounded_md()
        .overflow_hidden()
        // Header
        .child(
            h_flex()
                .items_center()
                .gap_1p5()
                .px_2()
                .py_1()
                .child(div().text_sm().text_color(accent).child(IconName::Info))
                .child(div().text_xs().font_semibold().text_color(accent).child("System")),
        )
        // Content
        .child(div().px_2().py_1().text_xs().text_color(cx.theme().foreground).child(content))
}

// ── RenderOnce Implementation ────────────────────────────────────────

impl RenderOnce for ChatMessage {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.variant {
            ChatMessageVariant::Text { role, content } => {
                render_text(role, content, cx).into_any_element()
            }
            ChatMessageVariant::Thought { content } => {
                render_thought(content, cx).into_any_element()
            }
            ChatMessageVariant::ToolCall { title, kind, status, content } => {
                render_tool_call(title, kind, status, content, cx).into_any_element()
            }
            ChatMessageVariant::Plan { entries } => render_plan(entries, cx).into_any_element(),
            ChatMessageVariant::System { content } => render_system(content, cx).into_any_element(),
        }
    }
}
