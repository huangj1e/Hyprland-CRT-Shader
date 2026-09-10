# Performance and power notes

The shader is designed to remain practical as a compositor-wide effect, but animated screen shaders have two distinct costs: fragment processing and continuously scheduling output frames.

## Current shader profile

The current implementation uses:

- one screen-shader pass
- normally four texture lookups per fragment
- no loops or blur kernels
- no FBM or simplex noise
- inexpensive hash and layered sine functions for analog noise
- physical-pixel displacement converted through `fullSize`

The strongest cost multipliers are output pixel count, refresh rate, number of monitors, and continuous rendering. A 4K high-refresh display processes far more fragments per second than the reference 1366×768 at 60 Hz environment.

## Continuous animation

The packaged configuration enables:

```text
debug:damage_tracking = 0
debug:vfr = false
```

This keeps the `time`-driven animation running while the desktop is otherwise idle. Continuous frame scheduling can consume more power than the shader's fragment math, especially on laptops, high-refresh outputs, 4K/ultrawide monitors, and multi-monitor systems.

When the panel disables the effect, it clears `screen_shader` and sets:

```text
debug:damage_tracking = 1
debug:vfr = true
```

These are runtime changes. A later Hyprland reload can reapply values from the user's configuration file.

If battery life or idle power is more important than animation, disable the effect when not needed, remove the shader configuration, or use a static variant that does not depend on `time` before restoring VFR/damage tracking.

## Control-panel overhead

The Rust + Slint panel is not required after values have been applied. While open, it adds normal GUI rendering and:

- waits 180 ms after the latest slider event before applying live-preview changes
- rewrites the user shader and triggers a Hyprland shader disable/enable cycle
- checks the Omarchy theme file modification time once per second

Rapid live tuning can therefore cause repeated shader recompilation. Disable **Live preview**, make several changes, and use **Apply now** when evaluating parameters on slower hardware.

## Measuring impact

Compare the same idle and active workloads with the shader enabled and disabled. Useful tools vary by GPU and platform, for example:

- `intel_gpu_top` for Intel GPUs
- `nvtop` for supported GPUs
- `nvidia-smi` for NVIDIA telemetry
- `radeontop` or appropriate Mesa tools for AMD
- `powerstat`, `powertop`, or battery discharge rate for system power

Record:

- resolution, scale, transform, and refresh rate for every output
- Hyprland and driver versions
- idle GPU utilization/power with the shader off and on
- frame pacing during window movement and fullscreen video
- whether continuous rendering or a particular shader change caused the regression

## Performance requirements for changes

When modifying `shaders/crt.frag`:

1. Count and document additional texture lookups.
2. Avoid loops, broad branching, blur kernels, and expensive layered procedural noise.
3. Keep calculations outside conditional paths where that reduces duplicated work, but verify compiler behavior where relevant.
4. Test readable text, animations, fullscreen content, and idle power.
5. Test all available scales and multiple outputs.
6. Mention expected performance impact in the pull request.

If a visually valuable feature is expensive, make its trade-off explicit and keep conservative defaults rather than silently increasing continuous compositor cost.
