# Arcturus - Hobbyist operating system written in Rust.
# Copyright (C) 2024 Theomund
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

SHELL := /bin/sh

BOOT_BIN := $(addprefix /usr/share/limine/,limine-bios.sys limine-bios-cd.bin limine-uefi-cd.bin)
BOOT_CONFIG := bootloader/limine.conf
BOOT_EFI := $(addprefix /usr/share/limine/,BOOTIA32.EFI BOOTX64.EFI)
IMAGE := target/arcturus.iso
IMAGE_ROOT := target/iso_root
KERNEL := target/x86_64-unknown-none/debug/kernel
OVMF := /usr/share/edk2/ovmf/OVMF_CODE.fd

$(IMAGE): $(BOOT_BIN) $(BOOT_CONFIG) $(BOOT_EFI) $(KERNEL)
	rm -rf $(IMAGE_ROOT)
	mkdir -p $(IMAGE_ROOT)/boot/
	cp -v $(KERNEL) $(IMAGE_ROOT)/boot/
	mkdir -p $(IMAGE_ROOT)/boot/limine
	cp -v $(BOOT_BIN) $(BOOT_CONFIG) $(IMAGE_ROOT)/boot/limine/
	mkdir -p $(IMAGE_ROOT)/EFI/BOOT/
	cp -v $(BOOT_EFI) $(IMAGE_ROOT)/EFI/BOOT/
	xorriso -as mkisofs -b boot/limine/limine-bios-cd.bin \
			-no-emul-boot -boot-load-size 4 -boot-info-table \
			--efi-boot boot/limine/limine-uefi-cd.bin \
			--efi-boot-part --efi-boot-image --protective-msdos-label \
			$(IMAGE_ROOT) -o $(IMAGE)
	rm -rf $(IMAGE_ROOT)

$(KERNEL):
	cargo build -p kernel

.PHONY: all
all: build

.PHONY: build
build: $(IMAGE)

.PHONY: clean
clean:
	cargo clean

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: run-bios
run-bios: build
	qemu-system-x86_64 -M q35 -m 2G -cdrom $(IMAGE)

.PHONY: run-uefi
run-uefi: $(OVMF) build
	qemu-system-x86_64 -M q35 -m 2G -drive if=pflash,unit=0,format=raw,file=$(OVMF),readonly=on -cdrom $(IMAGE)

.PHONY: test
test:
	cargo test
