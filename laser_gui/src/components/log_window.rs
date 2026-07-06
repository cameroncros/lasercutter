use crate::style::*;
use dioxus::prelude::*;

#[component]
pub fn LogWindow(msglog: Signal<Vec<String>>, on_close: EventHandler<()>) -> Element {
    rsx! {
        div { class: LOG_WINDOW_OVERLAY_CLASSES,
            style: "bottom: anchor(--status-bar top); left: anchor(--status-bar left)",
            div { class: LOG_WINDOW_CLASSES,
                // div { "{msglog.read().len()}" }
                div { class: LOG_WINDOW_CONTENT_CLASSES,
                    for msg in msglog.read().iter() {
                        div { key: "{msg}", class: "mb-1", "{msg}" }
                    }
                }
            }
        }
    }
}
