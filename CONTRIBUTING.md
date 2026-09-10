# Development and contribution guide

Thanks for helping improve Hyprland CRT Shader. This document describes the current Rust/Slint application, shader workflow, validation, deployment tests, and Arch packaging process.

User installation and configuration instructions are in [README.md](README.md) and [docs/README.zh-CN.md](docs/README.zh-CN.md).

## Repository layout

| Path | Purpose |
|---|---|
| `shaders/crt.frag` | Single-pass Hyprland screen shader and tunable defaults |
| `src/main.rs` | Control-panel state, localization, shader editing, theming, and `hyprctl` integration |
| `ui/app.slint` | Slint components and application layout |
| `build.rs` | Compiles `ui/app.slint` during the Cargo build |
| `config/` | Packaged Lua and legacy Hyprland configuration examples |
| `packaging/arch/` | Arch `PKGBUILD` and generated `.SRCINFO` |
| `packaging/*.desktop` | Desktop application integration |
| `scripts/check.sh` | Rust and optional GLSL validation |
| `scripts/build-arch-package.sh` | Reproducible local/CI package staging |
| `Makefile` | Check, install, uninstall, package, and clean entry points |
| `docs/` | Chinese guide plus compatibility and performance notes |

## Development dependencies

For complete validation on Arch Linux:

```bash
sudo pacman -S --needed base-devel git rust cargo cmake ninja \
  fontconfig libxkbcommon wayland glslang
```

Typical Debian/Ubuntu dependencies:

```bash
sudo apt update
sudo apt install git build-essential cargo rustc cmake ninja-build \
  libfontconfig1-dev libxkbcommon-dev libwayland-dev glslang-tools
```

The project pins Slint and `slint-build` to `1.9.2` and commits `Cargo.lock`. Use `--locked` in validation, release, and packaging commands. Do not update dependencies accidentally while making an unrelated change.

## Initial setup

```bash
git clone https://github.com/huangj1e/Hyprland-CRT-Shader.git
cd Hyprland-CRT-Shader
cargo fetch --locked
make check
```

Build a debug binary:

```bash
cargo build --locked
```

Build the optimized binary used by installation and packaging:

```bash
cargo build --release --locked
```

Build outputs are written below `target/` and are ignored by Git.

## Validation

Run before every commit or pull request:

```bash
make check
```

This runs:

```bash
cargo fmt --check
cargo check --locked
glslangValidator -S frag shaders/crt.frag  # if available
```

The equivalent script is:

```bash
./scripts/check.sh
```

If `glslangValidator` is absent, shader validation is skipped with a warning; install it before changing GLSL. Also check whitespace errors:

```bash
git diff --check
```

GitHub Actions repeats `make check` on Ubuntu and Arch Linux, then creates an Arch package as an unprivileged builder user. Local checks should pass before relying on CI.

## Running during development

The control panel must run inside an active Hyprland session because it requires `HYPRLAND_INSTANCE_SIGNATURE` and executes `hyprctl`:

```bash
cargo run --locked
```

For release behavior:

```bash
cargo run --release --locked
```

When run from a checkout, the application can use `shaders/crt.frag` as its source shader. On first run it creates the persistent user copy at:

```text
${XDG_CONFIG_HOME:-$HOME/.config}/hyprland-crt-shader/crt.frag
```

This persistence can hide changes to defaults during development. Back up or remove the user copy when testing first-run behavior:

```bash
cp ~/.config/hyprland-crt-shader/crt.frag /tmp/crt.frag.backup 2>/dev/null || true
rm -f ~/.config/hyprland-crt-shader/crt.frag
cargo run --locked
```

Be aware that moving sliders with live preview enabled changes the running Hyprland shader. Keep a terminal available and use the emergency command if necessary:

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

## Shader development

Follow these constraints unless a change explicitly justifies and documents an exception:

- Keep the implementation single-pass and suitable for compositor-wide continuous use.
- Avoid loops, blur kernels, FBM, simplex noise, and unnecessary texture lookups.
- Express displacement in physical pixels with `fullSize`; never hard-code an output resolution.
- Preserve readable defaults. Aggressive glitch-art behavior should be opt-in.
- Keep user-tunable `const float` values near the top of `shaders/crt.frag`.
- Preserve the `const float KEY = value;` shape expected by the control-panel parser.
- Add new control-panel parameters in the same order as their shader constants where practical.
- Document changes to texture lookups, branches, and continuous rendering cost.

Validate GLSL directly after every shader change:

```bash
glslangValidator -S frag shaders/crt.frag
```

Then test in Hyprland:

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

Test readable text, window movement, fullscreen content, idle animation, all available monitor scales/transforms, and multi-monitor behavior.

## Control-panel development

### Rust logic

`src/main.rs` contains:

- Parameter keys, ranges, steps, groups, and four-language labels
- System/source shader discovery
- Persistent user-copy creation and atomic constant replacement
- Generated Hyprland config includes for persistence across reloads and restarts
- Immediate runtime application through `hyprctl eval`
- 180 ms live-preview debounce
- Omarchy theme loading and one-second file polling
- UI callbacks and emergency recovery

When adding a tunable parameter:

1. Add or identify its `const float` in `shaders/crt.frag`.
2. Add a `Parameter` entry with exactly the same key.
3. Set safe `min`, `max`, and `step` values.
4. Assign the correct contiguous group.
5. Provide Chinese, English, Japanese, and Korean labels.
6. Confirm reset, live preview, direct apply, and persistence.

The grouped UI calculates a start and count for each parameter group. Keep parameter entries belonging to the same group contiguous unless that implementation is changed as well.

### Slint UI

`ui/app.slint` defines custom themed buttons, toggles, sliders, and the responsive grouped layout. After changing it, run `cargo check --locked`; `build.rs` compiles the Slint file and reports syntax/type errors.

Test at both preferred and minimum window sizes. Verify:

- Button text has adequate horizontal padding in every language.
- Slider tracks never overlap the numeric value column.
- Group spacing is compact and consistent.
- Long translated strings remain readable.
- Keyboard shortcuts still work while controls have focus.
- Colors have sufficient contrast with default and Omarchy themes.

### Localization and themes

User-visible labels should remain available in all four languages. Status and error messages are currently bilingual in several code paths; preserve at least useful Chinese/English diagnostics when modifying those paths.

Omarchy colors are loaded from:

```text
~/.local/state/omarchy/current/theme/colors.toml
```

The recognized keys are `accent`, `background`, `foreground`, and `color8`; `OMARCHY_ACCENT` overrides the accent. Always test fallback colors when that file is absent or malformed.

## Test a staged installation

Before installing into `/usr`, inspect a `DESTDIR` deployment:

```bash
rm -rf ./stage
make install DESTDIR="$PWD/stage" PREFIX=/usr
find ./stage -type f -print
```

Expected paths are documented in [README.md](README.md). For a real local installation:

```bash
sudo env "PATH=$PATH" make install PREFIX=/usr
```

Then close any running panel and launch the newly installed binary:

```bash
command -v hypr-crt-control
hypr-crt-control
```

Remove a direct installation with:

```bash
sudo make uninstall PREFIX=/usr
```

Do not mix unmanaged `make install` files and pacman ownership during packaging tests. If a pacman package is installed, upgrade/remove it through pacman.

## Arch package workflow

Package as a regular user; `makepkg` intentionally refuses to run as root:

```bash
make check
make package
```

Or pass options directly:

```bash
./scripts/build-arch-package.sh --noconfirm
```

The script stages the current checkout in `build/arch/`, runs `makepkg`, regenerates `packaging/arch/.SRCINFO`, and writes packages to `dist/`.

Inspect package metadata and contents:

```bash
pacman -Qip ./dist/hyprland-crt-shader-*.pkg.tar.zst
pacman -Qlp ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

Install or upgrade it:

```bash
sudo pacman -U ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

After installation, verify the desktop entry, executable, shader, configuration examples, and license. Run the panel and test a Hyprland reload.

When changing `pkgver`, dependencies, package description, sources, or checksums:

1. Update `Cargo.toml` where applicable.
2. Update `packaging/arch/PKGBUILD`.
3. Run `make package` to regenerate `packaging/arch/.SRCINFO`.
4. Update `CHANGELOG.md` and installation examples if needed.
5. Confirm that the package can install, upgrade, and uninstall cleanly.

The current PKGBUILD is designed around files staged from the repository. A public AUR package should use stable release sources and real checksums according to AUR packaging practices.

## Documentation requirements

Update both [README.md](README.md) and [docs/README.zh-CN.md](docs/README.zh-CN.md) for user-visible installation, configuration, or control-panel behavior. Also update:

- [docs/COMPATIBILITY.md](docs/COMPATIBILITY.md) for API, driver, scale, or tested-version changes
- [docs/PERFORMANCE.md](docs/PERFORMANCE.md) for rendering or power-impact changes
- [CHANGELOG.md](CHANGELOG.md) for release-facing changes
- Package metadata if runtime/build dependencies or installed files change

Ensure every documented command and path matches the current Makefile, scripts, and package contents.

## Commit and pull-request style

Use short imperative subjects where practical:

```text
feat: add rolling vertical-sync tear
fix: keep parameter groups compact
docs: document Arch packaging workflow
```

A pull request should explain motivation, visible behavior, validation performed, Hyprland/GPU environment, and performance impact. Use the pull-request checklist and do not omit documentation updates.

## Bug reports

Include:

- Hyprland version and commit
- GPU and Mesa/proprietary driver version
- Monitor resolution, refresh rate, scale, and transform
- Installation method and package/project version
- Output from `hyprctl configerrors`
- Relevant compositor or shader errors
- Whether the issue disappears when the shader is disabled
- Whether the system shader or persistent user copy is active
