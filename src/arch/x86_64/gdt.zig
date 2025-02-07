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

const Entry = u64;

const Access = packed struct {
    accessed: u1,
    read_write: u1,
    direction_conforming: u1,
    executable: u1,
    descriptor_type: u1,
    descriptor_privilege: u1,
    present: u1,
};

const Flags = packed struct {
    reserved: u1,
    long_mode: u1,
    size: u1,
    granularity: u1,
};

const Segment = packed struct {
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

pub const Table = struct {
    descriptors: [5]Entry,

    pub fn init() Table {
        const null_segment: Entry = 0;
        const kernel_code_segment: Entry = 0;
        const kernel_data_segment: Entry = 0;
        const user_code_segment: Entry = 0;
        const user_data_segment: Entry = 0;

        return Table{
            .descriptors = .{
                null_segment,
                kernel_code_segment,
                kernel_data_segment,
                user_code_segment,
                user_data_segment,
            },
        };
    }

    pub fn entries(self: Table) [5]Entry {
        return self.descriptors;
    }

    pub fn pointer(self: Table) Pointer {
        return Pointer{
            .limit = (self.descriptors.len * @sizeOf(Entry)) - 1,
            .base = @intFromPtr(&self.descriptors),
        };
    }

    pub fn load(self: Table) void {
        const ptr = self.pointer();
        instruction.lgdt(ptr);
        register.CS.set(0);
        register.DS.set(0);
        register.ES.set(0);
        register.FS.set(0);
        register.GS.set(0);
        register.SS.set(0);
    }
};

test "Pointer Limit" {
    const gdt = Table.init();
    const pointer = gdt.pointer();
    try std.testing.expectEqual(39, pointer.limit);
}

test "Null Segment" {
    const gdt = Table.init();
    const entries = gdt.entries();
    try std.testing.expectEqual(0, entries[0]);
}
