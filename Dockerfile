# Build the Arch Linux package in a reproducible Linux environment.
# The package is currently published for x86_64, so keep the build platform
# explicit when this is invoked on an ARM workstation (for example, Apple
# Silicon via Docker's emulation support).
FROM archlinux:base-devel AS builder

RUN pacman -Syu --noconfirm \
        cargo \
        cmake \
        fontconfig \
        glslang \
        libxkbcommon \
        ninja \
        wayland \
    && pacman -Scc --noconfirm

RUN useradd --create-home builder

WORKDIR /src
COPY --chown=builder:builder . .
RUN chown builder:builder /src

# makepkg must run as a non-root user. --nodeps is intentional: the runtime
# dependencies are declared in packaging/arch/PKGBUILD and were installed
# above; skipping host package checks also makes this work with Docker's
# package database consistently.
RUN runuser -u builder -- \
    ./scripts/build-arch-package.sh --noconfirm --nodeps

# An output stage lets callers export dist/ directly with BuildKit:
# docker build --output type=local,dest=dist .
FROM scratch AS artifact
COPY --from=builder /src/dist/ /
