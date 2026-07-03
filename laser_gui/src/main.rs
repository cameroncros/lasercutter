// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use laser_cutter::gcode_generator::workspace::Workspace;

mod components;
mod style;

use crate::components::leftbar::LeftBar;
use crate::components::rightbar::RightBar;
use crate::components::statusbar::StatusBar;
use crate::components::topbar::TopBar;
use crate::components::workspace::WorkspaceView;

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    launch(App);
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let workspace = use_signal(|| Workspace::init(100.0, 100.0));
    let errormsg = use_signal(String::new);
    let rendertime = use_signal(String::new);
    let refresh = use_signal(|| 0);

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "h-screen flex flex-col",
            TopBar { workspace, errormsg, refresh }

            div { class: "v-screen flex flex-row",
                LeftBar { workspace, errormsg, refresh },
                WorkspaceView {
                    workspace,
                    rendertime,
                    errormsg,
                    refresh,
                },
                RightBar {}
            }

            StatusBar { errormsg, rendertime }
        }
    }
}
