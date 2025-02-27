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

use architecture::x86_64::idt::Table;
use core::cell::LazyCell;
use utility::info;
use utility::lock::Spinlock;

use crate::gdt::GDT;
use crate::isr;

pub static IDT: Spinlock<LazyCell<Table>> = Spinlock::new(LazyCell::new(|| {
    Table::new(
        isr::breakpoint_handler,
        isr::segment_not_present_handler,
        GDT.lock().selector(1),
    )
}));

pub fn init() {
    IDT.lock().load();
    info!("Initialized the interrupt descriptor table.");
}
