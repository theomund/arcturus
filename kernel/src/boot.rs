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

use bootloader::limine::info::Request;
use bootloader::limine::marker::{RequestsEnd, RequestsStart};
use bootloader::limine::revision::Base;
use utility::info;

#[used]
#[unsafe(link_section = ".limine_requests_start")]
static START_MARKER: RequestsStart = RequestsStart::new();

#[used]
#[unsafe(link_section = ".limine_requests_end")]
static END_MARKER: RequestsEnd = RequestsEnd::new();

#[used]
#[unsafe(link_section = ".limine_requests")]
static BASE_REVISION: Base = Base::new(3);

#[used]
#[unsafe(link_section = ".limine_requests")]
static INFO_REQUEST: Request = Request::new();

pub fn init() {
    assert!(BASE_REVISION.is_supported());
    assert!(BASE_REVISION.is_valid());
    assert_eq!(BASE_REVISION.loaded(), 3);
    info!("Initialized the boot module.");
}
