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
use core::fmt::{Arguments, Error, Write, write};
use utility::logging::{Level, Log, set_logger};

use crate::serial::COM1;

struct SerialLogger;

impl Log for SerialLogger {
    fn handler(&self, level: Level, arguments: Arguments<'_>) -> Result<(), Error> {
        let guard = &mut COM1.lock();
        let port = LazyCell::force_mut(guard);

        match level {
            Level::Debug => {
                write!(port, "\x1b[38;5;34m")?;
                write!(port, "[DEBUG]")?;
                write!(port, "\x1b[0m ")?;
            }
            Level::Error => {
                write!(port, "\x1b[38;5;160m")?;
                write!(port, "[ERROR]")?;
                write!(port, "\x1b[0m ")?;
            }
            Level::Info => {
                write!(port, "\x1b[38;5;39m")?;
                write!(port, "[INFO]")?;
                write!(port, "\x1b[0m ")?;
            }
            Level::Trace => {
                write!(port, "\x1b[38;5;135m")?;
                write!(port, "[TRACE]")?;
                write!(port, "\x1b[0m ")?;
            }
            Level::Warn => {
                write!(port, "\x1b[38;5;184m")?;
                write!(port, "[WARN]")?;
                write!(port, "\x1b[0m ")?;
            }
        }

        write(port, arguments)?;

        writeln!(port)?;

        Ok(())
    }
}

static SERIAL_LOGGER: SerialLogger = SerialLogger;

pub fn init() {
    set_logger(&SERIAL_LOGGER);
}
