# Hyprland CRT Shader 1.1.0

Version 1.1.0 delivers the native live-control workflow and a refined multilingual interface for the Hyprland CRT screen shader.

## Highlights

- Native Rust + Slint control panel; Python and Tk are not required at runtime
- Live tuning for glitch timing, shake, waves, rolling tears, RGB separation, noise, and CRT display parameters
- 180 ms debounced live preview to avoid recompiling on every raw pointer event
- Persistent user shader at `${XDG_CONFIG_HOME:-$HOME/.config}/hyprland-crt-shader/crt.frag`
- Automatically generated Hyprland include so UI changes survive reloads, restarts, and login
- Chinese, English, Japanese, and Korean interface languages
- Omarchy theme colors with automatic refresh when the theme changes
- `Ctrl+R` to restore installed defaults
- `Ctrl+Shift+E` to disable the effect immediately
- Improved button padding, slider/value separation, and compact parameter-group spacing
- Expanded build, deployment, Arch packaging, compatibility, recovery, and performance documentation

## Fixes

- Numeric values no longer overlap slider tracks.
- Parameter groups no longer contain large blank areas.
- Repeated parameter descriptions have been removed.
- Bordered controls now have consistent horizontal padding, except for the fixed-width language selector.

## Compatibility

The project targets Hyprland 0.56 or later and has been tested on Hyprland 0.56.2. The screen shader expects `v_texcoord`, `tex`, `time`, and `fullSize` from Hyprland.

Animated effects use `damage_tracking = 0` and `vfr = false`, which can increase idle GPU use and power consumption.

## Install the attached Arch package

Download `hyprland-crt-shader-1.1.0-1-x86_64.pkg.tar.zst` from this release, then run:

```bash
sudo pacman -U ./hyprland-crt-shader-1.1.0-1-x86_64.pkg.tar.zst
```

Optionally verify the checksum first using the attached `SHA256SUMS` file:

```bash
sha256sum -c SHA256SUMS
```

## Build and install from source

```bash
git clone --branch v1.1.0 --depth 1 \
  https://github.com/huangj1e/Hyprland-CRT-Shader.git
cd Hyprland-CRT-Shader
sudo pacman -S --needed base-devel git rust cargo cmake ninja \
  fontconfig libxkbcommon wayland glslang
make check
make package
sudo pacman -U ./dist/hyprland-crt-shader-1.1.0-1-x86_64.pkg.tar.zst
```

Run `make package` as a regular user, not as root.

## Enable the shader

Add this line to `~/.config/hypr/hyprland.conf`:

```ini
source = /usr/share/hyprland-crt-shader/hyprland-crt-shader.conf
```

Reload and check the configuration:

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

Launch the control panel from the application menu or run:

```bash
hypr-crt-control
```

## Upgrade notes

The panel does not overwrite an existing user shader. If an older user copy is incompatible or you want the new defaults, back it up before opening the panel:

```bash
mv ~/.config/hyprland-crt-shader/crt.frag \
   ~/.config/hyprland-crt-shader/crt.frag.backup
hypr-crt-control
```

After the first apply, the panel adds a marked include to the active `~/.config/hypr/hyprland.conf` and/or `hyprland.lua`. That generated configuration points to the user shader, so tuned values and the enabled/disabled state survive Hyprland reloads and restarts.

## Emergency recovery

Press `Ctrl+Shift+E` in the panel, or run:

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

## Uninstall

```bash
sudo pacman -Rns hyprland-crt-shader
```

Remove the corresponding `source` or `screen_shader` entry from your Hyprland configuration.

See the English and Chinese README files for complete deployment and development documentation.
