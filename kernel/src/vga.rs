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

use bootloader::limine::framebuffer::{Framebuffer, Request};
use utility::info;

#[used]
#[unsafe(link_section = ".limine_requests")]
static FRAMEBUFFER_REQUEST: Request = Request::new();

pub fn draw_line(framebuffer: &Framebuffer) {
    for i in 0..100 {
        let pixel_offset = usize::try_from(i * framebuffer.pitch() + i * 4).unwrap();
        let address = framebuffer.address().wrapping_add(pixel_offset);
        unsafe {
            address.cast::<u32>().write_unaligned(0xFFFF_FFFF);
        }
    }
}

pub fn init() {
    let response = FRAMEBUFFER_REQUEST.response().unwrap();
    assert_eq!(response.revision(), 1);
    assert_eq!(response.framebuffer_count(), 1);

    let framebuffer = response.framebuffers().next().unwrap();
    assert_eq!(framebuffer.width(), 1280);
    assert_eq!(framebuffer.height(), 800);
    assert_eq!(framebuffer.pitch(), 5120);

    draw_line(&framebuffer);

    info!("Initialized the VGA framebuffer driver.");
}
