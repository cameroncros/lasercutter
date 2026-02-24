use std::time::Duration;

use base64::Engine;
use dioxus::html::geometry::WheelDelta;
// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::prelude::*;
use laser_cutter::{
    gcode_emulator::GCodeEmulator,
    gcode_generator::{cut::Cut, workspace::Workspace},
};

mod components;

use crate::components::{
    connection_controls::ConnectionControls,
    cutlist::CutList,
    machine_controls::MachineControls,
};

// We can import assets in dioxus with the `asset!` macro. This macro takes a path to an asset relative to the crate root.
// The macro returns an `Asset` type that will display as the path to the asset in the browser or a local path in desktop bundles.
const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    // The `launch` function is the main entry point for a dioxus app. It takes a component and renders it with the platform feature
    // you have enabled
    dioxus::launch(App);
}

fn render(workspace: &Workspace) -> anyhow::Result<(String, Duration)> {
    let start = std::time::Instant::now();
    let gcode = workspace.gen_gcode()?;
    let mut emu = GCodeEmulator::from_gcode(gcode)?;
    emu.run()?;
    let svg = emu.to_svg_str()?;
    let svg_b64 = base64::engine::general_purpose::STANDARD.encode(svg.as_bytes());
    Ok((
        format!("data:image/svg+xml;base64,{}", svg_b64),
        start.elapsed(),
    ))
}

fn new(workspace: &mut Signal<Workspace>) -> anyhow::Result<()> {
    workspace.set(Workspace::init(100.0, 100.0));
    Ok(())
}

fn save(workspace: &mut Signal<Workspace>) -> anyhow::Result<()> {
    let workspace = workspace.read();
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("YAML", &["yaml", "yml"])
        .save_file()
    {
        workspace.save(path)?
    }
    Ok(())
}

/// App is the main component of our app. Components are the building blocks of dioxus apps. Each component is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    let mut workspace = use_signal(|| Workspace::init(100.0, 100.0));
    let mut errormsg = use_signal(String::new);
    let mut zoom = use_signal(|| 1.0f32);

    let (preview, rendertime) = match render(&workspace.read()) {
        Ok((svg, duration)) => (svg, format!("{duration:?}")),
        Err(e) => {
            errormsg.set(e.to_string());
            (String::new(), "Failed".to_string())
        }
    };
    let status_msg = if !errormsg().is_empty() {
        errormsg()
    } else {
        String::new()
    };

    // The `rsx!` macro lets us define HTML inside of rust. It expands to an Element with all of our HTML inside.
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "h-screen flex flex-col",
            div { class: "flex items-center justify-between bg-gray-800 text-white px-4 py-3",
                div { class: "flex items-baseline gap-2",
                    span { class: "text-2xl font-semibold", "Laser Cutter" }
                    span { class: "text-gray-300", "a simple laser cutter" }
                }
                div { class: "flex gap-2",
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            if let Err(e) = new(&mut workspace) {
                                errormsg.set(e.to_string())
                            }
                        },
                        "New"
                    }
                    label { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        "Load"
                        input {
                            r#type: "file",
                            accept: "*.yaml,*.yml",
                            onchange: move |evt: Event<FormData>| {
                                if let [file] = &evt.files()[..] {
                                    match Workspace::load(file.path()) {
                                        Ok(ws) => {
                                            workspace.set(ws);
                                        }
                                        Err(e) => {
                                            errormsg.set(e.to_string());
                                        }
                                    }
                                }
                            },
                            hidden: true,
                        }
                    }
                    button {
                        class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        onclick: move |_| {
                            if let Err(e) = save(&mut workspace) {
                                errormsg.set(e.to_string())
                            }
                        },
                        "Save"
                    }
                }
            }

            div { class: "flex flex-1 overflow-hidden",
                div { class: "w-72 bg-gray-500 p-3 flex flex-col gap-2 overflow-auto",
                    CutList { workspace }
                    label { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                        "Add Cut"
                        input {
                            r#type: "file",
                            accept: "*.svg",
                            multiple: true,
                            hidden: true,
                            onchange: move |evt: Event<FormData>| {
                                let mut ws = workspace.write();

                                for file in evt.files() {
                                    match Cut::from_svg(file.path()) {
                                        Ok(cut) => ws.add_cut(cut),
                                        Err(e) => {
                                            errormsg.set(format!("Failed to load: {:?} - {}", file.path(), e));
                                            return;
                                        }
                                    }
                                }
                            },
                        }
                    }
                }

                div { class: "flex-1 bg-gray-300 flex flex-col overflow-hidden",
                    div { class: "flex items-center justify-between px-3 py-2 bg-gray-200 border-b border-gray-400",
                        span { class: "text-sm text-gray-700", "Preview" }
                        div { class: "flex items-center gap-2",
                            button {
                                class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                                onclick: move |_| zoom.set((zoom() - 0.1).max(0.2)),
                                "-"
                            }
                            span { class: "text-xs text-gray-700 w-12 text-center",
                                "{(zoom() * 100.0).round()}%"
                            }
                            button {
                                class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                                onclick: move |_| zoom.set((zoom() + 0.1).min(5.0)),
                                "+"
                            }
                            button {
                                class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded",
                                onclick: move |_| zoom.set(1.0),
                                "Reset"
                            }
                        }
                    }
                    div {
                        class: "flex-1 overflow-auto",
                        onwheel: move |e| {
                            let delta = e.delta();
                            match delta {
                                WheelDelta::Pixels(delta) => {
                                    if delta.y < 0.0 {
                                        zoom.set((zoom() + 0.1).min(5.0));
                                    } else if delta.y > 0.0 {
                                        zoom.set((zoom() - 0.1).max(0.2));
                                    }
                                }
                                WheelDelta::Lines(delta) => {
                                    if delta.y < 0.0 {
                                        zoom.set((zoom() + 0.1).min(5.0));
                                    } else if delta.y > 0.0 {
                                        zoom.set((zoom() - 0.1).max(0.2));
                                    }
                                }
                                WheelDelta::Pages(delta) => {
                                    if delta.y < 0.0 {
                                        zoom.set((zoom() + 0.1).min(5.0));
                                    } else if delta.y > 0.0 {
                                        zoom.set((zoom() - 0.1).max(0.2));
                                    }
                                }
                            }
                        },
                        div { class: "min-h-full min-w-full flex items-center justify-center p-6",
                            img {
                                class: "max-w-none max-h-none",
                                style: "transform: scale({zoom}); transform-origin: 0 0;",
                                src: preview,
                            }
                        }
                    }
                }

                div { class: "w-64 bg-gray-600 p-3 text-white",
                    ConnectionControls {}
                    MachineControls {}
                }
            }

            div { class: "bg-gray-900 text-gray-100 px-4 py-2 text-sm flex items-center justify-between",
                span { "Status: {status_msg}" }
                span { class: "text-gray-400", "{rendertime}" }
            }
        }
    }
}
