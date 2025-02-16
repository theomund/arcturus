FROM quay.io/fedora/fedora:41
RUN dnf copr enable -y mczernek/vale 
RUN dnf install -y \
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
