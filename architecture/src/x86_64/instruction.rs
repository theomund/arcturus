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

use super::{gdt, idt};

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

pub fn int<const VECTOR: u8>() {
    unsafe {
        asm!("int {vector}", vector = const VECTOR);
    }
}

pub fn int3() {
    unsafe {
        asm!("int3");
    }
}

pub fn lgdt(register: &gdt::Register) {
    unsafe {
        asm!("lgdt [{}]", in(reg) register);
    }
}

pub fn lidt(register: &idt::Register) {
    unsafe {
        asm!("lidt [{}]", in(reg) register);
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
