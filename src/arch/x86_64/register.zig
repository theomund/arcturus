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

pub const CS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%cs, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%cs"
            :
            : [value] "r" (value),
        );
    }
};

pub const DS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%ds, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%ds"
            :
            : [value] "r" (value),
        );
    }
};

pub const ES = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%es, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%es"
            :
            : [value] "r" (value),
        );
    }
};

pub const FS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%fs, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%fs"
            :
            : [value] "r" (value),
        );
    }
};

pub const GS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%gs, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%gs"
            :
            : [value] "r" (value),
        );
    }
};

pub const SS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%ss, %[value]"
            : [value] "=r" (-> u16),
        );
    }

    pub fn set(value: u16) void {
        asm volatile ("mov %[value], %%ss"
            :
            : [value] "r" (value),
        );
    }
};
