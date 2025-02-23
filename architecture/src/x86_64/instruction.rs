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

use core::arch::asm;

use super::gdt;

pub fn cli() {
    unsafe {
        asm!("cli");
    }
}

pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}

#[must_use]
pub fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port);
    }
    value
}

pub fn lgdt(pointer: &gdt::Pointer) {
    unsafe {
        asm!("lgdt [{}]", in(reg) pointer);
    }
}

pub fn ltr(selector: gdt::Selector) {
    unsafe {
        asm!("ltr {0:x}", in(reg) selector.0);
    }
}

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value);
    }
}

pub fn sti() {
    unsafe {
        asm!("sti");
    }
}
