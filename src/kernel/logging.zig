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

const serial = @import("serial.zig");
const std = @import("std");

pub fn log(comptime level: std.log.Level, comptime scope: @Type(.EnumLiteral), comptime format: []const u8, args: anytype) void {
    const writer = serial.port.writer();
    const color = switch (level) {
        std.log.Level.debug => std.io.tty.Color.bright_green,
        std.log.Level.err => std.io.tty.Color.bright_red,
        std.log.Level.info => std.io.tty.Color.bright_blue,
        std.log.Level.warn => std.io.tty.Color.bright_yellow,
    };
    const config: std.io.tty.Config = .escape_codes;
    std.io.tty.Config.setColor(config, writer, color) catch unreachable;
    const message = comptime "[" ++ level.asText() ++ "] " ++ @tagName(scope) ++ ": " ++ format ++ "\n";
    try std.fmt.format(writer, message, args);
}
