use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_logger() -> WorkerGuard {
    let log_dir = Path::new("logs");

    let file_appender = tracing_appender::rolling::daily(log_dir, "app.log");
    let (non_blocking_file, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));

    let stdout_layer = fmt::layer().with_target(true).with_thread_ids(true);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(stdout_layer)
        .with(file_layer)
        .init();

    tracing::info!("Logger initialized. Logs are stored in ./logs/");

    guard
}
