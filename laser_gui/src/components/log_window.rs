use crate::components::log_entry::LogEntry;
use crate::style::*;
use crate::tracing_logger::LogEvent;
use dioxus::prelude::*;

#[component]
pub fn LogWindow(msglog: Signal<Vec<LogEvent>>) -> Element {
    rsx! {
        div { class: LOG_WINDOW_OVERLAY_CLASSES,
            style: "bottom: anchor(--status-bar top); left: anchor(--status-bar left)",
            div { class: LOG_WINDOW_CLASSES,
                div { class: LOG_WINDOW_CONTENT_CLASSES,
                    div { class: "flex flex-col gap-1",
                        div { class: "flex items-center gap-4 font-bold border-b border-gray-600 pb-1 text-gray-400",
                            div { class: "w-1/5", "Timestamp" }
                            div { class: "w-1/5", "Level" }
                            div { class: "w-1/5", "Target" }
                            div { class: "flex-1", "Message" }
                        }
                        for event in msglog.read().iter() {
                            LogEntry { event: event.clone() }
                        }
                    }
                }
            }
        }
    }
}
