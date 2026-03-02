use dioxus::prelude::*;
use laser_cutter::gcode_generator::{
    operation::{cut::Cut, raster::Raster, Operation},
    workspace::Workspace,
};
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
    Trash2,
};

use crate::{components::repeat_button::RepeatButton, style::*};

#[component]
pub fn MoveControls(workspace: Signal<Workspace>, index: usize) -> Element {
    let mut rapid_rate = use_signal(|| 1.0);
    let scale_step = 1.1f32;
    rsx! {
        div { class: DETAILS_CLASSES, // TODO: Hide when unconnected, open when connected
            summary { class: SUMMARY_CLASSES, "Move Controls" }

            div { class: "px-2",
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton {
                            repeat_fn: move || {
                                let mut workspace = workspace.write();
                                if let Some(cut) = workspace.items.get_mut(index) {
                                    match cut {
                                        Operation::Cut(c) => c.transform.scale(1.0 / scale_step),
                                        Operation::Raster(r) => r.transform.scale(1.0 / scale_step),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.translate(0.0, -*rapid_rate.read()),
                                        Operation::Raster(r) => r.transform.translate(0.0, -*rapid_rate.read()),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.scale(scale_step),
                                        Operation::Raster(r) => r.transform.scale(scale_step),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.translate(-*rapid_rate.read(), 0.0),
                                        Operation::Raster(r) => r.transform.translate(-*rapid_rate.read(), 0.0),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.reset(),
                                        Operation::Raster(r) => r.transform.reset(),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.translate(*rapid_rate.read(), 0.0),
                                        Operation::Raster(r) => r.transform.translate(*rapid_rate.read(), 0.0),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.rotate(-*rapid_rate.read()),
                                        Operation::Raster(r) => r.transform.rotate(-*rapid_rate.read()),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.translate(0.0, *rapid_rate.read()),
                                        Operation::Raster(r) => r.transform.translate(0.0, *rapid_rate.read()),
                                    };
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
                                    match cut {
                                        Operation::Cut(c) => c.transform.rotate(*rapid_rate.read()),
                                        Operation::Raster(r) => r.transform.rotate(*rapid_rate.read()),
                                    };
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
pub fn CutElem(cut: Cut, workspace: Signal<Workspace>, index: usize, is_last: bool) -> Element {
    rsx! {
        details { class: DETAILS_CLASSES,
            summary { class: SUMMARY_CLASSES,
                div { class: "flex items-center justify-between text-white",
                    "{cut}"
                    div { class: "flex gap-2",
                        button {
                            class: BUTTON_CLASSES,
                            visibility: if index == 0 { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index - 1);
                            },
                            MoveUp {}
                        }
                        button {
                            class: BUTTON_CLASSES,
                            visibility: if is_last { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index + 1);
                            },
                            MoveDown {}
                        }
                        button {
                            class: BUTTON_CLASSES,
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.remove(index);
                            },
                            Trash2 {}
                        }
                    }
                }

            }
            // Controls section
            MoveControls { workspace, index }
        }
    }
}

#[component]
pub fn RasterElem(
    raster: Raster,
    index: usize,
    is_last: bool,
    workspace: Signal<Workspace>,
) -> Element {
    let mut raster_scale = use_signal(|| 1.0);
    let mut raster_angle = use_signal(|| 0.0);
    rsx! {
        div { class: DETAILS_CLASSES,
            summary { class: SUMMARY_CLASSES,
                div { class: "flex items-center justify-between text-white",
                    "{raster}"
                    div { class: "flex gap-2",
                        button {
                            class: BUTTON_CLASSES,
                            visibility: if index == 0 { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index - 1);
                            },
                            MoveUp {}
                        }
                        button {
                            class: BUTTON_CLASSES,
                            visibility: if is_last { "hidden" } else { "visible" },
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.swap(index, index + 1);
                            },
                            MoveDown {}
                        }
                        button {
                            class: BUTTON_CLASSES,
                            onclick: move |_| {
                                let mut workspace = workspace.write();
                                workspace.items.remove(index);
                            },
                            Trash2 {}
                        }
                    }
                }
            }
            // Raster controls
            details { class: DETAILS_CLASSES, open: true, // TODO: Hide when unconnected, open when connected
                summary { class: SUMMARY_CLASSES, "Raster Controls" }
                div { class: "flex -mx-2",
                    div { class: "w-1/4 px-2",
                        RepeatButton { repeat_fn: move || { *raster_scale.write() -= 0.1 },
                            Minus {}
                        }
                    }
                    div { class: "w-1/2 px-2",
                        input { value: "{raster_scale}" }
                    }
                    div { class: "w-1/4 px-2",
                        RepeatButton { repeat_fn: move |_| { *raster_scale.write() += 0.1 },
                            Plus {}
                        }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/4 px-2",
                        RepeatButton {
                            repeat_fn: move || {
                                            let current = *raster_angle.read();
                                let mut new = (current - 1.0) % 360.0;
                                while new < 0.0 {
                                    new += 360.0;
                                }
                                *raster_angle.write() = new;
                            },
                            RefreshCw {}
                        }
                    }
                    div { class: "w-1/2 px-2",
                        input { value: "{raster_angle}" }
                    }
                    div { class: "w-1/4 px-2",
                        RepeatButton {
                            repeat_fn: move |_| {
                                let current = *raster_angle.read();
                                *raster_angle.write() = (current + 1.0) % 360.0;
                            },
                            RefreshCcw {}
                        }
                    }
                }
            }

            // Controls section
            MoveControls { workspace, index }
        }
    }
}

#[component]
pub fn CutList(workspace: Signal<Workspace>) -> Element {
    let workspace_read = workspace.read();

    rsx! {
        for (index , cut) in workspace_read.items().iter().enumerate() {
            match cut {
                Operation::Cut(cut) => {
                    rsx! {
                        CutElem {
                            cut: cut.clone(),
                            index,
                            is_last: index == workspace_read.items().len() - 1,
                            workspace,
                        }
                    }
                }
                Operation::Raster(raster) => {
                    rsx! {
                        RasterElem {
                            raster: raster.clone(),
                            index,
                            is_last: index == workspace_read.items().len() - 1,
                            workspace,
                        }
                    }
                }
            }
        }
    }
}
