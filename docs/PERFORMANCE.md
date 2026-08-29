# Performance notes

The shader is intended to remain practical as a compositor-wide effect:

- one screen-shader pass
- four texture lookups per fragment in the current implementation
- no loops, blur kernels, FBM, or simplex noise
- hash and layered sine functions for analog noise
- displacement expressed in physical pixels through `fullSize`

Animation requires Hyprland to render continuously:

```text
debug:damage_tracking = 0
debug:vfr = false
```

Continuous rendering can cost more power than the fragment math itself, particularly on laptops, high-refresh displays, 4K outputs, and multi-monitor setups. Use `hypr-crt-toggle` when the effect is not needed.

When changing the shader, document any additional texture lookups or expensive operations in the pull request. Test text rendering, window animation, fullscreen content, idle power behavior, and all available output scales.
