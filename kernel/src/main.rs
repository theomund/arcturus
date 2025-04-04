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
#![feature(abi_x86_interrupt)]

mod boot;
mod gdt;
mod idt;
mod isr;
mod logger;
mod serial;
mod tss;
mod vga;

use architecture::x86_64::instruction;
use core::panic::PanicInfo;
use utility::{error, info};

#[unsafe(no_mangle)]
extern "C" fn kmain() -> ! {
    logger::init();

    serial::init();

    boot::init();

    gdt::init();

    tss::init();

    idt::init();

    vga::init();

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
