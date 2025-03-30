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

use core::ptr;

#[repr(C)]
pub struct Request {
    id: [u64; 4],
    revision: u64,
    response: *mut Response,
}

impl Default for Request {
    fn default() -> Self {
        Self::new()
    }
}

impl Request {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: [
                0xc7b1_dd30_df4c_8b88,
                0x0a82_e883_a194_f07b,
                0xf550_38d8_e2a1_202f,
                0x2794_26fc_f5f5_9740,
            ],
            revision: 0,
            response: ptr::null_mut(),
        }
    }
}

unsafe impl Send for Request {}
unsafe impl Sync for Request {}

#[repr(C)]
struct Response {
    revision: u64,
    name: *const u8,
    version: *const u8,
}
