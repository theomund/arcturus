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

use super::instruction;
use super::register;
use super::tss::Segment;

pub struct Table {
    descriptors: [Descriptor; 7],
    selectors: [Selector; 6],
}

#[repr(C, packed(2))]
pub struct Register {
    limit: u16,
    base: u64,
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Descriptor(pub u64);

impl Descriptor {
    fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        let limit_low = u64::from(limit & 0xFFFF);
        let base_low = u64::from(base & 0xFFFF);
        let base_middle = u64::from((base >> 16) & 0xFF);
        let access_low = u64::from(access);
        let limit_high = u64::from((limit >> 16) & 0xF);
        let flags_low = u64::from(flags);
        let base_high = u64::from((base >> 24) & 0xFF);

        let value = limit_low
            | (base_low << 16)
            | (base_middle << 32)
            | (access_low << 40)
            | (limit_high << 48)
            | (flags_low << 52)
            | (base_high << 56);

        Self(value)
    }

    fn new_system(base: u32) -> Self {
        let value = u64::from(base);

        Self(value)
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Selector(pub u16);

impl Selector {
    #[must_use]
    fn new(index: u16, privilege_level: u16) -> Self {
        Self((index << 3) | privilege_level)
    }
}

impl Table {
    #[must_use]
    pub fn new(segment: *const Segment) -> Self {
        let null_descriptor = Descriptor::new(0x0, 0x0, 0x0, 0x0);
        let null_selector = Selector::new(0, 0);

        let kernel_code_descriptor = Descriptor::new(0x0, 0xFFFFF, 0x9A, 0xA);
        let kernel_code_selector = Selector::new(1, 0);

        let kernel_data_descriptor = Descriptor::new(0x0, 0xFFFFF, 0x92, 0xC);
        let kernel_data_selector = Selector::new(2, 0);

        let user_code_descriptor = Descriptor::new(0x0, 0xFFFFF, 0xFA, 0xA);
        let user_code_selector = Selector::new(3, 3);

        let user_data_descriptor = Descriptor::new(0x0, 0xFFFFF, 0xF2, 0xC);
        let user_data_selector = Selector::new(4, 3);

        let task_state_base = segment as u64;
        let task_state_base_low = (task_state_base & 0xFFFF_FFFF) as u32;
        let task_state_base_high = (task_state_base >> 32) as u32;
        let task_state_limit =
            u32::try_from(size_of::<Segment>() - 1).expect("Failed to calculate limit.");

        let task_state_descriptor_low =
            Descriptor::new(task_state_base_low, task_state_limit, 0x89, 0x0);
        let task_state_descriptor_high = Descriptor::new_system(task_state_base_high);
        let task_state_selector = Selector::new(5, 0);

        Self {
            descriptors: [
                null_descriptor,
                kernel_code_descriptor,
                kernel_data_descriptor,
                user_code_descriptor,
                user_data_descriptor,
                task_state_descriptor_low,
                task_state_descriptor_high,
            ],
            selectors: [
                null_selector,
                kernel_code_selector,
                kernel_data_selector,
                user_code_selector,
                user_data_selector,
                task_state_selector,
            ],
        }
    }

    fn base(&self) -> u64 {
        self.descriptors.as_ptr() as u64
    }

    #[must_use]
    pub fn descriptor(&self, index: usize) -> Descriptor {
        self.descriptors[index]
    }

    fn limit(&self) -> u16 {
        (size_of_val(&self.descriptors) - 1) as u16
    }

    #[must_use]
    pub fn register(&self) -> Register {
        Register {
            limit: self.limit(),
            base: self.base(),
        }
    }

    #[must_use]
    pub fn selector(&self, index: usize) -> Selector {
        self.selectors[index]
    }

    pub fn load(&self) {
        instruction::cli();

        let register = self.register();
        instruction::lgdt(&register);

        register::CS::set(self.selector(1));
        register::DS::set(self.selector(2));
        register::ES::set(self.selector(2));
        register::FS::set(self.selector(2));
        register::GS::set(self.selector(2));
        register::SS::set(self.selector(2));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_table() -> Table {
        let segment = Segment::new();
        Table::new(&segment)
    }

    fn get_descriptor(index: usize) -> u64 {
        let table = create_table();
        table.descriptor(index).0
    }

    fn get_selector(index: usize) -> u16 {
        let table = create_table();
        table.selector(index).0
    }

    #[test]
    fn test_null_descriptor() {
        assert_eq!(get_descriptor(0), 0x0000000000000000);
    }

    #[test]
    fn test_null_selector() {
        assert_eq!(get_selector(0), 0x0000);
    }

    #[test]
    fn test_kernel_code_descriptor() {
        assert_eq!(get_descriptor(1), 0x00AF9A000000FFFF);
    }

    #[test]
    fn test_kernel_code_selector() {
        assert_eq!(get_selector(1), 0x0008);
    }

    #[test]
    fn test_kernel_data_descriptor() {
        assert_eq!(get_descriptor(2), 0x00CF92000000FFFF);
    }

    #[test]
    fn test_kernel_data_selector() {
        assert_eq!(get_selector(2), 0x0010);
    }

    #[test]
    fn test_user_code_descriptor() {
        assert_eq!(get_descriptor(3), 0x00AFFA000000FFFF);
    }

    #[test]
    fn test_user_code_selector() {
        assert_eq!(get_selector(3), 0x001B);
    }

    #[test]
    fn test_user_data_descriptor() {
        assert_eq!(get_descriptor(4), 0x00CFF2000000FFFF);
    }

    #[test]
    fn test_user_data_selector() {
        assert_eq!(get_selector(4), 0x0023);
    }

    #[test]
    fn test_task_state_descriptor() {
        assert_ne!(get_descriptor(5), 0x0000000000000000);
        assert_ne!(get_descriptor(6), 0x0000000000000000);
    }

    #[test]
    fn test_task_state_selector() {
        assert_eq!(get_selector(5), 0x0028);
    }
}
