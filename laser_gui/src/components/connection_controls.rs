use std::{
    cmp::PartialEq,
    fmt::{Display, Formatter},
};

use dioxus::prelude::*;

use crate::style::*;

const LOCAL: &str = "LOCAL";
const NETWORK: &str = "NETWORK";

#[derive(Debug, PartialEq)]
enum ConnectionType {
    Local,
    Network,
}

impl Display for ConnectionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConnectionType::Local => f.write_str(LOCAL),
            ConnectionType::Network => f.write_str(NETWORK),
        }
    }
}

#[component]
fn NetworkConnection() -> Element {
    rsx! { "TODO" }
}

#[component]
fn LocalConnection(connections: Vec<String>) -> Element {
    let mut connection = match connections.first() {
        None => use_signal(String::new),
        Some(s) => use_signal(|| (*s).clone()),
    };
    rsx! {
        select {
            value: "{connection}",
            onchange: move |v| { connection.set(v.value()) },
            for c in connections {
                option { value: c.clone(), selected: *connection.read() == c, "{c}" }
            }
        }
    }
}

#[component]
pub(crate) fn ConnectionControls() -> Element {
    let mut connected = use_signal(|| false);
    let mut connection_type = use_signal(|| ConnectionType::Local);
    rsx! {
        details { class: DETAILS_CLASSES, open: !*connected.read(),
            summary { class: SUMMARY_CLASSES, "Connection Settings" }
            div {
                select {
                    value: "{connection_type}",
                    onchange: move |v| {
                        match v.value().as_str() {
                            LOCAL => connection_type.set(ConnectionType::Local),
                            NETWORK => connection_type.set(ConnectionType::Network),
                            _ => {}
                        }
                    },
                    option {
                        value: LOCAL,
                        selected: *connection_type.read() == ConnectionType::Local,
                        "Local"
                    }
                    option {
                        value: NETWORK,
                        selected: *connection_type.read() == ConnectionType::Network,
                        "Network"
                    }
                }
            }
            if *connection_type.read() == ConnectionType::Local {
                LocalConnection { connections: vec![String::from("ertuy"), String::from("fglkh")] }
            } else {
                NetworkConnection {}
            }
            span {
                button {
                    class: BUTTON_CLASSES,
                    onclick: move |_| {
                        if *connected.read() {

                            connected.set(false);
                        } else {

                            connected.set(true);
                        }
                    },
                    if *connected.read() {
                        "Disconnect"
                    } else {
                        "Connect"
                    }
                }
            }
        }
    }
}
