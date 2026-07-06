use crate::style::{STATUS_BAR_CLASSES, STATUS_BAR_SUBTEXT_CLASSES};
use dioxus::prelude::*;

#[component]
pub fn StatusBar(
    msglog: Signal<Vec<String>>,
    rendertime: Signal<String>,
    show_log: Signal<bool>,
    on_toggle_log: EventHandler<()>,
) -> Element {
    let status_msg = match msglog.last() {
        None => String::new(),
        Some(s) => s.clone(),
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
