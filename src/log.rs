use tracing::Level;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn log_trace(level: Level) {
  let global_logger = FmtSubscriber::builder()
    .with_env_filter(EnvFilter::from_default_env())
    .with_max_level(level)
    .without_time()
    .with_target(false)
    .compact()
    .finish();
  tracing::subscriber::set_global_default(global_logger).expect("internal error: fail to setup log subscriber");
}
