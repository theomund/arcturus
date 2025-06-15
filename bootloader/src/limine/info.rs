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

use core::{
    ffi::{CStr, c_char, c_ulong},
    ptr,
};

#[repr(C)]
pub struct Request {
    id: [c_ulong; 4],
    revision: c_ulong,
    response: *const Response,
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
            response: ptr::null(),
        }
    }

    #[must_use]
    pub fn response(&self) -> Option<Response> {
        if self.response.is_null() {
            None
        } else {
            unsafe {
                let response = self.response.read_volatile();
                Some(response)
            }
        }
    }
}

unsafe impl Send for Request {}
unsafe impl Sync for Request {}

#[repr(C)]
pub struct Response {
    revision: c_ulong,
    name: *const c_char,
    version: *const c_char,
}

impl Response {
    #[must_use]
    pub fn revision(&self) -> c_ulong {
        self.revision
    }

    #[must_use]
    pub fn name(&self) -> &str {
        unsafe { CStr::from_ptr(self.name).to_str().unwrap() }
    }

    #[must_use]
    pub fn version(&self) -> &str {
        unsafe { CStr::from_ptr(self.version).to_str().unwrap() }
    }
}
