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

#[repr(transparent)]
pub struct RequestsStart {
    id: [c_ulong; 4],
}

impl Default for RequestsStart {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestsStart {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: [
                0xf6b8_f4b3_9de7_d1ae,
                0xfab9_1a69_40fc_b9cf,
                0x785c_6ed0_15d3_e316,
                0x181e_920a_7852_b9d9,
            ],
        }
    }
}

#[repr(transparent)]
pub struct RequestsEnd {
    id: [c_ulong; 2],
}

impl Default for RequestsEnd {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestsEnd {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: [0xadc0_e053_1bb1_0d03, 0x9572_709f_3176_4c62],
        }
    }
}
