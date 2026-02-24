use std::time::{Duration, Instant};

use dioxus::{core::Task, prelude::*};
use laser_cutter::gcode_generator::{cut::Cut, workspace::Workspace};
use lucide_dioxus::{
    Minus,
    MoveDown,
    MoveLeft,
    MoveRight,
    MoveUp,
    OctagonMinus,
    Plus,
    RefreshCcw,
    RefreshCw,
};

use crate::components::repeat_button::RepeatButton;

#[component]
pub fn CutElem(cut: Cut, index: usize, is_last: bool, workspace: Signal<Workspace>) -> Element {
    let mut rapid_rate = use_signal(|| 1.0);
    let scale_step = 1.1f32;
    rsx! {
        details { class: "mb-4 border border-gray-200 rounded-lg open:shadow-lg transition-shadow duration-300 bg-gray-700 text-white text-xs font-semibold px-2 py-1 rounded w-full",
            summary { class: "p-4 font-semibold cursor-pointer bg-gray-100 hover:bg-gray-200 list-none bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                div { class: "flex items-center justify-between text-white",
                    "{cut}"
                    div {
                        class: "flex gap-2",
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full gap-0",
                            visibility: if index == 0 { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index - 1);
                            },
                            MoveUp {}
                        }
                        button {
                            class: "bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full gap-0",
                            visibility: if is_last { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index + 1);
                            },
                            MoveDown {}
                        }
                    }
                }

            }
            // Controls section
            div { class: "px-2",
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move || {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(1.0 / scale_step);
                                }
                            },

                            Minus {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, -*rapid_rate.read());
                                }

                            },
                            MoveUp {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.scale(scale_step);
                                }
                            },
                            Plus {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(-*rapid_rate.read(), 0.0);
                                }
                            },
                            MoveLeft {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.reset();
                                }
                            },
                            OctagonMinus {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(*rapid_rate.read(), 0.0);
                                }
                            },
                            MoveRight {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(-*rapid_rate.read());
                                }
                            },
                            RefreshCw {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.translate(0.0, *rapid_rate.read());
                                }
                            },
                            MoveDown {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    cut.transform.rotate(*rapid_rate.read());
                                }
                            },
                            RefreshCcw {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    button {
                        class: "rounded-l",
                        class: "w-1/4 bg-gray-300 text-gray-800 font-bold py-2 px-4",
                        class: if *rapid_rate.read() == 0.1 { "bg-gray-400" } else { "hover:bg-gray-400" },
                        disabled: *rapid_rate.read() == 0.1,
                        onclick: move |_| {
                            rapid_rate.set(0.1);
                        },
                        "0.1"
                    }
                    button {
                        class: "w-1/4 bg-gray-300 text-gray-800 font-bold py-2 px-4",
                        class: if *rapid_rate.read() == 1.0 { "bg-gray-400" } else { "hover:bg-gray-400" },
                        disabled: *rapid_rate.read() == 1.0,
                        onclick: move |_| {
                            rapid_rate.set(1.0);
                        },
                        "1"
                    }
                    button {
                        class: "w-1/4 bg-gray-300 text-gray-800 font-bold py-2 px-4",
                        class: if *rapid_rate.read() == 10.0 { "bg-gray-400" } else { "hover:bg-gray-400" },
                        disabled: *rapid_rate.read() == 10.0,
                        onclick: move |_| {
                            rapid_rate.set(10.0);
                        },
                        "10"
                    }
                    button {
                        class: "rounded-r",
                        class: "w-1/4 bg-gray-300 text-gray-800 font-bold py-2 px-4",
                        class: if *rapid_rate.read() == 100.0 { "bg-gray-400" } else { "hover:bg-gray-400" },
                        disabled: *rapid_rate.read() == 100.0,
                        onclick: move |_| {
                            rapid_rate.set(100.0);
                        },
                        "100"
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
