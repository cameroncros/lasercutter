use crate::style::{
    MENU_BUTTON_CLASSES, TOP_BAR_ACTIONS_CONTAINER_CLASSES, TOP_BAR_CLASSES,
    TOP_BAR_SUBTITLE_CLASSES, TOP_BAR_TITLE_CLASSES, TOP_BAR_TITLE_CONTAINER_CLASSES,
};
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
pub fn TopBar(workspace: Signal<Workspace>) -> Element {
    rsx! {
        div { class: TOP_BAR_CLASSES,
            div { class: TOP_BAR_TITLE_CONTAINER_CLASSES,
                span { class: TOP_BAR_TITLE_CLASSES, "Laser Cutter" }
                span { class: TOP_BAR_SUBTITLE_CLASSES, "a simple laser cutter" }
            }
            div { class: TOP_BAR_ACTIONS_CONTAINER_CLASSES,
                button {
                    class: MENU_BUTTON_CLASSES,
                    onclick: move |_| {
                        if let Err(e) = new(&mut workspace) {
                            error!("Failed to create new workspace: {e}");
                        }
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
                            error!("Failed to create load workspace: {e}");
                                    }
                                }
                            }
                        },
                        hidden: true,
                    }
                }
                button {
                    class: MENU_BUTTON_CLASSES,
                    onclick: move |_| {
                        if let Err(e) = save(&mut workspace) {
                            error!("Failed to create save workspace: {e}");
                        }
                    },
                    "Save"
                }
            }
        }
    }
}
