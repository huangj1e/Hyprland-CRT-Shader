# Contributing

Thanks for helping improve Hyprland CRT Shader.

## Development setup

Required for full validation on Arch Linux:

```bash
sudo pacman -S --needed git base-devel glslang
```

Run checks before submitting a change:

```bash
make check
```

Build the Arch package as a regular user:

```bash
make package
```

## Guidelines

- Keep the shader single-pass and suitable for continuous compositor use.
- Avoid loops, blur kernels, FBM, and unnecessarily expensive noise functions.
- Express displacement in physical pixels using `fullSize`; do not hard-code a resolution.
- Preserve readable defaults. Strong glitch-art presets should be opt-in.
- Put user-tunable constants near the top of `shaders/crt.frag` and document them.
- Update English and Chinese documentation for user-visible behavior changes.
- Run `make check` and test `hyprctl reload` before opening a pull request.

## Commit style

Use short imperative subjects where practical, for example:

```text
feat: add rolling vertical-sync tear
fix: keep animation running while output is idle
docs: clarify HiDPI behavior
```

## Bug reports

Include:

- Hyprland version and commit
- GPU and Mesa/driver version
- Monitor resolution, refresh rate, scale, and transform
- Output from `hyprctl configerrors`
- Relevant compositor shader errors
- Whether the issue disappears when the shader is disabled
