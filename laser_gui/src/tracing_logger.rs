use std::io;
use std::sync::{LazyLock, Mutex};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::fmt::MakeWriter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

pub struct LogLayer {
    tx: UnboundedSender<String>,
}

impl LogLayer {}

impl io::Write for LogLayer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tx
            .send(String::from_utf8_lossy(buf).to_string())
            .map_err(io::Error::other)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!()
    }
}

struct LogMaker {
    tx: UnboundedSender<String>,
}

impl MakeWriter<'_> for LogMaker {
    type Writer = LogLayer;

    fn make_writer(&self) -> Self::Writer {
        LogLayer {
            tx: self.tx.clone(),
        }
    }
}

pub static LOG_RX: LazyLock<Mutex<Option<UnboundedReceiver<String>>>> =
    LazyLock::new(|| Mutex::new(None));

pub fn init_tracing() {
    let (tx, rx) = unbounded_channel();

    let filter = EnvFilter::builder()
        .with_env_var("RUST_LOG")
        .from_env_lossy()
        // Set the base level when not matched by other directives to WARN.
        .add_directive(LevelFilter::WARN.into())
        // Set the max level for `my_crate::my_mod` to DEBUG, overriding
        // any directives parsed from the env variable.
        .add_directive("dioxus=off".parse().unwrap());

    tracing_subscriber::fmt()
        .with_target(true)
        .with_thread_names(true)
        .with_writer(LogMaker { tx })
        .with_env_filter(filter)
        .init();

    let mut lock = (*LOG_RX).lock().unwrap();
    *lock = Some(rx);
}
