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

pub enum Level {
    Debug,
    Error,
    Info,
    Trace,
    Warn,
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arguments:tt)+) => {{
        use core::fmt::Write;

        let guard = &mut $crate::serial::COM1.lock();
        let port = LazyCell::force_mut(guard);

        match $level {
            $crate::logger::Level::Debug => {
                writeln!(port, "[DEBUG] {}", format_args!($($arguments)+)).ok();
            }
            $crate::logger::Level::Error => {
                writeln!(port, "[ERROR] {}", format_args!($($arguments)+)).ok();
            }
            $crate::logger::Level::Info => {
                writeln!(port, "[INFO] {}", format_args!($($arguments)+)).ok();
            }
            $crate::logger::Level::Trace => {
                writeln!(port, "[TRACE] {}", format_args!($($arguments)+)).ok();
            }
            $crate::logger::Level::Warn => {
                writeln!(port, "[WARN] {}", format_args!($($arguments)+)).ok();
            }
        }
    }};
}

#[macro_export]
macro_rules! debug {
    ($($arguments:tt)+) => ($crate::log!($crate::logger::Level::Debug, $($arguments)+));
}

#[macro_export]
macro_rules! error {
    ($($arguments:tt)+) => ($crate::log!($crate::logger::Level::Error, $($arguments)+));
}

#[macro_export]
macro_rules! info {
    ($($arguments:tt)+) => ($crate::log!($crate::logger::Level::Info, $($arguments)+));
}

#[macro_export]
macro_rules! trace {
    ($($arguments:tt)+) => ($crate::log!($crate::logger::Level::Trace, $($arguments)+));
}

#[macro_export]
macro_rules! warn {
    ($($arguments:tt)+) => ($crate::log!($crate::logger::Level::Warn, $($arguments)+));
}
