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

use architecture::x86_64::idt::{Handlers, Table};
use core::cell::LazyCell;
use utility::info;
use utility::lock::Spinlock;

use crate::gdt::GDT;
use crate::isr::{
    alignment_check_handler, bound_range_exceeded_handler, breakpoint_handler,
    control_protection_handler, debug_handler, device_not_available_handler,
    division_error_handler, double_fault_handler, general_protection_fault_handler,
    hypervisor_injection_handler, invalid_opcode_handler, invalid_tss_handler,
    machine_check_handler, non_maskable_interrupt_handler, overflow_handler, page_fault_handler,
    security_handler, segment_not_present_handler, simd_floating_point_handler,
    stack_segment_fault_handler, virtualization_handler, vmm_communication_handler,
    x87_floating_point_handler,
};

pub static IDT: Spinlock<LazyCell<Table>> = Spinlock::new(LazyCell::new(|| {
    let handlers = Handlers {
        division_error_handler,
        debug_handler,
        non_maskable_interrupt_handler,
        breakpoint_handler,
        overflow_handler,
        bound_range_exceeded_handler,
        invalid_opcode_handler,
        device_not_available_handler,
        double_fault_handler,
        invalid_tss_handler,
        segment_not_present_handler,
        stack_segment_fault_handler,
        general_protection_fault_handler,
        page_fault_handler,
        x87_floating_point_handler,
        alignment_check_handler,
        machine_check_handler,
        simd_floating_point_handler,
        virtualization_handler,
        control_protection_handler,
        hypervisor_injection_handler,
        vmm_communication_handler,
        security_handler,
    };

    Table::new(&handlers, GDT.lock().selector(1))
}));

pub fn init() {
    IDT.lock().load();
    info!("Initialized the interrupt descriptor table.");
}
