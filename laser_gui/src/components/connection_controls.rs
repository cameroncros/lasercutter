use std::{
    cmp::PartialEq,
    fmt::{Display, Formatter},
};

use dioxus::prelude::*;

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
        details {
            class: "mb-4 border border-gray-200 rounded-lg open:shadow-lg transition-shadow duration-300 bg-gray-700 text-white text-xs font-semibold px-2 py-1 rounded w-full",
            open: !*connected.read(),
            summary { class: "p-4 font-semibold cursor-pointer bg-gray-100 hover:bg-gray-200 list-none bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
                "Connection Settings"
            }
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
                    class: "flex flex-row justify-center items-center bg-gray-700 hover:bg-gray-800 text-white text-xs font-semibold px-2 py-1 rounded w-full",
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
