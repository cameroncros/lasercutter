use crate::tracing_logger::LogEvent;
use chrono::{DateTime, Local};
use dioxus::prelude::*;
use tracing::Level;

#[component]
pub fn LogEntry(event: LogEvent) -> Element {
    let localtime: DateTime<Local> = event.timestamp.into();
    let timestamp_string = localtime.format("%Y-%m-%d %H:%M:%S");
    let level_color = match event.level {
        Level::ERROR => "text-red-500",
        Level::WARN => "text-yellow-500",
        Level::INFO => "text-blue-500",
        Level::DEBUG => "text-purple-500",
        Level::TRACE => "text-gray-500",
    };
    rsx! {
        div { class: "flex items-center gap-4 hover:bg-gray-700",
            div { class: "w-1/5 text-xs", "{timestamp_string}" }
            div { class: format!("w-1/5 text-xs {}", level_color), "{event.level}" }
            div { class: "w-1/5 text-xs", "{event.target}" }
            div { class: "flex-1 text-xs", "{event.message}" }
        }
    }
}
