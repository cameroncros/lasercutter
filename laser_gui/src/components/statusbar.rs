use crate::style::{STATUS_BAR_CLASSES, STATUS_BAR_SUBTEXT_CLASSES};
use crate::components::log_window::LogWindow;
use dioxus::prelude::*;

#[component]
pub fn StatusBar(msglog: Signal<Vec<String>>, rendertime: Signal<String>) -> Element {
    let mut show_log = use_signal(|| false);
    let status_msg = if !msglog().is_empty() {
        msglog().last().unwrap_or(&"".to_string()).clone()
    } else {
        String::new()
    };

    rsx! {
        div { 
            class: STATUS_BAR_CLASSES,
            onclick: move |_| show_log.toggle(),
            span { "Status: {status_msg}" }
            span { class: STATUS_BAR_SUBTEXT_CLASSES, "{rendertime}" }
        }
        if show_log() {
            LogWindow { 
                msglog, 
                on_close: move |_| show_log.set(false) 
            }
        }
    }
}
