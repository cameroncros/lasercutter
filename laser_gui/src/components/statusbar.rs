use dioxus::prelude::*;

#[component]
pub fn StatusBar(errormsg: Signal<String>, rendertime: Signal<String>) -> Element {
    let status_msg = if !errormsg().is_empty() {
        errormsg()
    } else {
        String::new()
    };

    rsx! {
        div { class: "bg-gray-900 text-gray-100 px-4 py-2 text-sm flex items-center justify-between",
            span { "Status: {status_msg}" }
            span { class: "text-gray-400", "{rendertime}" }
        }
    }
}
