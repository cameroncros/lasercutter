use crate::style::{STATUS_BAR_CLASSES, STATUS_BAR_SUBTEXT_CLASSES};
use dioxus::prelude::*;

#[component]
pub fn StatusBar(errormsg: Signal<String>, rendertime: Signal<String>) -> Element {
    let status_msg = if !errormsg().is_empty() {
        errormsg()
    } else {
        String::new()
    };

    rsx! {
        div { class: STATUS_BAR_CLASSES,
            span { "Status: {status_msg}" }
            span { class: STATUS_BAR_SUBTEXT_CLASSES, "{rendertime}" }
        }
    }
}
