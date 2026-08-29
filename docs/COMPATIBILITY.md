# Compatibility

## Confirmed environment

The initial implementation was verified with:

- Hyprland 0.56.2
- OpenGL ES 3.2 on Mesa 26.2.1
- Intel HD Graphics 520
- 1366x768 at 60 Hz

## Required screen-shader interface

```glsl
in vec2 v_texcoord;
uniform sampler2D tex;
uniform float time;
uniform vec2 fullSize;
```

Hyprland's screen-shader API is not guaranteed to remain identical forever. Before reporting a shader bug on a substantially different Hyprland release, compare that release's official `example/screenShader.frag` and renderer implementation.

## Resolution and scale

The shader does not hard-code a resolution. Hyprland supplies each output's framebuffer size through `fullSize`. Physical-pixel displacement is converted to UV coordinates separately on each output. Aspect correction is applied to barrel curvature.

Hyprland uses one global shader configuration, so parameters are shared across outputs even though each output is sized independently.
