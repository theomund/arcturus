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

use core::{ffi::c_void, ptr, slice};

#[repr(C)]
pub struct Framebuffer {
    address: *mut c_void,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
    unused: [u8; 7],
    edid_size: u64,
    edid: *const c_void,
    mode_count: u64,
    modes: *const *const Mode,
}

impl Framebuffer {
    #[must_use]
    pub fn address(&self) -> *mut u8 {
        self.address.cast()
    }

    #[must_use]
    pub fn width(&self) -> u64 {
        self.width
    }

    #[must_use]
    pub fn height(&self) -> u64 {
        self.height
    }

    #[must_use]
    pub fn pitch(&self) -> u64 {
        self.pitch
    }
}

#[repr(C)]
pub struct Mode {
    pitch: u64,
    width: u64,
    height: u64,
    bpp: u64,
    memory_model: u8,
    red_mask_size: u8,
    red_mask_shift: u8,
    green_mask_size: u8,
    green_mask_shift: u8,
    blue_mask_size: u8,
    blue_mask_shift: u8,
}

#[repr(C)]
pub struct Request {
    id: [u64; 4],
    revision: u64,
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
                0x9d58_27dc_d881_dd75,
                0xa314_8604_f6fa_b11b,
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
    revision: u64,
    framebuffer_count: u64,
    framebuffers: *const *const Framebuffer,
}

impl Response {
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision
    }

    #[must_use]
    pub fn framebuffer_count(&self) -> u64 {
        self.framebuffer_count
    }

    pub fn framebuffers(&self) -> impl Iterator<Item = Framebuffer> {
        let length = usize::try_from(self.framebuffer_count).unwrap();
        unsafe {
            slice::from_raw_parts(self.framebuffers, length)
                .iter()
                .map(|&x| x.read_volatile())
        }
    }
}
