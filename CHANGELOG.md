# Changelog

All notable changes to this project will be documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases use [Semantic Versioning](https://semver.org/).

## [Unreleased]

### Added

- Live Tk control panel for tuning shader parameters with debounced Hyprland recompilation.
- Persistent per-user shader copy so pacman-managed files remain untouched.

## [1.0.0] - 2026-08-29

### Added

- Single-pass CRT and analog-TV screen shader for Hyprland.
- Barrel curvature, scanlines, RGB phosphor mask, chromatic aberration, vignette, flicker, and analog noise.
- Periodic signal glitches, block noise, horizontal sync waves, and a rolling vertical-sync tear.
- Resolution-aware physical-pixel parameters through `fullSize`.
- Lua and legacy Hyprland configuration examples.
- Temporary `hypr-crt-toggle` command with power-saving state restoration.
- Arch Linux `PKGBUILD` and pacman package workflow.

[Unreleased]: https://github.com/huangj1e/Hyprland-CRT-Shader/compare/v1.0.0...HEAD
[1.0.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/releases/tag/v1.0.0
