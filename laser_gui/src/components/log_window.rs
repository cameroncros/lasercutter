use dioxus::prelude::*;
use crate::style::*;

#[component]
pub fn LogWindow(msglog: Signal<Vec<String>>, on_close: EventHandler<()>) -> Element {
    rsx! {
        div { class: LOG_WINDOW_OVERLAY_CLASSES,
            div { class: LOG_WINDOW_CLASSES,
                div { class: LOG_WINDOW_HEADER_CLASSES,
                    span { class: "text-white font-bold", "System Log" }
                    button {
                        class: LOG_WINDOW_CLOSE_BUTTON_CLASSES,
                        onclick: move |_| on_close.call(()),
                        "Close"
                    }
                }
                div { class: LOG_WINDOW_CONTENT_CLASSES,
                    for msg in msglog().iter() {
                        div { key: "{msg}", class: "mb-1", "{msg}" }
                    }
                }
            }
        }
    }
}
