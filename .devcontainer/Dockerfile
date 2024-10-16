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

FROM fedora:40
RUN dnf install -y \
    --repofrompath 'terra,https://repos.fyralabs.com/terra$releasever' \
    --setopt='terra.gpgkey=https://repos.fyralabs.com/terra$releasever/key.asc' \
    terra-release-40 \
    && dnf clean all
RUN dnf install -y \
    gcc-14.2.1 \
    gdb-15.1 \
    git-2.47.0 \
    limine-8.1.2 \
    make-4.4.1 \
    qemu-system-x86-8.2.7 \
    && dnf clean all
RUN dnf copr enable -y mczernek/vale \
    && dnf install -y vale-3.7.1 \
    && dnf clean all
RUN useradd -m developer
USER developer
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2024-10-16 --target x86_64-unknown-none
ENV PATH="$PATH:/home/developer/.cargo/bin"
