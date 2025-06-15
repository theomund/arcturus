// Arcturus - Hobbyist operating system written in Rust.
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

use core::ffi::c_ulong;

#[repr(C)]
pub struct Base {
    id: [c_ulong; 2],
    revision: c_ulong,
}

impl Base {
    #[must_use]
    pub const fn new(revision: c_ulong) -> Self {
        Self {
            id: [0xf956_2b2d_5c95_a6c8, 0x6a7b_3849_4453_6bdc],
            revision,
        }
    }

    #[must_use]
    pub fn is_supported(&self) -> bool {
        self.revision == 0
    }

    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.id[1] != 0x6a7b_3849_4453_6bdc
    }

    #[must_use]
    pub fn loaded(&self) -> c_ulong {
        self.id[1]
    }
}
