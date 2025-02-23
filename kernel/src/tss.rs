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

use architecture::x86_64::tss::Segment;
use core::cell::LazyCell;

use crate::gdt::GDT;
use crate::lock::Spinlock;

pub static TSS: Spinlock<LazyCell<Segment>> = Spinlock::new(LazyCell::new(Segment::new));

pub fn init() {
    Segment::load(GDT.lock().selector(5));
}
