// Arcturus - Hobbyist operating system written in Zig.
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

const instruction = @import("instruction.zig");
const isr = @import("isr.zig");

const Descriptor = packed struct {
    offset_1: u16,
    selector: u16,
    interrupt_stack_table: u3,
    reserved_1: u5,
    gate_type: u4,
    reserved_2: u1,
    desired_privilege_level: u2,
    present: u1,
    offset_2: u48,
    reserved_3: u32,
};

pub const Pointer = extern struct {
    limit: u16 align(1),
    base: u64 align(1),
};

pub const Table = struct {
    descriptors: [256]u128,

    pub fn init(selector: u16) Table {
        const null_descriptor = Descriptor{
            .offset_1 = 0x0,
            .selector = selector,
            .interrupt_stack_table = 0,
            .reserved_1 = 0,
            .gate_type = 0xE,
            .reserved_2 = 0,
            .desired_privilege_level = 0,
            .present = 0,
            .offset_2 = 0x0,
            .reserved_3 = 0,
        };

        const breakpoint_offset = @intFromPtr(&isr.breakpoint_exception);

        const breakpoint_descriptor = Descriptor{
            .offset_1 = @truncate(breakpoint_offset),
            .selector = selector,
            .interrupt_stack_table = 0,
            .reserved_1 = 0,
            .gate_type = 0xF,
            .reserved_2 = 0,
            .desired_privilege_level = 0,
            .present = 1,
            .offset_2 = @truncate(breakpoint_offset >> 16),
            .reserved_3 = 0,
        };

        var descriptor_table = [_]u128{@bitCast(null_descriptor)} ** 256;

        descriptor_table[3] = @bitCast(breakpoint_descriptor);

        return Table{
            .descriptors = descriptor_table,
        };
    }

    fn pointer(self: Table) Pointer {
        return Pointer{
            .limit = (self.descriptors.len * @sizeOf(u128)) - 1,
            .base = @intFromPtr(&self.descriptors[0]),
        };
    }

    pub fn load(self: Table) void {
        const ptr = self.pointer();
        instruction.lidt(&ptr);
        instruction.sti();
        instruction.int3();
    }
};
