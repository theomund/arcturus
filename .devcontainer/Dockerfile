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

FROM quay.io/fedora/fedora:41@sha256:991a06b2425c13613ef8ace721a9055e52a64f65cd96c2b18c72bde43fe1308b
RUN dnf copr enable -y mczernek/vale
RUN dnf install -y \
    bash-completion-2.13 \
    git-2.48.1 \
    qemu-system-x86-9.1.2 \
    vale-3.9.3 \
    yamllint-1.35.1 \
    zig-0.13.0 \
    && dnf clean all
WORKDIR /usr/local/bin
RUN curl -L https://builds.zigtools.org/zls-linux-x86_64-0.13.0.tar.xz | tar -xJ zls
RUN useradd -m developer
USER developer
WORKDIR /home/developer
