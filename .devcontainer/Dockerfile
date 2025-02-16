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

FROM quay.io/fedora/fedora:41
RUN dnf copr enable -y mczernek/vale 
RUN dnf install -y \
    gcc-14.2.1 \
    git-2.48.1 \
    just-1.39.0 \
    qemu-system-x86-9.1.2 \
    rustup-1.27.1 \
    vale-3.9.5 \
    xorriso-1.5.6 \
    yamllint-1.35.1 \
    && dnf clean all
RUN useradd -m developer
USER developer
RUN rustup-init -y --default-toolchain nightly-2025-02-16 --target x86_64-unknown-none
