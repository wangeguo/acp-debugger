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
use gpui_component::Root;

struct AcpDebugger {
    text: SharedString,
}

impl AcpDebugger {
    fn new() -> Self {
        Self { text: "ACP Debugger".into() }
    }
}

impl Render for AcpDebugger {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .bg(rgb(0x2e3440))
            .size_full()
            .justify_center()
            .items_center()
            .text_xl()
            .text_color(rgb(0xeceff4))
            .child(self.text.clone())
    }
}

fn main() {
    let app = Application::new();

    app.run(|ctx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(ctx);

        ctx.spawn(async move |ctx| {
            ctx.open_window(WindowOptions::default(), |window, ctx| {
                let view = ctx.new(|_| AcpDebugger::new());
                // This first level on the window, should be a Root.
                ctx.new(|ctx| Root::new(view, window, ctx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();

        ctx.activate(true);
    });
}
