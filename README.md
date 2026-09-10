# Hyprland CRT Shader

A single-pass CRT / analog-TV screen shader for Hyprland, with a native Rust + Slint control panel for live tuning.

[简体中文](docs/README.zh-CN.md) · [Development](CONTRIBUTING.md) · [Compatibility](docs/COMPATIBILITY.md) · [Performance](docs/PERFORMANCE.md) · [Changelog](CHANGELOG.md)

![Hyprland](https://img.shields.io/badge/Hyprland-0.56%2B-58E1FF)
![Rust](https://img.shields.io/badge/Rust-2021-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

## Demo

[![Hyprland CRT Shader demo — click to watch on YouTube](docs/review.jpg)](https://youtu.be/snSKB8iGME8)

▶️ **Click the image above to watch the demo video on YouTube** — CRT curvature, scanlines, RGB phosphor mask, glitch effects, and the live control panel in action.

## Features

- CRT barrel curvature, soft tube edges, scanlines, RGB phosphor mask, vignette, and flicker
- Analog white noise, horizontal interference, chromatic aberration, and RGB separation
- Screen shake, horizontal sync waves, block noise, periodic signal failures, and a rolling sync tear
- Independent horizontal and vertical image scaling
- Resolution-aware pixel displacement through Hyprland's `fullSize` uniform
- Native graphical control panel with 180 ms debounced live preview
- Persistent per-user shader copy; package-managed files are not edited by the panel
- Chinese, English, Japanese, and Korean UI
- Automatic Omarchy color loading and live theme refresh
- Keyboard recovery shortcuts: `Ctrl+R` resets parameters and `Ctrl+Shift+E` disables the effect

The shader has one pass, normally uses four texture lookups per pixel, and contains no loops, blur kernels, FBM, or simplex noise.

## Requirements

### Runtime

- Hyprland with this screen-shader interface:

  ```glsl
  in vec2 v_texcoord;
  uniform sampler2D tex;
  uniform float time;
  uniform vec2 fullSize;
  ```

- `hyprctl` available in `PATH`
- `fontconfig`, `libxkbcommon`, and Wayland runtime libraries for the control panel
- `xdg-open` (normally from `xdg-utils`) to use **Open Shader folder**

The current code and package metadata target Hyprland 0.56 or later and were tested on Hyprland 0.56.2. See [Compatibility](docs/COMPATIBILITY.md) before using a substantially different release.

### Build

- Rust toolchain with Cargo (stable, Rust 2021 edition)
- C/C++ build toolchain
- CMake and Ninja (required by the Slint renderer dependency)
- Development headers for Fontconfig, libxkbcommon, and Wayland
- `glslangValidator` from `glslang`/`glslang-tools` for full shader validation (optional but recommended)

Arch Linux:

```bash
sudo pacman -S --needed base-devel git rust cargo cmake ninja fontconfig libxkbcommon wayland glslang
```

Debian/Ubuntu package names vary by release; a typical setup is:

```bash
sudo apt update
sudo apt install git build-essential cargo rustc cmake ninja-build \
  libfontconfig1-dev libxkbcommon-dev libwayland-dev glslang-tools
```

## Quick start on Arch Linux

Build and install a native pacman package as a **regular user**:

```bash
git clone https://github.com/huangj1e/Hyprland-CRT-Shader.git
cd Hyprland-CRT-Shader
make check
make package
sudo pacman -U ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

Enable the packaged shader in `~/.config/hypr/hyprland.conf`:

```ini
source = /usr/share/hyprland-crt-shader/hyprland-crt-shader.conf
```

Then reload and verify:

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

Launch the panel from the application menu or run:

```bash
hypr-crt-control
```

## Build from source

Clone the repository and run all available checks:

```bash
git clone https://github.com/huangj1e/Hyprland-CRT-Shader.git
cd Hyprland-CRT-Shader
make check
```

`make check` runs:

```bash
cargo fmt --check
cargo check --locked
glslangValidator -S frag shaders/crt.frag  # when installed
```

Build an optimized binary:

```bash
cargo build --release --locked
```

The result is:

```text
target/release/hyprland-crt-shader
```

Run it directly from the repository **inside a Hyprland session**:

```bash
./target/release/hyprland-crt-shader
```

The application exits if `HYPRLAND_INSTANCE_SIGNATURE` is absent. When run from the repository root it uses the local `shaders/crt.frag` if no installed system shader exists.

## Install from source

The recommended installation method on Arch is the pacman package described below. For a direct system installation:

```bash
cargo build --release --locked
sudo env "PATH=$PATH" make install PREFIX=/usr
```

The install target currently builds once more before copying files. `PATH` must therefore expose Cargo under `sudo`. To stage files without touching the live root filesystem, use `DESTDIR`:

```bash
rm -rf ./stage
make install DESTDIR="$PWD/stage" PREFIX=/usr
find ./stage -type f -print
```

Installed files:

| Path | Purpose |
|---|---|
| `/usr/bin/hypr-crt-control` | Control-panel launcher |
| `/usr/share/applications/hyprland-crt-control.desktop` | Desktop application entry |
| `/usr/share/hyprland-crt-shader/crt.frag` | Packaged shader and panel defaults |
| `/usr/share/hyprland-crt-shader/hyprland-crt-shader.lua` | Lua configuration example |
| `/usr/share/hyprland-crt-shader/hyprland-crt-shader.conf` | Legacy configuration include |
| `/usr/share/licenses/hyprland-crt-shader/LICENSE` | License |

`PREFIX=/usr/local` is also supported; the application searches both `/usr/share/hyprland-crt-shader/crt.frag` and `/usr/local/share/hyprland-crt-shader/crt.frag`.

To remove a direct installation made with the Makefile:

```bash
sudo make uninstall PREFIX=/usr
```

## Build an Arch Linux package

Never run `makepkg`, `make package`, or `scripts/build-arch-package.sh` as root.

```bash
make check
make package
```

The packaging script performs the following operations:

1. Recreates `build/arch/`.
2. Copies the shader, Rust/Slint sources, configuration examples, desktop entry, lockfile, and license.
3. Runs `makepkg --cleanbuild --force` with any additional arguments passed to the script.
4. Regenerates `packaging/arch/.SRCINFO`.
5. Copies resulting `*.pkg.tar.*` files to `dist/`.

For a noninteractive build:

```bash
./scripts/build-arch-package.sh --noconfirm
```

Inspect and install the result:

```bash
pacman -Qip ./dist/hyprland-crt-shader-*.pkg.tar.zst
pacman -Qlp ./dist/hyprland-crt-shader-*.pkg.tar.zst
sudo pacman -U ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

Upgrade by building a newer checkout and running the same `pacman -U` command. Uninstall with:

```bash
sudo pacman -Rns hyprland-crt-shader
```

The repository's `packaging/arch/PKGBUILD` packages the current checkout through the staging script; it is suitable for local/CI packaging. Publishing to the AUR additionally requires an AUR repository, release-source URLs/checksums, and normal AUR maintainer workflow. Do not assume `yay -S hyprland-crt-shader` is available until the package has actually been published.

## Configure Hyprland

The packaged shader is installed at:

```text
/usr/share/hyprland-crt-shader/crt.frag
```

### Lua configuration

Add the following to `~/.config/hypr/hyprland.lua`:

```lua
hl.config({
    decoration = {
        screen_shader = "/usr/share/hyprland-crt-shader/crt.frag",
    },
    debug = {
        damage_tracking = 0,
        vfr             = false,
    },
})
```

Or load the packaged example when absolute-path `require` is supported by the Lua configuration environment:

```lua
require("/usr/share/hyprland-crt-shader/hyprland-crt-shader")
```

### Legacy configuration

Add this to `~/.config/hypr/hyprland.conf`:

```ini
source = /usr/share/hyprland-crt-shader/hyprland-crt-shader.conf
```

`damage_tracking = 0` and `vfr = false` keep time-based animation moving while the desktop is idle, but increase power use.

### Verify deployment

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
ls -l /usr/share/hyprland-crt-shader/crt.frag
command -v hypr-crt-control
```

## Control-panel behavior and user data

Start the panel inside the active Hyprland session:

```bash
hypr-crt-control
```

At first launch it creates:

```text
${XDG_CONFIG_HOME:-$HOME/.config}/hyprland-crt-shader/crt.frag
```

The file is copied from the system shader only when the user copy does not already exist. The panel edits constants in this copy atomically, points Hyprland to it through `hyprctl eval`, and recompiles the effect. Slider changes use a 180 ms debounce when live preview is enabled.

Important behavior:

- Package upgrades do not overwrite the user copy.
- **Reset defaults** reads defaults from the currently installed system shader.
- Applying a value automatically writes persistent configuration below `${XDG_CONFIG_HOME:-$HOME/.config}/hyprland-crt-shader/` and adds one marked include to `~/.config/hypr/hyprland.conf` and/or `~/.config/hypr/hyprland.lua`.
- The generated persistent configuration points `screen_shader` at the user copy, so panel values survive `hyprctl reload`, Hyprland restarts, and login.
- Disabling or emergency-disabling the effect also updates the generated configuration to keep it disabled after restart and restores `damage_tracking = 1`, `vfr = true`.
- Existing Hyprland configuration is otherwise preserved; the marked include is added only once.

The panel starts in English. Click the language button to cycle through Chinese, English, Japanese, and Korean. On Omarchy it reads `~/.local/state/omarchy/current/theme/colors.toml`, watches that file for changes once per second, and accepts `OMARCHY_ACCENT` as an accent-color override.

## Manual shader customization

To maintain a separate hand-edited shader:

```bash
mkdir -p ~/.config/hypr/shaders
cp /usr/share/hyprland-crt-shader/crt.frag ~/.config/hypr/shaders/crt.frag
$EDITOR ~/.config/hypr/shaders/crt.frag
glslangValidator -S frag ~/.config/hypr/shaders/crt.frag
```

Point `decoration:screen_shader` at that file and reload Hyprland. Tunable constants are grouped near the top of [`shaders/crt.frag`](shaders/crt.frag). Do not directly edit `/usr/share`; package upgrades replace it.

## Recovery and troubleshooting

Immediately disable the effect from a working terminal:

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

Inside the panel, use `Ctrl+Shift+E`. Use `Ctrl+R` to restore installed defaults.

If the display is unusable, switch to a TTY, remove/comment the shader configuration, and restart the Hyprland session. The installer and package do not edit user Hyprland configuration automatically.

Common checks:

```bash
hyprctl configerrors
hyprctl getoption decoration:screen_shader
glslangValidator -S frag /usr/share/hyprland-crt-shader/crt.frag
echo "$HYPRLAND_INSTANCE_SIGNATURE"
```

If an old user shader is incompatible with a newer release, preserve it and let the panel recreate it:

```bash
mv ~/.config/hyprland-crt-shader/crt.frag \
   ~/.config/hyprland-crt-shader/crt.frag.backup
hypr-crt-control
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for repository layout, development commands, UI/shader modification rules, CI parity, packaging tests, and contribution requirements.

## License

MIT — see [LICENSE](LICENSE).
