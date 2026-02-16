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
use gpui_component::{Root, TitleBar};
use gpui_component_assets::Assets;

mod app;
#[allow(dead_code)]
mod client;
mod components;
mod models;
mod panels;

fn main() {
    let app = Application::new().with_assets(Assets);

    app.run(|cx| {
        // This must be called before using any GPUI Component features.
        gpui_component::init(cx);
        panels::AgentPanel::init(cx);

        cx.spawn(async move |cx| {
            let window_options = WindowOptions {
                // Setup GPUI to use custom title bar
                titlebar: Some(TitleBar::title_bar_options()),
                ..Default::default()
            };
            cx.open_window(window_options, |window, cx| {
                let view = cx.new(|cx| app::AcpDebugger::new(window, cx));
                // This first level on the window, should be a Root.
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();

        cx.activate(true);
    });
}
