# Hyprland CRT Shader

A single-pass Hyprland screen shader that turns the complete compositor output into a readable CRT / analog-TV display with restrained glitch animation.

[简体中文](docs/README.zh-CN.md) · [Contributing](CONTRIBUTING.md) · [Changelog](CHANGELOG.md)

![Hyprland](https://img.shields.io/badge/Hyprland-0.56%2B-58E1FF)
![License](https://img.shields.io/badge/license-MIT-blue)

## Effects

- CRT barrel curvature and soft tube edge
- Horizontal scanlines and RGB phosphor mask
- Chromatic aberration
- Vignette and subtle flicker
- Frame-varying analog noise
- Horizontal sync waves and occasional jump lines
- A vertical-sync tear rolling from top to bottom
- Periodic whole-screen signal failure
- RGB separation and rectangular block noise during glitches

The implementation has no loops, blur, FBM, or simplex noise. It normally performs four texture lookups per pixel. Parameters use UV coordinates or physical pixels through Hyprland's `fullSize` uniform, so no resolution is hard-coded.

## Compatibility

This package targets Hyprland versions whose screen-shader API provides:

```glsl
in vec2 v_texcoord;
uniform sampler2D tex;
uniform float time;
uniform vec2 fullSize;
```

It was developed and tested on Hyprland 0.56.2. Always check release notes when using a substantially different Hyprland version.

## Arch Linux installation

### Build a local pacman package

Install the normal Arch build tools if they are not already present:

```bash
sudo pacman -S --needed base-devel git
```

Clone this repository and build the package as a regular user:

```bash
git clone https://github.com/huangj1e/Hyprland-CRT-Shader.git
cd hyprland-crt-shader
make check
make package
sudo pacman -U ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

`make package` stages the files and invokes `makepkg`. Do not run it as root. The standalone AUR metadata is maintained under `packaging/arch/`.

### Install a prebuilt package

If a release contains a `.pkg.tar.zst` file:

```bash
sudo pacman -U ./hyprland-crt-shader-1.0.0-1-any.pkg.tar.zst
```

### AUR workflow

After publishing the PKGBUILD to the AUR, users can install it using an AUR helper, for example:

```bash
yay -S hyprland-crt-shader
```

This repository contains both `PKGBUILD` and `.SRCINFO`. The project still needs to be submitted to the AUR before the command above becomes available publicly.

## Enable the shader

The package installs the shader at:

```text
/usr/share/hyprland-crt-shader/crt.frag
```

### Hyprland Lua configuration

Add this to `~/.config/hypr/hyprland.lua`:

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

Alternatively, load the packaged example:

```lua
require("/usr/share/hyprland-crt-shader/hyprland-crt-shader")
```

If absolute-path `require` is unavailable in your Lua configuration environment, copy the example block above instead.

### Legacy configuration

Add this to `~/.config/hypr/hyprland.conf`:

```ini
source = /usr/share/hyprland-crt-shader/hyprland-crt-shader.conf
```

Then reload:

```bash
hyprctl reload
```

Check for errors:

```bash
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

## Live control panel

Launch the graphical editor from a terminal inside Hyprland:

```bash
hypr-crt-control
```

The panel provides grouped sliders for glitch timing, screen shake, waves, rolling tears, RGB separation, noise, curvature, scanlines, vignette, flicker, and overscan. Changes are applied after a short debounce, so the effect updates while a slider is moved.

Hyprland does not currently expose arbitrary custom uniforms to screen shaders. The panel therefore creates and edits:

```text
~/.config/hyprland-crt-shader/crt.frag
```

It then temporarily points Hyprland at this per-user copy and recompiles it. The packaged shader under `/usr/share` remains unchanged. Slider values persist in the user copy between panel sessions.

The GUI requires Python and Tk. They are installed automatically by the Arch package dependencies.

## Temporary toggle

Run from a terminal inside the Hyprland session:

```bash
hypr-crt-toggle
```

Run it once to disable the shader and again to enable it. While disabled, the command restores VFR and damage tracking to reduce idle GPU use. It does not edit your configuration, so `hyprctl reload` restores the configured default.

## Parameter tuning

All parameters have Chinese comments in the top section of [`shaders/crt.frag`](shaders/crt.frag). Important groups include:

| Effect | Parameters |
|---|---|
| Glitch timing | `GLITCH_INTERVAL`, `GLITCH_DURATION`, `GLITCH_POWER` |
| Screen shake | `SHAKE_BASE_PIXELS`, `SHAKE_GLITCH_PIXELS` |
| Horizontal sync waves | `WAVE_BASE_PIXELS`, `WAVE_GLITCH_PIXELS` |
| Rolling tear | `ROLLING_TEAR_STRENGTH`, `ROLLING_TEAR_WIDTH`, `ROLLING_TEAR_SPEED` |
| RGB separation | `RGB_SHIFT_BASE_PIXELS`, `RGB_SHIFT_GLITCH` |
| Block noise | `BLOCK_NOISE_STRENGTH`, `BLOCK_NOISE_AMOUNT` |
| CRT appearance | `CURVATURE`, `SCANLINE_STRENGTH`, `RGB_MASK_STRENGTH`, `VIGNETTE_STRENGTH` |

To modify a system-installed shader safely, copy it into your user configuration first:

```bash
mkdir -p ~/.config/hypr/shaders
cp /usr/share/hyprland-crt-shader/crt.frag ~/.config/hypr/shaders/crt.frag
```

Then point `screen_shader` to that copy. Package upgrades will not overwrite your customized version.

Validate after editing:

```bash
glslangValidator -S frag ~/.config/hypr/shaders/crt.frag
hyprctl reload
hyprctl configerrors
```

## Power and performance

Animated screen shaders require frames even while the desktop is otherwise idle:

```text
debug:damage_tracking = 0
debug:vfr = false
```

This increases idle GPU use, especially at 4K or with multiple displays. The toggle command restores power-saving settings while the shader is off. If battery life matters more than animation, disable the shader or remove time-dependent effects and restore VFR/damage tracking.

## Recovery

Temporarily disable from a working terminal:

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

If the shader prevents normal use, switch to a TTY, remove/comment the `screen_shader` configuration, and restart or reload the Hyprland session. Configuration files are never modified by the package itself.

## Uninstall

Remove the package:

```bash
sudo pacman -Rns hyprland-crt-shader
```

Remove the `screen_shader` block or `source` line from your Hyprland configuration before or after uninstalling.

## License

MIT
