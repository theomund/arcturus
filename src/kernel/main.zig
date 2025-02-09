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
const logging = @import("logging.zig");
const serial = @import("serial.zig");
const std = @import("std");
const tss = @import("tss.zig");

const Logger = std.log.scoped(.kernel);

pub const std_options = std.Options{
    .log_level = .debug,
    .logFn = logging.log,
};

export fn _start() callconv(.C) noreturn {
    serial.init() catch |err| {
        std.debug.panic("Failed to initialize the serial port driver: {}", .{err});
    };

    gdt.init();

    tss.init();

    Logger.info("Successfully initialized the operating system.", .{});

    done();
}

pub fn panic(message: []const u8, trace: ?*std.builtin.StackTrace, address: ?usize) noreturn {
    _ = message;
    _ = trace;
    _ = address;
    done();
}

inline fn done() noreturn {
    arch.instruction.cli();
    while (true) {
        arch.instruction.hlt();
    }
}
