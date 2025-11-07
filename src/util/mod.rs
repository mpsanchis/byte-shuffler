mod logger;

use logger::Logger;
pub use logger::Verbosity;
use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

pub fn init_logger(verbosity: Verbosity) {
    LOGGER
        .set(Logger::from(verbosity))
        .expect("Logger already initialised")
}

pub fn logger() -> &'static Logger {
    LOGGER.get().expect("Logger not initialized")
}
