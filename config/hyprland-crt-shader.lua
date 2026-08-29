-- Hyprland 0.56+ Lua configuration example.
-- Copy or require this file from ~/.config/hypr/hyprland.lua.
hl.config({
    decoration = {
        screen_shader = "/usr/share/hyprland-crt-shader/crt.frag",
    },
    debug = {
        -- time-based shaders need full, continuously scheduled output frames.
        damage_tracking = 0,
        vfr             = false,
    },
})
