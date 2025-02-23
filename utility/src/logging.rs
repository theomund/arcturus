// Arcturus - Hobbyist operating system written in Rust.
// Copyright (C) 2025 Theomund
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use core::cell::LazyCell;
use core::fmt::{Arguments, Error};

use crate::lock::Spinlock;

pub enum Level {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
}

pub trait Log: Sync + Send {
    fn handler(&self, level: Level, arguments: Arguments<'_>) -> Result<(), Error>;
}

static LOGGER: Spinlock<LazyCell<Option<&dyn Log>>> = Spinlock::new(LazyCell::new(|| None));

pub fn set_logger(logger: &'static dyn Log) {
    let guard = &mut LOGGER.lock();
    let global_logger = LazyCell::force_mut(guard);

    *global_logger = Some(logger);
}

pub fn get_logger() -> Option<&'static dyn Log> {
    **LOGGER.lock()
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arguments:tt)+) => {
        if let Some(logger) = $crate::logging::get_logger() {
            logger.handler($level, format_args!($($arguments)+)).expect("Failed to write log message.");
        }
    };
}

#[macro_export]
macro_rules! debug {
    ($($arguments:tt)+) => ($crate::log!($crate::logging::Level::Debug, $($arguments)+));
}

#[macro_export]
macro_rules! error {
    ($($arguments:tt)+) => ($crate::log!($crate::logging::Level::Error, $($arguments)+));
}

#[macro_export]
macro_rules! info {
    ($($arguments:tt)+) => ($crate::log!($crate::logging::Level::Info, $($arguments)+));
}

#[macro_export]
macro_rules! trace {
    ($($arguments:tt)+) => ($crate::log!($crate::logging::Level::Trace, $($arguments)+));
}

#[macro_export]
macro_rules! warn {
    ($($arguments:tt)+) => ($crate::log!($crate::logging::Level::Warn, $($arguments)+));
}
