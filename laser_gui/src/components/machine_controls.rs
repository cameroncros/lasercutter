use dioxus::prelude::*;
use lucide_dioxus::{Flame, House, MoveDown, MoveLeft, MoveRight, MoveUp};

use crate::{components::repeat_button::RepeatButton, style::*};

#[component]
pub(crate) fn MachineControls() -> Element {
    let mut rapid_rate = use_signal(|| 1.0);
    rsx! {
        details { class: DETAILS_CLASSES, open: true, // TODO: Hide when unconnected, open when connected
            summary { class: SUMMARY_CLASSES, "Machine Controls" }
            div { class: "px-2",
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton { repeat_fn: move |_| {}, Flame {} }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton { repeat_fn: move |_| {}, MoveUp {} }
                    }
                    div { class: "w-1/3 px-2" }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2",
                        RepeatButton { repeat_fn: move |_| {}, MoveLeft {} }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton { // Doesnt need repeat
                            repeat_fn: move |_| {},
                            House {}
                        }
                    }
                    div { class: "w-1/3 px-2",
                        RepeatButton { repeat_fn: move |_| {}, MoveRight {} }
                    }
                }
                div { class: "flex -mx-2",
                    div { class: "w-1/3 px-2" }
                    div { class: "w-1/3 px-2",
                        RepeatButton { repeat_fn: move |_| {}, MoveDown {} }
                    }
                    div { class: "w-1/3 px-2" }
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
