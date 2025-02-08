# Arcturus - Hobbyist operating system written in Zig.
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

FROM quay.io/fedora/fedora:41@sha256:d86454521e81cc5a222c7bf2f1910e912c268729739948a605a7c8af18e83c6b
RUN dnf copr enable -y mczernek/vale
RUN dnf install -y \
    bash-completion-2.13 \
    git-2.48.1 \
    lldb-19.1.7 \
    qemu-system-x86-9.1.2 \
    vale-3.9.4 \
    xorriso-1.5.6 \
    yamllint-1.35.1 \
    zig-0.13.0 \
    && dnf clean all
WORKDIR /usr/local/bin
RUN curl -L https://builds.zigtools.org/zls-linux-x86_64-0.13.0.tar.xz | tar -xJ zls
RUN useradd -m developer
USER developer
WORKDIR /home/developer
