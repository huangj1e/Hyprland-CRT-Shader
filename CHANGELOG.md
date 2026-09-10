# Changelog

All notable changes to this project will be documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases use [Semantic Versioning](https://semver.org/).

## [Unreleased]

## [1.1.0] - 2026-09-10

### Added

- Native Rust + Slint control panel for tuning shader parameters with 180 ms debounced Hyprland recompilation.
- Persistent per-user shader copy so package-managed files remain untouched.
- Chinese, English, Japanese, and Korean interface languages.
- Omarchy theme color integration with automatic theme refresh.
- `Ctrl+R` reset and `Ctrl+Shift+E` emergency-disable shortcuts.
- Detailed source-build, deployment, Arch packaging, compatibility, and performance documentation.

### Changed

- Added themed controls and compact grouped parameter layout with consistent padding and spacing.
- Added desktop application integration through `hypr-crt-control`.

### Fixed

- Prevented slider tracks from overlapping their numeric value columns.
- Removed repeated parameter descriptions and excessive empty space between parameter groups.

## [1.0.0] - 2026-08-29

### Added

- Single-pass CRT and analog-TV screen shader for Hyprland.
- Barrel curvature, scanlines, RGB phosphor mask, chromatic aberration, vignette, flicker, and analog noise.
- Periodic signal glitches, block noise, horizontal sync waves, and a rolling vertical-sync tear.
- Resolution-aware physical-pixel parameters through `fullSize`.
- Lua and legacy Hyprland configuration examples.
- Native Rust + Slint graphical control panel with application-menu integration.
- Arch Linux `PKGBUILD` and pacman package workflow.

[Unreleased]: https://github.com/huangj1e/Hyprland-CRT-Shader/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/releases/tag/v1.0.0
