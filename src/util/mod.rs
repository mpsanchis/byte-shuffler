mod logger;

use std::sync::OnceLock;
use logger::{Logger};
pub use logger::Verbosity;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init_logger(verbosity: Verbosity) {
    LOGGER.set(Logger::from(verbosity)).expect("Logger already initialised")
}

pub fn get_logger() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
}