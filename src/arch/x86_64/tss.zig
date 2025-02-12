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

const gdt = @import("gdt.zig");
const instruction = @import("instruction.zig");
const std = @import("std");

const Access = packed struct {
    segment_type: u4,
    descriptor_type: u1,
    descriptor_privilege: u2,
    present: u1,
};

const Low = packed struct {
    limit_low: u16,
    base_low: u24,
    access: Access,
    limit_high: u4,
    flags: gdt.Flags,
    base_middle: u8,
};

const High = packed struct {
    base_high: u32,
    reserved: u32,
};

pub const Descriptor = struct {
    low: Low,
    high: High,
};

pub const Segment = extern struct {
    reserved_1: u32 align(1),
    privilege_stack_table: [3]u64 align(1),
    reserved_2: u64 align(1),
    interrupt_stack_table: [7]u64 align(1),
    reserved_3: u64 align(1),
    reserved_4: u16 align(1),
    io_map_base: u16 align(1),

    pub fn init() Segment {
        return Segment{
            .reserved_1 = 0,
            .privilege_stack_table = [_]u64{0} ** 3,
            .reserved_2 = 0,
            .interrupt_stack_table = [_]u64{0} ** 7,
            .reserved_3 = 0,
            .reserved_4 = 0,
            .io_map_base = @sizeOf(Segment),
        };
    }
};

pub fn load(selector: u16) void {
    instruction.ltr(selector);
}

test "I/O Map Base" {
    const tss = Segment.init();
    try std.testing.expectEqual(0x68, tss.io_map_base);
}
