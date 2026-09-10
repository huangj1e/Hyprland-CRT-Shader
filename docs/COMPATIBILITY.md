# Compatibility

This document records the screen-shader API, runtime assumptions, tested environment, and common compatibility checks for the current project.

## Supported environment

The current code and package metadata target Hyprland 0.56 or later. The initial/current reference environment was:

- Hyprland 0.56.2
- OpenGL ES 3.2 on Mesa 26.2.1
- Intel HD Graphics 520
- 1366×768 at 60 Hz

This is a tested environment, not a guarantee that every later Hyprland, renderer, GPU, or driver combination behaves identically. Hyprland's shader and configuration APIs may change between releases.

## Required screen-shader interface

The shader expects Hyprland to provide:

```glsl
in vec2 v_texcoord;
uniform sampler2D tex;
uniform float time;
uniform vec2 fullSize;
```

`tex` supplies the compositor output, `time` drives animation, and `fullSize` converts physical-pixel settings to UV coordinates. If a newer Hyprland release changes names, types, GLSL version requirements, or coordinate conventions, compare this repository's shader with that release's official `example/screenShader.frag` and renderer source.

Validate syntax independently with:

```bash
glslangValidator -S frag shaders/crt.frag
```

A successful standalone validation does not prove compositor API compatibility; always check Hyprland after loading:

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

## Runtime control API

The graphical panel additionally assumes:

- It is launched inside Hyprland and `HYPRLAND_INSTANCE_SIGNATURE` is set.
- `hyprctl` is available in `PATH`.
- The current Hyprland build accepts `hyprctl eval` with `hl.config(...)`.
- `decoration:screen_shader`, `debug:damage_tracking`, and `debug:vfr` remain valid configuration keys.

If the static shader works but the panel cannot enable or disable it, test the exact runtime operation manually:

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

Configuration syntax support can differ from shader support. On versions without the expected Lua/eval interface, configure the shader statically and treat the panel as unsupported until its integration is adapted.

## Linux desktop dependencies

The installed panel uses Slint 1.9.2 with the Winit and FemtoVG backend. Package metadata declares:

- `fontconfig`
- `libxkbcommon`
- `wayland`

The **Open Shader folder** action invokes `xdg-open`; install `xdg-utils` if that button fails. Fonts must contain glyphs for the selected Chinese, English, Japanese, or Korean UI language.

## Resolution, scale, and multiple outputs

The shader does not hard-code a resolution. Hyprland supplies each output's framebuffer size through `fullSize`; physical-pixel displacement is converted separately for each output, and barrel-curvature math includes aspect correction.

Hyprland uses one global screen-shader configuration. Parameter values are therefore shared across outputs even though framebuffer dimensions are evaluated per output. Visually inspect:

- fractional and integer output scales
- rotated/transformed outputs
- mixed-resolution and mixed-refresh multi-monitor layouts
- fullscreen and direct-scanout-sensitive content

A physical-pixel value can look proportionally weaker on a very high-density display even when it is mathematically resolution-aware.

## Installed and source-tree shader discovery

The panel searches for its default/source shader in this order:

1. `/usr/share/hyprland-crt-shader/crt.frag`
2. `/usr/local/share/hyprland-crt-shader/crt.frag`
3. a source-relative location inferred from the executable
4. `shaders/crt.frag` below the Cargo manifest directory embedded at build time

After the first launch, tuning is stored separately at:

```text
${XDG_CONFIG_HOME:-$HOME/.config}/hyprland-crt-shader/crt.frag
```

An old user copy can contain constants incompatible with a newer application. Back it up and let the panel recreate it when investigating an upgrade problem:

```bash
mv ~/.config/hyprland-crt-shader/crt.frag \
   ~/.config/hyprland-crt-shader/crt.frag.backup
hypr-crt-control
```

## Reporting compatibility problems

Include Hyprland version/commit, GPU, driver, output mode/scale/transform, active shader path, installation method, `hyprctl configerrors`, and whether the static packaged shader works without the panel. See [CONTRIBUTING.md](../CONTRIBUTING.md) for the complete report checklist.
