use crate::style::{STATUS_BAR_CLASSES, STATUS_BAR_SUBTEXT_CLASSES};
use crate::tracing_logger::LogEvent;
use dioxus::prelude::*;

#[component]
pub fn StatusBar(
    msglog: Signal<Vec<LogEvent>>,
    rendertime: Signal<String>,
    show_log: Signal<bool>,
    on_toggle_log: EventHandler<()>,
) -> Element {
    let status_msg = match msglog.last() {
        None => String::new(),
        Some(event) => format!("[{:?}] {}: {}", event.level, event.target, event.message),
    };

    rsx! {
        div {
            class: STATUS_BAR_CLASSES,
            style: "anchor-name: --status-bar",
            onclick: move |_| on_toggle_log.call(()),
            span { "Status: {status_msg}" }
            span { class: STATUS_BAR_SUBTEXT_CLASSES, "{rendertime}" }
        }
    }
}
