use crate::style::MENU_BUTTON_CLASSES;
use dioxus::prelude::*;
use laser_cutter::gcode_generator::workspace::Workspace;

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

#[component]
pub fn TopBar(
    workspace: Signal<Workspace>,
    errormsg: Signal<String>,
    refresh: Signal<i32>,
) -> Element {
    rsx! {
        div { class: "flex items-center justify-between bg-gray-800 text-white px-4 py-3",
            div { class: "flex items-baseline gap-2",
                span { class: "text-2xl font-semibold", "Laser Cutter" }
                span { class: "text-gray-300", "a simple laser cutter" }
            }
            div { class: "flex gap-2",
                button {
                    class: MENU_BUTTON_CLASSES,
                    onclick: move |_| {
                        if let Err(e) = new(&mut workspace) {
                            errormsg.set(e.to_string())
                        }
                        refresh += 1;
                    },
                    "New"
                }
                label { class: MENU_BUTTON_CLASSES,
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
                            refresh += 1;
                        },
                        hidden: true,
                    }
                }
                button {
                    class: MENU_BUTTON_CLASSES,
                    onclick: move |_| {
                        if let Err(e) = save(&mut workspace) {
                            errormsg.set(e.to_string())
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
