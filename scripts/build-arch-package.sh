#!/usr/bin/env bash
set -euo pipefail

root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
build_dir="$root/build/arch"
dist_dir="$root/dist"

if (( EUID == 0 )); then
    echo "error: makepkg must not be run as root" >&2
    echo "run this script as a regular user" >&2
    exit 1
fi

command -v makepkg >/dev/null 2>&1 || {
    echo "error: makepkg is missing (install base-devel on Arch Linux)" >&2
    exit 1
}

rm -rf "$build_dir"
mkdir -p "$build_dir" "$dist_dir"
cp "$root/packaging/arch/PKGBUILD" "$build_dir/PKGBUILD"
tar -C "$root" -czf "$build_dir/project.tar.gz" \
    --transform='s,^,project/,' \
    Cargo.toml Cargo.lock build.rs LICENSE src ui shaders config \
    packaging/hyprland-crt-control.desktop

(
    cd "$build_dir"
    makepkg --cleanbuild --force "$@"
    makepkg --printsrcinfo > "$root/packaging/arch/.SRCINFO"
)

cp "$build_dir"/*.pkg.tar.* "$dist_dir/"
echo "Packages written to: $dist_dir"
