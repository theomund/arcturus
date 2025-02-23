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

#![no_main]
#![no_std]
#![warn(clippy::pedantic)]
#![feature(lazy_get)]

mod gdt;
mod logger;
mod serial;
mod tss;

use architecture::x86_64::instruction;
use core::cell::LazyCell;
use core::panic::PanicInfo;

#[unsafe(no_mangle)]
extern "C" fn kmain() -> ! {
    serial::init();

    gdt::init();

    tss::init();

    info!("Successfully initialized the operating system.");

    done();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info.message());

    done();
}

fn done() -> ! {
    instruction::cli();
    loop {
        instruction::hlt();
    }
}
