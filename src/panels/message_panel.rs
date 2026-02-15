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
use gpui_component::{button::Button, v_flex, ActiveTheme as _, IconName, WindowExt};

use crate::{components::MessageItem, models::AcpMessage, panels::DetailPanel};

#[derive(IntoElement)]
pub struct MessagePanel;

impl MessagePanel {
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
                    .gap_4()
                    .child(
                        Button::new("record")
                            .compact()
                            .rounded_lg()
                            .outline()
                            .icon(IconName::CircleCheck)
                            .label("Record"),
                    )
                    .child(
                        Button::new("clean")
                            .compact()
                            .rounded_lg()
                            .outline()
                            .icon(IconName::CircleX)
                            .label("Clean"),
                    )
                    .child(
                        Button::new("copy")
                            .compact()
                            .rounded_lg()
                            .outline()
                            .icon(IconName::Copy)
                            .label("Copy"),
                    ),
            )
            .child(Button::new("new-message").rounded_lg().outline().icon(IconName::Plus))
    }

    fn message(
        title: impl Into<SharedString>,
        json_content: impl Into<SharedString>,
        is_response: bool,
    ) -> impl IntoElement {
        let title: SharedString = title.into();
        let json_content: SharedString = json_content.into();
        let acp_message = AcpMessage::parse(title.clone(), json_content.clone(), is_response);

        div()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                let msg = acp_message.clone();
                let msg_title = msg.title.clone();
                window.open_sheet(cx, move |sheet, _, _| {
                    sheet
                        .size_full()
                        .margin_top(px(84.)) // Space for window title bar
                        .size(px(400.))
                        .title(format!("{} - Message Details", msg_title))
                        .child(DetailPanel::new(msg.clone()))
                })
            })
            .child(MessageItem::new(title, json_content, is_response))
    }
}

impl RenderOnce for MessagePanel {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex().size_full().overflow_hidden().child(self.header(cx)).child(
            div()
                .id("scrollable-messages-container")
                .flex_1()
                .overflow_y_scroll()
                .px_3()
                .py_2()
                // initialize request
                .child(Self::message(
                    "initialize",
                    r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":1,"clientCapabilities":{"fs":{"readTextFile":true,"writeTextFile":true},"terminal":true},"clientInfo":{"name":"my-client","title":"My Client","version":"1.0.0"}}}"#,
                    false,
                ))
                // initialize response
                .child(Self::message(
                    "initialize",
                    r#"{"jsonrpc":"2.0","id":0,"result":{"protocolVersion":1,"agentCapabilities":{"loadSession":true,"promptCapabilities":{"image":true,"audio":true},"mcp":{"http":true,"sse":true}},"agentInfo":{"name":"my-agent","title":"My Agent","version":"1.0.0"},"authMethods":[]}}"#,
                    true,
                ))
                // authenticate request
                .child(Self::message(
                    "authenticate",
                    r#"{"jsonrpc":"2.0","id":1,"method":"authenticate","params":{"method":"api_key","credentials":{"key":"sk-..."}}}"#,
                    false,
                ))
                // authenticate response
                .child(Self::message(
                    "authenticate",
                    r#"{"jsonrpc":"2.0","id":1,"result":{"success":true}}"#,
                    true,
                ))
                // session/new request
                .child(Self::message(
                    "session/new",
                    r#"{"jsonrpc":"2.0","id":2,"method":"session/new","params":{"cwd":"/home/user/project","mcpServers":[{"name":"filesystem","command":"/path/to/mcp-server","args":["--stdio"],"env":[]}]}}"#,
                    false,
                ))
                // session/new response
                .child(Self::message(
                    "session/new",
                    r#"{"jsonrpc":"2.0","id":2,"result":{"sessionId":"sess_abc123def456"}}"#,
                    true,
                ))
                // session/prompt request
                .child(Self::message(
                    "session/prompt",
                    r#"{"jsonrpc":"2.0","id":3,"method":"session/prompt","params":{"sessionId":"sess_abc123def456","prompt":[{"type":"text","text":"Can you analyze this code?"}]}}"#,
                    false,
                ))
                // session/update notification (agent_message_chunk)
                .child(Self::message(
                    "session/update",
                    r#"{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"sess_abc123def456","update":{"sessionUpdate":"agent_message_chunk","delta":"I'll analyze the code..."}}}"#,
                    true,
                ))
                // session/update notification (tool_call)
                .child(Self::message(
                    "session/update",
                    r#"{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"sess_abc123def456","update":{"sessionUpdate":"tool_call","toolCallId":"call_001","title":"Reading file","kind":"read","status":"pending"}}}"#,
                    true,
                ))
                // session/update notification (tool_call_update)
                .child(Self::message(
                    "session/update",
                    r#"{"jsonrpc":"2.0","method":"session/update","params":{"sessionId":"sess_abc123def456","update":{"sessionUpdate":"tool_call_update","toolCallId":"call_001","status":"completed","content":[{"type":"text","text":"File content here..."}]}}}"#,
                    true,
                ))
                // fs/read_text_file request (agent to client)
                .child(Self::message(
                    "fs/read_text_file",
                    r#"{"jsonrpc":"2.0","id":4,"method":"fs/read_text_file","params":{"path":"/home/user/project/main.py"}}"#,
                    false,
                ))
                // fs/read_text_file response
                .child(Self::message(
                    "fs/read_text_file",
                    r#"{"jsonrpc":"2.0","id":4,"result":{"content":"def main():\n    print('Hello')\n"}}"#,
                    true,
                ))
                // fs/write_text_file request
                .child(Self::message(
                    "fs/write_text_file",
                    r#"{"jsonrpc":"2.0","id":5,"method":"fs/write_text_file","params":{"path":"/home/user/project/output.txt","content":"Result data"}}"#,
                    false,
                ))
                // fs/write_text_file response
                .child(Self::message(
                    "fs/write_text_file",
                    r#"{"jsonrpc":"2.0","id":5,"result":{"success":true}}"#,
                    true,
                ))
                // terminal/create request
                .child(Self::message(
                    "terminal/create",
                    r#"{"jsonrpc":"2.0","id":6,"method":"terminal/create","params":{"cwd":"/home/user/project"}}"#,
                    false,
                ))
                // terminal/create response
                .child(Self::message(
                    "terminal/create",
                    r#"{"jsonrpc":"2.0","id":6,"result":{"terminalId":"term_xyz789"}}"#,
                    true,
                ))
                // session/cancel notification
                .child(Self::message(
                    "session/cancel",
                    r#"{"jsonrpc":"2.0","method":"session/cancel","params":{"sessionId":"sess_abc123def456"}}"#,
                    false,
                )),
        )
    }
}
