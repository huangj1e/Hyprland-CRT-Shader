# Hyprland CRT Shader

[English](../README.md) · [贡献指南](../CONTRIBUTING.md) · [更新日志](../CHANGELOG.md)

一个作用于 Hyprland 最终合成画面的单 Pass CRT / 模拟电视 Shader。默认包含周期性故障效果，同时尽量保持桌面文字可读。

## 效果

- CRT 桶形曲面和管面软边缘
- 水平扫描线和 RGB 荧光粉子像素
- 色差、暗角和轻微闪烁
- 每帧变化的模拟信号噪点
- 水平同步波浪和偶发跳线
- 从上向下滚动的垂直同步撕裂带
- 周期性整屏坏信号、RGB 分离和矩形块噪声

实现中没有循环、模糊、FBM 或 Simplex Noise。通常每像素进行 4 次纹理采样。所有像素参数都通过 Hyprland 的 `fullSize` uniform 换算，没有写死分辨率。

## 兼容性

目标 API：

```glsl
in vec2 v_texcoord;
uniform sampler2D tex;
uniform float time;
uniform vec2 fullSize;
```

已在 Hyprland 0.56.2 上开发和测试。使用相差较大的版本时，请先确认 Hyprland Screen Shader API。

## Arch Linux 安装

确保系统具有基本构建工具：

```bash
sudo pacman -S --needed base-devel git
```

克隆并以普通用户构建 pacman 软件包：

```bash
git clone https://github.com/REPLACE_ME/hyprland-crt-shader.git
cd hyprland-crt-shader
make check
make package
sudo pacman -U ./dist/hyprland-crt-shader-*.pkg.tar.zst
```

不要使用 root 运行 `make package` 或 `makepkg`。独立的 AUR 元数据位于 `packaging/arch/`。

如果已经下载预编译包：

```bash
sudo pacman -U ./hyprland-crt-shader-1.0.0-1-any.pkg.tar.zst
```

项目正式提交到 AUR 后可以使用：

```bash
yay -S hyprland-crt-shader
```

注意：仓库中的 `REPLACE_ME` 必须由项目发布者替换成实际 GitHub 用户名，并发布 Git 仓库、提交 AUR 后，其他人才可以使用上面的 AUR 命令。

## 启用

软件包将 Shader 安装到：

```text
/usr/share/hyprland-crt-shader/crt.frag
```

### Lua 配置

在 `~/.config/hypr/hyprland.lua` 中加入：

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

### 传统配置

在 `~/.config/hypr/hyprland.conf` 中加入：

```ini
source = /usr/share/hyprland-crt-shader/hyprland-crt-shader.conf
```

重新加载并检查：

```bash
hyprctl reload
hyprctl configerrors
hyprctl getoption decoration:screen_shader
```

## 临时开关

安装后可以在 Hyprland 终端中直接执行：

```bash
hypr-crt-toggle
```

执行一次关闭，再执行一次开启。关闭时会恢复 VFR 和 damage tracking，减少静止桌面功耗。脚本不会修改配置文件，因此 `hyprctl reload` 会恢复配置文件中的默认状态。

## 调节参数

[`shaders/crt.frag`](../shaders/crt.frag) 顶部已经包含完整中文参数说明。主要参数：

| 效果 | 参数 |
|---|---|
| 故障周期 | `GLITCH_INTERVAL`、`GLITCH_DURATION`、`GLITCH_POWER` |
| 整屏抖动 | `SHAKE_BASE_PIXELS`、`SHAKE_GLITCH_PIXELS` |
| 水平波浪 | `WAVE_BASE_PIXELS`、`WAVE_GLITCH_PIXELS` |
| 滚动撕裂 | `ROLLING_TEAR_STRENGTH`、`ROLLING_TEAR_WIDTH`、`ROLLING_TEAR_SPEED` |
| RGB 分离 | `RGB_SHIFT_BASE_PIXELS`、`RGB_SHIFT_GLITCH` |
| 块状噪声 | `BLOCK_NOISE_STRENGTH`、`BLOCK_NOISE_AMOUNT` |
| CRT 外观 | `CURVATURE`、`SCANLINE_STRENGTH`、`RGB_MASK_STRENGTH`、`VIGNETTE_STRENGTH` |

不要直接编辑 `/usr/share` 下的软件包文件。先复制到用户目录：

```bash
mkdir -p ~/.config/hypr/shaders
cp /usr/share/hyprland-crt-shader/crt.frag ~/.config/hypr/shaders/crt.frag
```

然后把 `screen_shader` 改为用户目录中的文件。这样升级软件包不会覆盖自定义参数。

修改后验证：

```bash
glslangValidator -S frag ~/.config/hypr/shaders/crt.frag
hyprctl reload
hyprctl configerrors
```

## 性能和功耗

为了让 `time` 动画在桌面静止时继续播放，需要：

```text
debug:damage_tracking = 0
debug:vfr = false
```

这会提高静止桌面的 GPU 使用和笔记本功耗，4K、多显示器环境更加明显。暂时不需要时建议运行 `hypr-crt-toggle` 关闭。

## 故障恢复

在可用终端中临时关闭：

```bash
hyprctl eval 'hl.config({ decoration = { screen_shader = "" }, debug = { damage_tracking = 1, vfr = true } })'
```

如果画面无法操作，请切换到 TTY，删除或注释 Hyprland 配置中的 `screen_shader` 设置，再重新进入 Hyprland 会话。软件包安装过程本身不会修改用户配置。

## 卸载

```bash
sudo pacman -Rns hyprland-crt-shader
```

同时从 Hyprland 配置中移除对应的 `screen_shader` 或 `source` 配置。

## 许可证

MIT
