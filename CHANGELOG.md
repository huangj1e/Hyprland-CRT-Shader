# Changelog

All notable changes to this project will be documented in this file. The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and releases use [Semantic Versioning](https://semver.org/).

## [1.5.0] - 2026-09-28

### Fixed

- Respect zero-valued shake, wave, sync-tear, and RGB-shift controls without injecting hidden horizontal motion.
- Upgrade stale per-user shader copies when the installed shader implementation changes.
- Fix GLSL vector type mismatch that prevented the shader from compiling.
- Persist control-panel shader values and enabled state across Hyprland reloads, restarts, and login by generating a user configuration include.

### Added

- Independent horizontal and vertical image-scale controls, including automatic migration from the former single overscan value.

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

[1.5.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/compare/v1.1.0...v1.5.0
[1.1.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/huangj1e/Hyprland-CRT-Shader/releases/tag/v1.0.0
