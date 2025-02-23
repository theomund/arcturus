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

use architecture::x86_64::serial::{Port, Ports};
use core::cell::LazyCell;
use core::fmt::{Result, Write};
use utility::lock::Spinlock;

use crate::info;

pub static COM1: Spinlock<LazyCell<Port>> = Spinlock::new(LazyCell::new(|| Port::new(Ports::COM1)));

pub fn setup_title() -> Result {
    let guard = &mut COM1.lock();
    let port = LazyCell::force_mut(guard);

    write!(port, "\x1b[38;5;202m")?;
    write!(port, "\x1b[1m")?;
    writeln!(port, "Arcturus v0.1.0 (x86_64)")?;
    writeln!(port, "Copyright (C) 2025 Theomund")?;
    writeln!(port, "\x1b[0m")?;

    Ok(())
}

pub fn init() {
    setup_title().expect("Failed to setup title bar.");

    info!("Initialized the serial port driver.");
}
