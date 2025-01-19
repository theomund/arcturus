FROM quay.io/fedora/fedora:41
RUN dnf copr enable -y mczernek/vale
RUN dnf install -y \
    --nogpgcheck \
    --repofrompath 'terra,https://repos.fyralabs.com/terra41' \
    terra-release-41 \
    && dnf clean all
RUN dnf install -y \ 
    git-2.48.1 \
    limine-8.7.0 \
    qemu-system-x86-9.1.2 \
    vale-3.9.1 \
    zig-0.13.0 \
    && dnf clean all
RUN useradd -m developer
USER developer
