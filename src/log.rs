use rolling_file::{BasicRollingFileAppender, RollingConditionBasic};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{filter::EnvFilter, FmtSubscriber};

use crate::config::*;

pub fn init() -> WorkerGuard {
    let log_path: String = get_config(CONFIG_KEY_LOG_PATH);
    let max_files: usize = get_config(CONFIG_KEY_MAX_LOG_FILES);
    let with_color: bool = get_config(CONFIG_KEY_LOG_WITH_COLOR);

    let (non_blocking, worker_guard) = match log_path.as_str() {
        "stdout" => tracing_appender::non_blocking(std::io::stdout()),
        "stderr" => tracing_appender::non_blocking(std::io::stderr()),
        _ => {
            let file_appender = BasicRollingFileAppender::new(
                log_path,
                RollingConditionBasic::new().daily(),
                max_files,
            )
            .expect("build rolling file appender failed");
            tracing_appender::non_blocking(file_appender)
        }
    };

    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking)
        .with_ansi(with_color)
        .init();

    worker_guard
}
