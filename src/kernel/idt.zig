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

const arch = @import("arch");
const gdt = @import("gdt.zig");
const std = @import("std");

const Logger = std.log.scoped(.idt);

pub var table: arch.idt.Table = undefined;

pub fn init() void {
    const selector = gdt.table.selectors[1];
    table = arch.idt.Table.init(selector);
    table.load();
    Logger.info("Initialized the interrupt descriptor table.", .{});
}
