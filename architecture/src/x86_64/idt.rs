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
type HaltHandler = extern "x86-interrupt" fn(Frame) -> !;
type ErrorHandler = extern "x86-interrupt" fn(Frame, u64);
type HaltErrorHandler = extern "x86-interrupt" fn(Frame, u64) -> !;

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
        let address = handler as usize as u64;

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

    pub fn new_with_halt(
        handler: HaltHandler,
        selector: Selector,
        interrupt_stack_table: u8,
        gate: Gate,
    ) -> Self {
        let address = handler as usize as u64;

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
    pub fn new_with_error(
        handler: ErrorHandler,
        selector: Selector,
        interrupt_stack_table: u8,
        gate: Gate,
    ) -> Self {
        let address = handler as usize as u64;

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

    pub fn new_with_halt_error(
        handler: HaltErrorHandler,
        selector: Selector,
        interrupt_stack_table: u8,
        gate: Gate,
    ) -> Self {
        let address = handler as usize as u64;

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
    code_segment: u64,
    cpu_flags: u64,
    stack_pointer: u64,
    stack_segment: u64,
}

pub struct Handlers {
    pub division_error_handler: Handler,
    pub debug_handler: Handler,
    pub non_maskable_interrupt_handler: Handler,
    pub breakpoint_handler: Handler,
    pub overflow_handler: Handler,
    pub bound_range_exceeded_handler: Handler,
    pub invalid_opcode_handler: Handler,
    pub device_not_available_handler: Handler,
    pub double_fault_handler: HaltErrorHandler,
    pub invalid_tss_handler: ErrorHandler,
    pub segment_not_present_handler: ErrorHandler,
    pub stack_segment_fault_handler: ErrorHandler,
    pub general_protection_fault_handler: ErrorHandler,
    pub page_fault_handler: ErrorHandler,
    pub x87_floating_point_handler: Handler,
    pub alignment_check_handler: ErrorHandler,
    pub machine_check_handler: HaltHandler,
    pub simd_floating_point_handler: Handler,
    pub virtualization_handler: Handler,
    pub control_protection_handler: ErrorHandler,
    pub hypervisor_injection_handler: Handler,
    pub vmm_communication_handler: ErrorHandler,
    pub security_handler: ErrorHandler,
}

pub struct Table {
    descriptors: [Descriptor; 256],
}

impl Table {
    #[must_use]
    pub fn new(handlers: &Handlers, selector: Selector) -> Self {
        let null_descriptor = Descriptor::default();
        let mut descriptors = [null_descriptor; 256];

        let division_error_descriptor = Descriptor::new(
            handlers.division_error_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[0] = division_error_descriptor;

        let debug_descriptor = Descriptor::new(handlers.debug_handler, selector, 0, Gate::Trap);
        descriptors[1] = debug_descriptor;

        let non_maskable_interrupt_descriptor = Descriptor::new(
            handlers.non_maskable_interrupt_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[2] = non_maskable_interrupt_descriptor;

        let breakpoint_descriptor =
            Descriptor::new(handlers.breakpoint_handler, selector, 0, Gate::Trap);
        descriptors[3] = breakpoint_descriptor;

        let overflow_descriptor =
            Descriptor::new(handlers.overflow_handler, selector, 0, Gate::Trap);
        descriptors[4] = overflow_descriptor;

        let bound_range_exceeded_descriptor = Descriptor::new(
            handlers.bound_range_exceeded_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[5] = bound_range_exceeded_descriptor;

        let invalid_opcode_descriptor = Descriptor::new(
            handlers.invalid_opcode_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[6] = invalid_opcode_descriptor;

        let device_not_available_descriptor = Descriptor::new(
            handlers.device_not_available_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[7] = device_not_available_descriptor;

        let double_fault_descriptor = Descriptor::new_with_halt_error(
            handlers.double_fault_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[8] = double_fault_descriptor;

        let invalid_tss_descriptor =
            Descriptor::new_with_error(handlers.invalid_tss_handler, selector, 0, Gate::Interrupt);
        descriptors[10] = invalid_tss_descriptor;

        let segment_not_present_descriptor = Descriptor::new_with_error(
            handlers.segment_not_present_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[11] = segment_not_present_descriptor;

        let stack_segment_fault_descriptor = Descriptor::new_with_error(
            handlers.stack_segment_fault_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[12] = stack_segment_fault_descriptor;

        let general_protection_fault_descriptor = Descriptor::new_with_error(
            handlers.general_protection_fault_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[13] = general_protection_fault_descriptor;

        let page_fault_descriptor =
            Descriptor::new_with_error(handlers.page_fault_handler, selector, 0, Gate::Interrupt);
        descriptors[14] = page_fault_descriptor;

        let x87_floating_point_descriptor = Descriptor::new(
            handlers.x87_floating_point_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[16] = x87_floating_point_descriptor;

        let alignment_check_descriptor = Descriptor::new_with_error(
            handlers.alignment_check_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[17] = alignment_check_descriptor;

        let machine_check_descriptor =
            Descriptor::new_with_halt(handlers.machine_check_handler, selector, 0, Gate::Interrupt);
        descriptors[18] = machine_check_descriptor;

        let simd_floating_point_descriptor = Descriptor::new(
            handlers.simd_floating_point_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[19] = simd_floating_point_descriptor;

        let virtualization_descriptor = Descriptor::new(
            handlers.virtualization_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[20] = virtualization_descriptor;

        let control_protection_descriptor = Descriptor::new_with_error(
            handlers.control_protection_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[21] = control_protection_descriptor;

        let hypervisor_injection_descriptor = Descriptor::new(
            handlers.hypervisor_injection_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[28] = hypervisor_injection_descriptor;

        let vmm_communication_descriptor = Descriptor::new_with_error(
            handlers.vmm_communication_handler,
            selector,
            0,
            Gate::Interrupt,
        );
        descriptors[29] = vmm_communication_descriptor;

        let security_descriptor =
            Descriptor::new_with_error(handlers.security_handler, selector, 0, Gate::Interrupt);
        descriptors[30] = security_descriptor;

        Self { descriptors }
    }

    fn base(&self) -> u64 {
        self.descriptors.as_ptr() as u64
    }

    fn limit(&self) -> u16 {
        u16::try_from(size_of_val(&self.descriptors) - 1).expect("Failed to calculate limit.")
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
