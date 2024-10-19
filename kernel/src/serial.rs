// Arcturus - Hobbyist operating system written in Rust.
// Copyright (C) 2024 Theomund
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

use crate::io::{inb, outb};

enum Ports {
    COM1 = 0x3F8,
}

struct Port {
    address: u16,
}

impl Write for Port {
    fn write_char(&mut self, c: char) -> Result {
        if c == '\n' {
            self.write('\r');
        }
        self.write(c);
        Ok(())
    }

    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            self.write_char(c)?;
        }
        Ok(())
    }

    fn write_fmt(&mut self, args: Arguments<'_>) -> Result {
        let string = args.as_str().expect("Failed to get arguments.");
        self.write_str(string)?;
        Ok(())
    }
}

impl Port {
    fn new(port: Ports) -> Port {
        let address = port as u16;

        outb(address + 1, 0x00);
        outb(address + 3, 0x80);
        outb(address, 0x03);
        outb(address + 1, 0x00);
        outb(address + 3, 0x03);
        outb(address + 2, 0xC7);
        outb(address + 4, 0x0B);
        outb(address + 4, 0x1E);
        outb(address, 0xAE);

        assert_eq!(inb(address), 0xAE);

        outb(address + 4, 0x0F);

        Port { address }
    }

    fn received(&self) -> bool {
        inb(self.address + 5) & 1 != 0
    }

    fn read(&self) -> u8 {
        while !self.received() {}
        inb(self.address)
    }

    fn transmit_empty(&self) -> bool {
        inb(self.address + 5) & 0x20 != 0
    }

    fn write(&self, character: char) {
        while !self.transmit_empty() {}
        outb(self.address, character as u8);
    }
}

pub fn init() -> Result {
    let mut port = Port::new(Ports::COM1);
    writeln!(port, "Arcturus v0.1.0 (x86_64)")?;
    writeln!(port, "Copyright (C) 2024 Theomund\n")?;
    write!(port, "[root@localhost ~]$ ")?;
    port.read();
    Ok(())
}
