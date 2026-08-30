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
cp "$root/LICENSE" "$build_dir/LICENSE"
cp "$root/shaders/crt.frag" "$build_dir/crt.frag"
cp "$root/bin/hypr-crt-toggle" "$build_dir/hypr-crt-toggle"
cp "$root/bin/hypr-crt-control" "$build_dir/hypr-crt-control"
cp "$root/config/hyprland-crt-shader.lua" "$build_dir/hyprland-crt-shader.lua"
cp "$root/config/hyprland-crt-shader.conf" "$build_dir/hyprland-crt-shader.conf"

(
    cd "$build_dir"
    makepkg --cleanbuild --force "$@"
    makepkg --printsrcinfo > "$root/packaging/arch/.SRCINFO"
)

cp "$build_dir"/*.pkg.tar.* "$dist_dir/"
echo "Packages written to: $dist_dir"
