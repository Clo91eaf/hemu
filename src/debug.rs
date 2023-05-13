use log::{debug, error, info, warn};
use std::env;

pub fn debug_init() {
  let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

  let filter_level = match log_level.as_str() {
    "error" => log::LevelFilter::Error,
    "warn" => log::LevelFilter::Warn,
    "info" => log::LevelFilter::Info,
    "debug" => log::LevelFilter::Debug,
    "trace" => log::LevelFilter::Trace,
    _ => panic!("Invalid log level: {}", log_level),
  };
  env::set_var("RUST_LOG", log_level);

  env_logger::builder().filter_level(filter_level).init();

  error!("This is an error message");
  warn!("This is a warning message");
  info!("This is an info message");
  debug!("This is a debug message");
}
