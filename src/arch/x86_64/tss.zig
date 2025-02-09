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

const TaskStateSegment = packed struct {
    reserved_low: u32,
    privilege_stack_table: 3[u64],
    reserved_middle: u64,
    interrupt_stack_table: 7[u64],
    reserved_high: u80,
    io_map_base: u16,

    pub fn init() TaskStateSegment {
        return TaskStateSegment{
            .reserved_low = 0,
            .privilege_stack_table = [_]u8{0} ** 3,
            .reserved_middle = 0,
            .interrupt_stack_table = [_]u8{0} ** 7,
            .reserved_high = 0,
            .io_map_base = @sizeOf(TaskStateSegment),
        };
    }

    pub fn load(selector: u16) void {
        instruction.ltr(selector);
    }
};
