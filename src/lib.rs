//! A logger for the wasm32-unknown-unknown target that prints all messages to
//! the web console
//!
//! ## Usage
//!
//! ```no_run
//! #[macro_use]
//! extern crate log;
//! extern crate web_console_logger;
//! extern crate wasm_bindgen;
//!
//! use wasm_bindgen::prelude::*;
//!
//! #[wasm_bindgen]
//! pub fn main() {
//!     web_console_logger::init().unwrap();
//!     // OR
//!     // web_console_logger::init_with_level(log::Level::Warn).unwrap();
//!
//!     warn!("This example message will be printed to the web browser's javascript console.");
//! }
//! ```
//!
//! ## License
//!
//! Web Console Logger is licensed under either of
//!
//!  * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
//!    http://www.apache.org/licenses/LICENSE-2.0)
//!  * MIT license ([LICENSE-MIT](LICENSE-MIT) or
//!    http://opensource.org/licenses/MIT)
//!
//! at your option.

extern crate log;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

use log::{Level, Log, Metadata, Record, SetLoggerError};

struct WebConsoleLogger {
    level: Level,
}

impl Log for WebConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            web_sys::console::log_1(&JsValue::from_str(
                format!(
                    "{:<5} [{}] {}",
                    record.level().to_string(),
                    record.module_path().unwrap_or_default(),
                    record.args()
                ).as_str(),
            ));
        }
    }

    fn flush(&self) {}
}

/// Initializes logging with the provided maximum log level.
pub fn init_with_level(level: Level) -> Result<(), SetLoggerError> {
    let logger = WebConsoleLogger { level };
    log::set_boxed_logger(Box::new(logger))?;
    log::set_max_level(level.to_level_filter());
    Ok(())
}

/// Initializes logging with a maximum log level of `Level::Trace`.
pub fn init() -> Result<(), SetLoggerError> {
    init_with_level(Level::Trace)
}
