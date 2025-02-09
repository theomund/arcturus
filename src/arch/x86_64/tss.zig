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

pub const Segment = packed struct {
    reserved_low: u32,
    rsp_0: u64,
    rsp_1: u64,
    rsp_2: u64,
    reserved_middle: u64,
    ist_1: u64,
    ist_2: u64,
    ist_3: u64,
    ist_4: u64,
    ist_5: u64,
    ist_6: u64,
    ist_7: u64,
    reserved_high: u80,
    io_map_base: u16,

    pub fn init() Segment {
        return Segment{
            .reserved_low = 0,
            .rsp_0 = 0,
            .rsp_1 = 0,
            .rsp_2 = 0,
            .reserved_middle = 0,
            .ist_1 = 0,
            .ist_2 = 0,
            .ist_3 = 0,
            .ist_4 = 0,
            .ist_5 = 0,
            .ist_6 = 0,
            .ist_7 = 0,
            .reserved_high = 0,
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
