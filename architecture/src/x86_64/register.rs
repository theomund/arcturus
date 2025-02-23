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

use super::gdt::Selector;

use core::arch::asm;

pub struct CS;

impl CS {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, cs", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!(
                "push {selector}",
                "lea {address}, [2f + rip]",
                "push {address}",
                "retfq",
                "2:",
                selector = in(reg) u64::from(selector.0),
                address = lateout(reg) _,
            );
        }
    }
}

pub struct DS;

impl DS {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, ds", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!("mov ds, {0:x}", in(reg) selector.0);
        }
    }
}

pub struct ES;

impl ES {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, es", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!("mov es, {0:x}", in(reg) selector.0);
        }
    }
}

pub struct FS;

impl FS {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, fs", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!("mov fs, {0:x}", in(reg) selector.0);
        }
    }
}

pub struct GS;

impl GS {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, gs", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!("mov gs, {0:x}", in(reg) selector.0);
        }
    }
}

pub struct SS;

impl SS {
    #[must_use]
    pub fn get() -> Selector {
        let value: u16;
        unsafe {
            asm!("mov {0:x}, ss", out(reg) value);
        }
        Selector(value)
    }

    pub fn set(selector: Selector) {
        unsafe {
            asm!("mov ss, {0:x}", in(reg) selector.0);
        }
    }
}
