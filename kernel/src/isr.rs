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

use architecture::x86_64::idt::Frame;
use utility::{debug, error, warn};

use crate::done;

pub extern "x86-interrupt" fn division_error_handler(frame: Frame) {
    warn!("Handled the division error exception.");
}

pub extern "x86-interrupt" fn debug_handler(frame: Frame) {
    debug!("Handled the debug trap.");
}

pub extern "x86-interrupt" fn non_maskable_interrupt_handler(frame: Frame) {
    warn!("Handled the non-maskable interrupt exception.");
}

pub extern "x86-interrupt" fn breakpoint_handler(frame: Frame) {
    debug!("Handled the breakpoint trap.");
}

pub extern "x86-interrupt" fn overflow_handler(frame: Frame) {
    warn!("Handled the overflow trap.");
}

pub extern "x86-interrupt" fn bound_range_exceeded_handler(frame: Frame) {
    warn!("Handled the bound range exceeded exception.");
}

pub extern "x86-interrupt" fn invalid_opcode_handler(frame: Frame) {
    warn!("Handled the invalid opcode exception.");
}

pub extern "x86-interrupt" fn device_not_available_handler(frame: Frame) {
    warn!("Handled the device not available exception.");
}

pub extern "x86-interrupt" fn double_fault_handler(frame: Frame, code: u64) -> ! {
    error!("Handled the double fault exception.");
    done();
}

pub extern "x86-interrupt" fn invalid_tss_handler(frame: Frame, code: u64) {
    warn!("Handled the invalid TSS exception.");
}

pub extern "x86-interrupt" fn segment_not_present_handler(frame: Frame, code: u64) {
    warn!("Handled the segment not present exception.");
}

pub extern "x86-interrupt" fn stack_segment_fault_handler(frame: Frame, code: u64) {
    warn!("Handled the stack segment fault exception.");
}

pub extern "x86-interrupt" fn general_protection_fault_handler(frame: Frame, code: u64) {
    warn!("Handled the general protection fault exception.");
}

pub extern "x86-interrupt" fn page_fault_handler(frame: Frame, code: u64) {
    warn!("Handled the page fault exception.");
}

pub extern "x86-interrupt" fn x87_floating_point_handler(frame: Frame) {
    warn!("Handled the x87 floating-point exception.");
}

pub extern "x86-interrupt" fn alignment_check_handler(frame: Frame, code: u64) {
    warn!("Handled the alignment check exception.");
}

pub extern "x86-interrupt" fn machine_check_handler(frame: Frame) -> ! {
    error!("Handled the machine check exception.");
    done();
}

pub extern "x86-interrupt" fn simd_floating_point_handler(frame: Frame) {
    warn!("Handled the SIMD floating-point exception.");
}

pub extern "x86-interrupt" fn virtualization_handler(frame: Frame) {
    warn!("Handled the virtualization exception.");
}

pub extern "x86-interrupt" fn control_protection_handler(frame: Frame, code: u64) {
    warn!("Handled the control protection exception.");
}

pub extern "x86-interrupt" fn hypervisor_injection_handler(frame: Frame) {
    warn!("Handled the hypervisor injection exception.");
}

pub extern "x86-interrupt" fn vmm_communication_handler(frame: Frame, code: u64) {
    warn!("Handled the VMM communication exception.");
}

pub extern "x86-interrupt" fn security_handler(frame: Frame, code: u64) {
    warn!("Handled the security exception.");
}
