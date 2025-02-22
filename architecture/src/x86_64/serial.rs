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

use core::fmt::{Arguments, Result, Write};

use super::instruction;

pub enum Ports {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
    COM5 = 0x5F8,
    COM6 = 0x4F8,
    COM7 = 0x5E8,
    COM8 = 0x4E8,
}

pub struct Port {
    address: u16,
}

impl Write for Port {
    fn write_char(&mut self, character: char) -> Result {
        if character == '\n' {
            self.write('\r');
        }

        self.write(character);

        Ok(())
    }

    fn write_fmt(&mut self, arguments: Arguments<'_>) -> Result {
        if let Some(string) = arguments.as_str() {
            self.write_str(string)?;
        }

        Ok(())
    }

    fn write_str(&mut self, string: &str) -> Result {
        for character in string.chars() {
            self.write_char(character)?;
        }

        Ok(())
    }
}

impl Port {
    #[must_use]
    pub fn new(port: Ports) -> Self {
        let address = port as u16;

        instruction::outb(address + 1, 0x00);
        instruction::outb(address + 3, 0x80);
        instruction::outb(address, 0x03);
        instruction::outb(address + 1, 0x00);
        instruction::outb(address + 3, 0x03);
        instruction::outb(address + 2, 0xC7);
        instruction::outb(address + 4, 0x0B);
        instruction::outb(address + 4, 0x1E);
        instruction::outb(address, 0xAE);

        assert!(
            instruction::inb(address) == 0xAE,
            "Failed to initialize serial port."
        );

        instruction::outb(address + 4, 0x0F);

        Self { address }
    }

    fn received(&self) -> bool {
        (instruction::inb(self.address + 5) & 1) != 0
    }

    fn transmit_empty(&self) -> bool {
        (instruction::inb(self.address + 5) & 0x20) != 0
    }

    #[must_use]
    pub fn read(&self) -> u8 {
        while !self.received() {}

        instruction::inb(self.address)
    }

    pub fn write(&self, character: char) {
        while !self.transmit_empty() {}

        instruction::outb(self.address, character as u8);
    }
}
