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
use super::instruction;

type Handler = extern "x86-interrupt" fn(Frame);
type HandlerWithCode = extern "x86-interrupt" fn(Frame, u64);

pub enum Gate {
    Null = 0xE,
    Interrupt = 0x8E,
    Trap = 0x8F,
}

#[repr(C, packed(2))]
pub struct Register {
    limit: u16,
    base: u64,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Descriptor {
    offset_low: u16,
    selector: Selector,
    interrupt_stack_table: u8,
    type_attributes: u8,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
}

impl Default for Descriptor {
    fn default() -> Self {
        Self {
            offset_low: 0,
            selector: Selector(0),
            interrupt_stack_table: 0,
            type_attributes: Gate::Null as u8,
            offset_middle: 0,
            offset_high: 0,
            reserved: 0,
        }
    }
}

impl Descriptor {
    #[must_use]
    pub fn new(
        handler: Handler,
        selector: Selector,
        interrupt_stack_table: u8,
        gate: Gate,
    ) -> Self {
        let address = handler as u64;

        Self {
            offset_low: (address & 0xFFFF) as u16,
            selector,
            interrupt_stack_table,
            type_attributes: gate as u8,
            offset_middle: ((address >> 16) & 0xFFFF) as u16,
            offset_high: (address >> 32) as u32,
            reserved: 0,
        }
    }

    #[must_use]
    pub fn new_with_code(
        handler: HandlerWithCode,
        selector: Selector,
        interrupt_stack_table: u8,
        gate: Gate,
    ) -> Self {
        let address = handler as u64;

        Self {
            offset_low: (address & 0xFFFF) as u16,
            selector,
            interrupt_stack_table,
            type_attributes: gate as u8,
            offset_middle: ((address >> 16) & 0xFFFF) as u16,
            offset_high: (address >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(C)]
pub struct Frame {
    instruction_pointer: u64,
    code_segment: u16,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u16,
}

pub struct Table {
    descriptors: [Descriptor; 256],
}

impl Table {
    #[must_use]
    pub fn new(
        breakpoint_handler: Handler,
        segment_not_present_handler: HandlerWithCode,
        selector: Selector,
    ) -> Self {
        let null_descriptor = Descriptor::default();
        let breakpoint_descriptor = Descriptor::new(breakpoint_handler, selector, 0, Gate::Trap);
        let segment_not_present_descriptor =
            Descriptor::new_with_code(segment_not_present_handler, selector, 0, Gate::Interrupt);

        let mut descriptors = [null_descriptor; 256];

        descriptors[3] = breakpoint_descriptor;
        descriptors[11] = segment_not_present_descriptor;

        Self { descriptors }
    }

    fn base(&self) -> u64 {
        self.descriptors.as_ptr() as u64
    }

    fn limit(&self) -> u16 {
        (size_of_val(&self.descriptors) - 1) as u16
    }

    pub fn load(&self) {
        let register = self.register();
        instruction::lidt(&register);
        instruction::sti();
        instruction::int3();
    }

    fn register(&self) -> Register {
        Register {
            limit: self.limit(),
            base: self.base(),
        }
    }
}
