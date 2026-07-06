use dioxus::prelude::*;
use laser_cutter::gcode_generator::workspace::Workspace;

mod components;
mod style;
mod tracing_logger;

use crate::components::leftbar::LeftBar;
use crate::components::log_window::LogWindow;
use crate::components::rightbar::RightBar;
use crate::components::statusbar::StatusBar;
use crate::components::topbar::TopBar;
use crate::components::workspace::WorkspaceView;
use crate::tracing_logger::{init_tracing, LOG_RX};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    init_tracing();
    launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let workspace = use_signal(|| Workspace::init(100.0, 100.0));
    let mut msglog = use_signal(Vec::new);
    let rendertime = use_signal(String::new);
    let mut show_log = use_signal(|| true);

    use_coroutine(move |_: UnboundedReceiver<String>| async move {
        loop {
            let recv = LOG_RX.lock().unwrap().take();
            if let Some(mut rx) = recv {
                loop {
                    if let Some(msg) = rx.recv().await {
                        msglog.push(msg);
                    }
                }
            } else {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
        }
    });

    error!("Hello World");

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "h-screen flex flex-col overflow-hidden",
            TopBar { workspace }

            div { class: "flex-1 flex flex-row overflow-hidden",
                LeftBar { workspace },
                WorkspaceView {
                    workspace,
                    rendertime,
                },
                RightBar {}
            }

            StatusBar {
                msglog,
                rendertime,
                show_log,
                on_toggle_log: move |_| show_log.toggle()
            }
        }
        if show_log() {
            LogWindow {
                msglog,
            }
        }
    }
}
