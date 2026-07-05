use dioxus::prelude::*;
use lucide_dioxus::{Flame, House, MoveDown, MoveLeft, MoveRight, MoveUp};

use crate::style::*;

#[component]
pub(crate) fn MachineControls() -> Element {
    let mut rapid_rate = use_signal(|| 1.0);
    rsx! {
        details { class: DETAILS_CLASSES, open: true, // TODO: Hide when unconnected, open when connected
            summary { class: SUMMARY_CLASSES, "Machine Controls" }
            div { class: GRID_CONTAINER_CLASSES,
                div { class: GRID_ROW_CLASSES,
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            Flame {}
                        }
                    }
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            MoveUp {}
                        }
                    }
                    div { class: GRID_CELL_EMPTY_CLASSES }
                }
                div { class: GRID_ROW_CLASSES,
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            MoveLeft {}
                        }
                    }
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            House {}
                        }
                    }
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            MoveRight {}
                        }
                    }
                }
                div { class: GRID_ROW_CLASSES,
                    div { class: GRID_CELL_EMPTY_CLASSES }
                    div { class: GRID_CELL_CLASSES,
                        button {
                            onclick: move |_| {},
                            MoveDown {}
                        }
                    }
                    div { class: GRID_CELL_EMPTY_CLASSES }
                }
                div { class: GRID_ROW_CLASSES,
                    button {
                        class: format!("{} {} {}", RAPID_BUTTON_LEFT_CLASSES, RAPID_BUTTON_CLASSES, if *rapid_rate.read() == 0.1 { "bg-gray-400" } else { "hover:bg-gray-400" }),
                        disabled: *rapid_rate.read() == 0.1,
                        onclick: move |_| {
                            rapid_rate.set(0.1);
                        },
                        "0.1"
                    }
                    button {
                        class: format!("{} {}", RAPID_BUTTON_CLASSES, if *rapid_rate.read() == 1.0 { "bg-gray-400" } else { "hover:bg-gray-400" }),
                        disabled: *rapid_rate.read() == 1.0,
                        onclick: move |_| {
                            rapid_rate.set(1.0);
                        },
                        "1"
                    }
                    button {
                        class: format!("{} {}", RAPID_BUTTON_CLASSES, if *rapid_rate.read() == 10.0 { "bg-gray-400" } else { "hover:bg-gray-400" }),
                        disabled: *rapid_rate.read() == 10.0,
                        onclick: move |_| {
                            rapid_rate.set(10.0);
                        },
                        "10"
                    }
                    button {
                        class: format!("{} {} {}", RAPID_BUTTON_RIGHT_CLASSES, RAPID_BUTTON_CLASSES, if *rapid_rate.read() == 100.0 { "bg-gray-400" } else { "hover:bg-gray-400" }),
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
