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
const std = @import("std");

const Context = Port;
const ReadError = error{};
const WriteError = error{};

const Logger = std.log.scoped(.serial);

const Ports = enum(u16) {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
};

const Port = struct {
    address: u16,

    const Reader = std.io.GenericReader(Context, ReadError, parse);
    const Writer = std.io.GenericWriter(Context, WriteError, append);

    fn init(address: u16) Port {
        arch.instruction.outb(address + 1, 0x00);
        arch.instruction.outb(address + 3, 0x80);
        arch.instruction.outb(address + 0, 0x03);
        arch.instruction.outb(address + 1, 0x00);
        arch.instruction.outb(address + 3, 0x03);
        arch.instruction.outb(address + 2, 0xC7);
        arch.instruction.outb(address + 4, 0x0B);
        arch.instruction.outb(address + 4, 0x1E);
        arch.instruction.outb(address + 0, 0xAE);

        if (arch.instruction.inb(address + 0) != 0xAE) {
            std.debug.panic("Failed to initialize the serial port.", .{});
        }

        arch.instruction.outb(address + 4, 0x0F);

        return Port{
            .address = address,
        };
    }

    fn received(self: Port) bool {
        return (arch.instruction.inb(self.address + 5) & 1) != 0;
    }

    fn read(self: Port) u8 {
        while (!self.received()) {}

        return arch.instruction.inb(self.address);
    }

    fn empty(self: Port) bool {
        return (arch.instruction.inb(self.address + 5) & 0x20) != 0;
    }

    fn write(self: Port, value: u8) void {
        while (!self.empty()) {}

        arch.instruction.outb(self.address, value);
    }

    fn append(context: Context, bytes: []const u8) WriteError!usize {
        for (bytes) |byte| {
            if (byte == '\n') {
                context.write('\r');
            }
            context.write(byte);
        }
        return bytes.len;
    }

    pub fn writer(self: Port) Writer {
        return .{ .context = self };
    }

    fn parse(context: Context, buffer: []u8) ReadError!usize {
        var i: u8 = 0;
        while (i < buffer.len) : (i += 1) {
            buffer[i] = context.read();
            if (buffer[i] == '\r') {
                break;
            }
        }
        return i;
    }

    pub fn reader(self: Port) Reader {
        return .{ .context = self };
    }
};

pub var COM1: Port = undefined;

pub fn init() !void {
    COM1 = Port.init(@intFromEnum(Ports.COM1));
    const writer = COM1.writer();
    try writer.print("Arcturus v0.1.0 (x86_64)\n", .{});
    try writer.print("Copyright (C) 2025 Theomund\n\n", .{});
    Logger.info("Initialized serial port driver.", .{});
}
