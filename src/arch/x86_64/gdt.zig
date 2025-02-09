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
const register = @import("register.zig");
const std = @import("std");

const Access = packed struct {
    accessed: u1,
    read_write: u1,
    direction_conforming: u1,
    executable: u1,
    descriptor_type: u1,
    descriptor_privilege: u2,
    present: u1,
};

const Flags = packed struct {
    reserved: u1,
    long_mode: u1,
    size: u1,
    granularity: u1,
};

const Descriptor = packed struct {
    limit_low: u16,
    base_low: u24,
    access: Access,
    limit_high: u4,
    flags: Flags,
    base_high: u8,
};

pub const Pointer = packed struct {
    limit: u16,
    base: u64,
};

const Selector = packed struct {
    requested_privilege_level: u2,
    table_indicator: u1,
    index: u13,
};

pub const Table = struct {
    descriptors: [5]u64,
    selectors: [5]u16,

    pub fn init() Table {
        const null_descriptor = Descriptor{
            .limit_low = 0x0,
            .base_low = 0x0,
            .access = .{
                .accessed = 0,
                .read_write = 0,
                .direction_conforming = 0,
                .executable = 0,
                .descriptor_type = 0,
                .descriptor_privilege = 0,
                .present = 0,
            },
            .limit_high = 0x0,
            .flags = .{
                .reserved = 0,
                .long_mode = 0,
                .size = 0,
                .granularity = 0,
            },
            .base_high = 0x0,
        };

        const null_selector = Selector{
            .requested_privilege_level = 0,
            .table_indicator = 0,
            .index = 0,
        };

        const kernel_code_descriptor = Descriptor{
            .limit_low = 0xFFFF,
            .base_low = 0x0,
            .access = .{
                .accessed = 0,
                .read_write = 1,
                .direction_conforming = 0,
                .executable = 1,
                .descriptor_type = 1,
                .descriptor_privilege = 0,
                .present = 1,
            },
            .limit_high = 0xF,
            .flags = .{
                .reserved = 0,
                .long_mode = 1,
                .size = 0,
                .granularity = 1,
            },
            .base_high = 0x0,
        };

        const kernel_code_selector = Selector{
            .requested_privilege_level = 0,
            .table_indicator = 0,
            .index = 1,
        };

        const kernel_data_descriptor = Descriptor{
            .limit_low = 0xFFFF,
            .base_low = 0x0,
            .access = .{
                .accessed = 0,
                .read_write = 1,
                .direction_conforming = 0,
                .executable = 0,
                .descriptor_type = 1,
                .descriptor_privilege = 0,
                .present = 1,
            },
            .limit_high = 0xF,
            .flags = .{
                .reserved = 0,
                .long_mode = 0,
                .size = 1,
                .granularity = 1,
            },
            .base_high = 0x0,
        };

        const kernel_data_selector = Selector{
            .requested_privilege_level = 0,
            .table_indicator = 0,
            .index = 2,
        };

        const user_code_descriptor = Descriptor{
            .limit_low = 0xFFFF,
            .base_low = 0x0,
            .access = .{
                .accessed = 0,
                .read_write = 1,
                .direction_conforming = 0,
                .executable = 1,
                .descriptor_type = 1,
                .descriptor_privilege = 3,
                .present = 1,
            },
            .limit_high = 0xF,
            .flags = .{
                .reserved = 0,
                .long_mode = 1,
                .size = 0,
                .granularity = 1,
            },
            .base_high = 0x0,
        };

        const user_code_selector = Selector{
            .requested_privilege_level = 3,
            .table_indicator = 0,
            .index = 3,
        };

        const user_data_descriptor = Descriptor{
            .limit_low = 0xFFFF,
            .base_low = 0x0,
            .access = .{
                .accessed = 0,
                .read_write = 1,
                .direction_conforming = 0,
                .executable = 0,
                .descriptor_type = 1,
                .descriptor_privilege = 3,
                .present = 1,
            },
            .limit_high = 0xF,
            .flags = .{
                .reserved = 0,
                .long_mode = 0,
                .size = 1,
                .granularity = 1,
            },
            .base_high = 0x0,
        };

        const user_data_selector = Selector{
            .requested_privilege_level = 3,
            .table_indicator = 0,
            .index = 4,
        };

        return Table{
            .descriptors = .{
                @bitCast(null_descriptor),
                @bitCast(kernel_code_descriptor),
                @bitCast(kernel_data_descriptor),
                @bitCast(user_code_descriptor),
                @bitCast(user_data_descriptor),
            },
            .selectors = .{
                @bitCast(null_selector),
                @bitCast(kernel_code_selector),
                @bitCast(kernel_data_selector),
                @bitCast(user_code_selector),
                @bitCast(user_data_selector),
            },
        };
    }

    pub fn entries(self: Table) struct { descriptors: [5]u64, selectors: [5]u16 } {
        return .{
            .descriptors = self.descriptors,
            .selectors = self.selectors,
        };
    }

    pub fn pointer(self: Table) Pointer {
        return Pointer{
            .limit = (self.descriptors.len * @sizeOf(u64)) - 1,
            .base = @intFromPtr(&self.descriptors[0]),
        };
    }

    pub fn load(self: Table) void {
        const ptr = self.pointer();
        instruction.lgdt(&ptr);
        register.CS.set(self.selectors[1]);
        register.DS.set(self.selectors[2]);
        register.ES.set(self.selectors[2]);
        register.FS.set(self.selectors[2]);
        register.GS.set(self.selectors[2]);
        register.SS.set(self.selectors[2]);
    }
};

test "Pointer Limit" {
    const gdt = Table.init();
    const pointer = gdt.pointer();
    try std.testing.expectEqual(39, pointer.limit);
}

test "Null Segment Descriptor" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x0, entries.descriptors[0]);
}

test "Null Segment Selector" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x0, entries.selectors[0]);
}

test "Kernel Code Segment Descriptor" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x00AF9A000000FFFF, entries.descriptors[1]);
}

test "Kernel Code Segment Selector" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x8, entries.selectors[1]);
}

test "Kernel Data Segment Descriptor" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x00CF92000000FFFF, entries.descriptors[2]);
}

test "Kernel Data Segment Selector" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x10, entries.selectors[2]);
}

test "User Code Segment Descriptor" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x00AFFA000000FFFF, entries.descriptors[3]);
}

test "User Code Segment Selector" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x1B, entries.selectors[3]);
}

test "User Data Segment Descriptor" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x00CFF2000000FFFF, entries.descriptors[4]);
}

test "User Data Segment Selector" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0x23, entries.selectors[4]);
}
