# Arcturus - Hobbyist operating system written in Rust.
# Copyright (C) 2025 Theomund
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

# Run all CI/CD stages.
all: lint test build

# Build the project source code.
build: iso

# Clean the project source tree.
clean:
    cargo clean

# Run the Rust linter.
clippy:
    cargo clippy

# Format the project source code.
format:
    cargo fmt

# Build the ISO image.
iso: kernel
    mkdir -p target/iso/root/boot
    cp -v target/x86_64-unknown-none/debug/kernel target/iso/root/boot/
    mkdir -p target/iso/root/boot/limine
    cp -v bootloader/src/limine.conf /usr/share/limine/{limine-bios.sys,limine-{bios-cd,uefi-cd}.bin} target/iso/root/boot/limine/
    mkdir -p target/iso/root/EFI/BOOT
    cp -v /usr/share/limine/BOOT{X64,IA32}.EFI target/iso/root/EFI/BOOT/
    xorriso -as mkisofs -R -r -J -b boot/limine/limine-bios-cd.bin \
            -no-emul-boot -boot-load-size 4 -boot-info-table -hfsplus \
            -apm-block-size 2048 --efi-boot boot/limine/limine-uefi-cd.bin \
            -efi-boot-part --efi-boot-image --protective-msdos-label \
            target/iso/root -o target/iso/arcturus.iso
    limine bios-install target/iso/arcturus.iso

# Build the kernel.
kernel:
    cargo build -p kernel

# Run the project linters.
lint: clippy vale yamllint

# Create a QEMU virtual machine with BIOS firmware.
run-bios: iso
    qemu-system-x86_64 -M q35 \
                       -m 2G \
                       -cdrom target/iso/arcturus.iso \
                       -boot d

# Create a QEMU virtual machine with UEFI firmware.
run-uefi: iso
    qemu-system-x86_64 -M q35 \
                       -m 2G \
                       -drive if=pflash,unit=0,format=raw,file=/usr/share/edk2/ovmf/OVMF_CODE.fd,readonly=on \
                       -cdrom target/iso/arcturus.iso

# Run the project test suite.
test:
    cargo test

# Run the prose linter.
vale:
    vale sync
    vale README.md

# Run the YAML linter.
yamllint:
    yamllint .github/workflows
