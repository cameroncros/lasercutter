use std::sync::{LazyLock, Mutex};
use std::time::SystemTime;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tracing::field::{Field, Visit};
use tracing::level_filters::LevelFilter;
use tracing::{Event, Level, Subscriber};
use tracing_subscriber::layer::Context;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[derive(Debug, Clone, PartialEq)]
pub struct LogEvent {
    pub timestamp: SystemTime,
    pub level: Level,
    pub target: String,
    pub message: String,
}

pub struct LogLayer {
    tx: UnboundedSender<LogEvent>,
}

struct MessageVisitor {
    message: Option<String>,
}

impl Visit for MessageVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_owned());
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        }
    }
}

fn extract_message(event: &Event<'_>) -> Option<String> {
    let mut visitor = MessageVisitor { message: None };
    event.record(&mut visitor);
    visitor.message
}

impl<S: Subscriber> tracing_subscriber::Layer<S> for LogLayer {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let timestamp = SystemTime::now();
        let level = event.metadata().level();
        let target = event.metadata().target().to_string();
        let message = extract_message(event).unwrap_or_default();

        let _ = self.tx.send(LogEvent {
            timestamp,
            level: *level,
            target,
            message,
        });
    }
}

pub static LOG_RX: LazyLock<Mutex<Option<UnboundedReceiver<LogEvent>>>> =
    LazyLock::new(|| Mutex::new(None));

pub fn init_tracing() {
    let (tx, rx) = unbounded_channel();

    let filter = EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .from_env_lossy()
        .add_directive(LevelFilter::DEBUG.into())
        .add_directive("dioxus=off".parse().unwrap());

    tracing_subscriber::registry()
        .with(filter)
        .with(LogLayer { tx })
        .init();

    let mut lock = (*LOG_RX).lock().unwrap();
    *lock = Some(rx);
}
