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

use bitflags::bitflags;

use crate::cpu::{instructions, registers, registers::Segment};

#[repr(transparent)]
struct Entry(u64);

impl From<Descriptor> for Entry {
    fn from(descriptor: Descriptor) -> Self {
        match descriptor {
            Descriptor::Segment(value) => Entry(value),
        }
    }
}

enum Descriptor {
    Segment(u64),
}

impl Descriptor {
    pub fn null_segment() -> Descriptor {
        Descriptor::Segment(DescriptorFlags::NULL.bits())
    }

    pub fn kernel_code_segment() -> Descriptor {
        Descriptor::Segment(DescriptorFlags::KERNEL_CODE.bits())
    }

    pub fn kernel_data_segment() -> Descriptor {
        Descriptor::Segment(DescriptorFlags::KERNEL_DATA.bits())
    }

    pub fn user_code_segment() -> Descriptor {
        Descriptor::Segment(DescriptorFlags::USER_CODE.bits())
    }

    pub fn user_data_segment() -> Descriptor {
        Descriptor::Segment(DescriptorFlags::USER_DATA.bits())
    }
}

bitflags! {
    struct DescriptorFlags: u64 {
        const LIMIT_LOW = 0xFFFF;
        const BASE_LOW = 0xFF_FFFF << 16;
        const ACCESSED = 1 << 40;
        const READ_WRITE = 1 << 41;
        const DIRECTION_CONFORMING = 1 << 42;
        const EXECUTABLE = 1 << 43;
        const TYPE = 1 << 44;
        const PRIVILEGE_LEVEL = 3 << 45;
        const PRESENT = 1 << 47;
        const LIMIT_HIGH = 0xF << 48;
        const RESERVED = 1 << 52;
        const LONG_MODE = 1 << 53;
        const SIZE = 1 << 54;
        const GRANULARITY = 1 << 55;
        const BASE_HIGH = 0xFF << 56;
    }
}

impl DescriptorFlags {
    const COMMON: Self = Self::from_bits_truncate(
        Self::LIMIT_LOW.bits()
            | Self::ACCESSED.bits()
            | Self::READ_WRITE.bits()
            | Self::TYPE.bits()
            | Self::PRESENT.bits()
            | Self::LIMIT_HIGH.bits()
            | Self::GRANULARITY.bits(),
    );

    const NULL: Self = Self::empty();

    const KERNEL_CODE: Self = Self::from_bits_truncate(
        Self::COMMON.bits() | Self::EXECUTABLE.bits() | Self::LONG_MODE.bits(),
    );

    const KERNEL_DATA: Self = Self::from_bits_truncate(Self::COMMON.bits() | Self::SIZE.bits());

    const USER_CODE: Self =
        Self::from_bits_truncate(Self::KERNEL_CODE.bits() | Self::PRIVILEGE_LEVEL.bits());

    const USER_DATA: Self =
        Self::from_bits_truncate(Self::KERNEL_DATA.bits() | Self::PRIVILEGE_LEVEL.bits());
}

#[repr(C, packed)]
pub struct DescriptorTablePointer {
    limit: u16,
    base: u64,
}

struct GlobalDescriptorTable {
    table: [Entry; 5],
}

impl GlobalDescriptorTable {
    fn new() -> Self {
        let table = [
            Descriptor::null_segment().into(),
            Descriptor::kernel_code_segment().into(),
            Descriptor::kernel_data_segment().into(),
            Descriptor::user_code_segment().into(),
            Descriptor::user_data_segment().into(),
        ];
        Self { table }
    }

    fn limit(&self) -> u16 {
        u16::try_from(self.table.len() * size_of::<u64>() - 1)
            .expect("Failed to calculate GDT limit.")
    }

    fn load(&self) {
        let pointer = &self.pointer();
        instructions::lgdt(pointer);
        registers::CS::set(0x08);
        registers::DS::set(0x10);
        registers::ES::set(0x10);
        registers::FS::set(0x10);
        registers::GS::set(0x10);
        registers::SS::set(0x10);
    }

    fn pointer(&self) -> DescriptorTablePointer {
        DescriptorTablePointer {
            limit: self.limit(),
            base: self.table.as_ptr() as u64,
        }
    }
}

pub fn init() {
    let gdt = GlobalDescriptorTable::new();
    gdt.load();
}
