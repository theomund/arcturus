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
        return asm volatile ("mov %%cs, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u64) void {
        asm volatile (
            \\ pushq %[selector]
            \\ lea 1f(%%rip), %%rax
            \\ pushq %%rax
            \\ lretq
            \\
            \\ 1:
            :
            : [selector] "r" (selector),
        );
    }
};

pub const DS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%ds, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u16) void {
        asm volatile ("mov %[selector], %%ds"
            :
            : [selector] "r" (selector),
        );
    }
};

pub const ES = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%es, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u16) void {
        asm volatile ("mov %[selector], %%es"
            :
            : [selector] "r" (selector),
        );
    }
};

pub const FS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%fs, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u16) void {
        asm volatile ("mov %[selector], %%fs"
            :
            : [selector] "r" (selector),
        );
    }
};

pub const GS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%gs, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u16) void {
        asm volatile ("mov %[selector], %%gs"
            :
            : [selector] "r" (selector),
        );
    }
};

pub const SS = struct {
    pub fn get() u16 {
        return asm volatile ("mov %%ss, %[selector]"
            : [selector] "=r" (-> u16),
        );
    }

    pub fn set(selector: u16) void {
        asm volatile ("mov %[selector], %%ss"
            :
            : [selector] "r" (selector),
        );
    }
};
