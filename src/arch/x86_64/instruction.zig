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

pub fn hlt() void {
    asm volatile ("hlt");
}

pub fn inb(port: u16) u8 {
    return asm volatile ("inb %[port], %[value]"
        : [value] "={al}" (-> u8),
        : [port] "N{dx}" (port),
    );
}

pub fn lgdt(pointer: gdt.Pointer) void {
    asm volatile ("lgdt (%[pointer])"
        :
        : [pointer] "r" (pointer),
    );
}

pub fn outb(port: u16, value: u8) void {
    asm volatile ("outb %[value], %[port]"
        :
        : [port] "{dx}" (port),
          [value] "{al}" (value),
    );
}
