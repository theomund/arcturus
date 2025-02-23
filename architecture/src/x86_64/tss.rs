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

use super::gdt;
use super::instruction;

#[repr(C, packed(4))]
pub struct Segment {
    reserved_1: u32,
    privilege_stack_table: [u64; 3],
    reserved_2: u64,
    interrupt_stack_table: [u64; 7],
    reserved_3: u64,
    reserved_4: u16,
    io_map_base: u16,
}

impl Default for Segment {
    fn default() -> Self {
        Self::new()
    }
}

impl Segment {
    #[must_use]
    pub fn new() -> Self {
        Self {
            reserved_1: 0,
            privilege_stack_table: [0; 3],
            reserved_2: 0,
            interrupt_stack_table: [0; 7],
            reserved_3: 0,
            reserved_4: 0,
            io_map_base: u16::try_from(size_of::<Self>()).expect("Failed to calculate base."),
        }
    }

    pub fn load(selector: gdt::Selector) {
        instruction::ltr(selector);
    }
}

#[cfg(test)]
mod tests {
    use super::Segment;

    #[test]
    fn test_io_map_base() {
        let segment = Segment::new();
        assert_eq!(segment.io_map_base, 0x0068);
    }
}
