use dioxus::prelude::*;
use laser_cutter::gcode_generator::{cut::Cut, workspace::Workspace};

#[component]
pub fn CutElem(cut: Cut, index: usize, is_last: bool, workspace: Signal<Workspace>) -> Element {
    let mut sc1 = use_signal(|| false);
    let scale_step = 1.1f32;
    let rotate_step = 5.0f32;
    let translate_step = 5.0f32;
    rsx! {
        div {
            onclick: move |_| {
                let show = *sc1.read();
                sc1.set(!show);
            },
            class: "w-full",
            h2 { "{cut}" }
            if index != 0 {
                button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    "^"
                }
            }
            if !is_last {
                button { class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
                    svg {}
                }
            }
        }
        if *sc1.read() {
            // Controls section
            div { class: "px-2",
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(1.0 / scale_step);
                                }
                            },
                            "Scale -"
                        }
                    }
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, -translate_step);
                                }

                            },
                            "Up"
                        }
                    }
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(scale_step);
                                }
                            },
                            "Scale +"
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(-translate_step, 0.0);
                                }
                            },
                            "Left"
                        }
                    }
                    div { class: "w-1/3 px-2" }
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(translate_step, 0.0);
                                }
                            },
                            "Right"
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(-rotate_step);
                                }
                            },
                            "Rotate -"
                        }
                    }
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, translate_step);
                                }
                            },
                            "Down"
                        }
                    }
                    div { class: "w-1/3 px-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(rotate_step);
                                }
                            },
                            "Rotate +"
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn CutList(workspace: Signal<Workspace>) -> Element {
    let workspace_read = workspace.read();

    rsx! {
        for (index , cut) in workspace_read.items().iter().enumerate() {
            CutElem {
                cut: cut.clone(),
                index,
                is_last: index == workspace_read.items().len() - 1,
                workspace,
            }
        }
    }
}
