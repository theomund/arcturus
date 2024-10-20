// Arcturus - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
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

pub mod instructions {
    use core::arch::asm;

    use crate::gdt::DescriptorTablePointer;

    pub fn cli() {
        unsafe {
            asm!("cli", options(nostack, preserves_flags));
        }
    }

    pub fn hlt() {
        unsafe {
            asm!("hlt", options(nomem, nostack));
        }
    }

    pub fn inb(address: u16) -> u8 {
        let value: u8;
        unsafe {
            asm!("in al, dx", out("al") value, in("dx") address, options(nomem, nostack, preserves_flags));
        }
        value
    }

    pub fn lgdt(pointer: &DescriptorTablePointer) {
        unsafe {
            asm!("lgdt [{}]", in(reg) pointer, options(readonly, nostack, preserves_flags));
        }
    }

    pub fn outb(address: u16, value: u8) {
        unsafe {
            asm!("out dx, al", in("dx") address, in("al") value, options(nomem, nostack, preserves_flags));
        }
    }
}

pub mod registers {
    use core::arch::asm;

    pub trait Segment {
        fn set(selector: u16);
    }

    pub struct CS;

    impl Segment for CS {
        fn set(selector: u16) {
            unsafe {
                asm!(
                    "push {sel}",
                    "lea {tmp}, [2f + rip]",
                    "push {tmp}",
                    "retfq",
                    "2:",
                    sel = in(reg) u64::from(selector),
                    tmp = lateout(reg) _,
                    options(preserves_flags)
                );
            }
        }
    }

    pub struct DS;

    impl Segment for DS {
        fn set(selector: u16) {
            unsafe {
                asm!("mov ds, {0:x}", in(reg) selector, options(nostack, preserves_flags));
            }
        }
    }

    pub struct ES;

    impl Segment for ES {
        fn set(selector: u16) {
            unsafe {
                asm!("mov es, {0:x}", in(reg) selector, options(nostack, preserves_flags));
            }
        }
    }

    pub struct FS;

    impl Segment for FS {
        fn set(selector: u16) {
            unsafe {
                asm!("mov fs, {0:x}", in(reg) selector, options(nostack, preserves_flags));
            }
        }
    }

    pub struct GS;

    impl Segment for GS {
        fn set(selector: u16) {
            unsafe {
                asm!("mov gs, {0:x}", in(reg) selector, options(nostack, preserves_flags));
            }
        }
    }

    pub struct SS;

    impl Segment for SS {
        fn set(selector: u16) {
            unsafe {
                asm!("mov ss, {0:x}", in(reg) selector, options(nostack, preserves_flags));
            }
        }
    }
}
