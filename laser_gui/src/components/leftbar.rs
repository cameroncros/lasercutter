use crate::components::operationslist::CutList;
use crate::style::MENU_BUTTON_CLASSES;
use dioxus::prelude::*;
use laser_cutter::gcode_generator::operation::cut::Cut;
use laser_cutter::gcode_generator::operation::raster::Raster;
use laser_cutter::gcode_generator::workspace::Workspace;

#[component]
pub fn LeftBar(workspace: Signal<Workspace>, errormsg: Signal<String>) -> Element {
    rsx! {
        div { class: "bg-gray-500 p-3 flex flex-col gap-2 overflow-auto",
            CutList { workspace }
            label { class: MENU_BUTTON_CLASSES,
                "Add Cut"
                input {
                    r#type: "file",
                    accept: "*.svg",
                    multiple: true,
                    hidden: true,
                    onchange: move |evt: Event<FormData>| {
                        let mut ws = workspace.write();

                        for file in evt.files() {
                            if file.path().extension().unwrap() == "svg" {
                                match Cut::from_svg(file.path()) {
                                    Ok(cut) => ws.add_operation(cut),
                                    Err(e) => {
                                        errormsg
                                            .set(format!("Failed to load: {:?} - {}", file.path(), e));
                                        return;
                                    }
                                }
                            } else {
                                match Raster::from_image(file.path()) {
                                    Ok(cut) => ws.add_operation(cut),
                                    Err(e) => {
                                        errormsg
                                            .set(format!("Failed to load: {:?} - {}", file.path(), e));
                                        return;
                                    }
                                }
                            }
                        }
                    },
                }
            }
        }
    }
}
