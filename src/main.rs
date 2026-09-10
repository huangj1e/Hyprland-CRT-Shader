use slint::{Model, ModelRc, SharedString, VecModel};
use std::{
    env, fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
    rc::Rc,
    sync::mpsc,
    thread,
    time::Duration,
};

slint::include_modules!();

const APP: &str = "hyprland-crt-shader";
const SYSTEM_SHADERS: &[&str] = &[
    "/usr/share/hyprland-crt-shader/crt.frag",
    "/usr/local/share/hyprland-crt-shader/crt.frag",
];

#[derive(Clone, Copy)]
struct Parameter {
    key: &'static str,
    label: &'static str,
    hint: &'static str,
    min: f32,
    max: f32,
    step: f32,
    group: i32,
}

const GROUPS: &[&str] = &[
    "信号故障 / Signal glitch",
    "抖动与波浪 / Shake and waves",
    "滚动同步撕裂 / Rolling sync tear",
    "RGB 与噪声 / RGB and noise",
    "CRT 显示 / CRT display",
];

const PARAMETERS: &[Parameter] = &[
    Parameter {
        key: "GLITCH_INTERVAL",
        label: "故障周期",
        hint: "Interval",
        min: 1.0,
        max: 120.0,
        step: 0.5,
        group: 0,
    },
    Parameter {
        key: "GLITCH_DURATION",
        label: "故障持续时间",
        hint: "Duration",
        min: 0.05,
        max: 3.0,
        step: 0.05,
        group: 0,
    },
    Parameter {
        key: "GLITCH_POWER",
        label: "周期性故障强度",
        hint: "Power",
        min: 0.0,
        max: 1.5,
        step: 0.01,
        group: 0,
    },
    Parameter {
        key: "BASE_GLITCH",
        label: "常态信号不稳",
        hint: "Base instability",
        min: 0.0,
        max: 0.6,
        step: 0.01,
        group: 0,
    },
    Parameter {
        key: "SHAKE_BASE_PIXELS",
        label: "常态抖动像素",
        hint: "Idle shake",
        min: 0.0,
        max: 4.0,
        step: 0.05,
        group: 1,
    },
    Parameter {
        key: "SHAKE_GLITCH_PIXELS",
        label: "故障抖动像素",
        hint: "Glitch shake",
        min: 0.0,
        max: 20.0,
        step: 0.1,
        group: 1,
    },
    Parameter {
        key: "WAVE_BASE_PIXELS",
        label: "常态水平波纹",
        hint: "Idle wave",
        min: 0.0,
        max: 5.0,
        step: 0.05,
        group: 1,
    },
    Parameter {
        key: "WAVE_GLITCH_PIXELS",
        label: "故障水平波纹",
        hint: "Glitch wave",
        min: 0.0,
        max: 30.0,
        step: 0.1,
        group: 1,
    },
    Parameter {
        key: "ROLLING_TEAR_STRENGTH",
        label: "滚动撕裂强度",
        hint: "Strength",
        min: 0.0,
        max: 2.0,
        step: 0.01,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_WIDTH",
        label: "撕裂带高度比例",
        hint: "Width",
        min: 0.005,
        max: 0.30,
        step: 0.005,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_SPEED",
        label: "向下滚动速度",
        hint: "Speed",
        min: 0.0,
        max: 2.0,
        step: 0.01,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_PIXELS",
        label: "撕裂水平错位",
        hint: "Displacement",
        min: 0.0,
        max: 50.0,
        step: 0.25,
        group: 2,
    },
    Parameter {
        key: "RGB_SHIFT_BASE_PIXELS",
        label: "常态 RGB 分离",
        hint: "Idle RGB shift",
        min: 0.0,
        max: 8.0,
        step: 0.05,
        group: 3,
    },
    Parameter {
        key: "RGB_SHIFT_GLITCH",
        label: "故障 RGB 分离",
        hint: "Glitch RGB shift",
        min: 0.0,
        max: 40.0,
        step: 0.25,
        group: 3,
    },
    Parameter {
        key: "BLOCK_NOISE_STRENGTH",
        label: "块状噪声强度",
        hint: "Block noise",
        min: 0.0,
        max: 1.0,
        step: 0.01,
        group: 3,
    },
    Parameter {
        key: "WHITE_NOISE_STRENGTH",
        label: "白噪点强度",
        hint: "White noise",
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 3,
    },
    Parameter {
        key: "HORIZONTAL_LINE",
        label: "高频水平暗纹",
        hint: "Horizontal noise",
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 3,
    },
    Parameter {
        key: "CURVATURE",
        label: "屏幕曲率",
        hint: "Curvature",
        min: 0.0,
        max: 0.20,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "SCANLINE_STRENGTH",
        label: "扫描线强度",
        hint: "Scanlines",
        min: 0.0,
        max: 0.40,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "RGB_MASK_STRENGTH",
        label: "RGB 荧光粉强度",
        hint: "Phosphor mask",
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "VIGNETTE_STRENGTH",
        label: "暗角强度",
        hint: "Vignette",
        min: 0.0,
        max: 0.70,
        step: 0.01,
        group: 4,
    },
    Parameter {
        key: "FLICKER_STRENGTH",
        label: "亮度闪烁",
        hint: "Flicker",
        min: 0.0,
        max: 0.08,
        step: 0.001,
        group: 4,
    },
    Parameter {
        key: "OVERSCAN",
        label: "画面裁边/缩小",
        hint: "Overscan",
        min: -0.10,
        max: 0.10,
        step: 0.002,
        group: 4,
    },
];

fn config_dir() -> PathBuf {
    env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
        .unwrap_or_else(|| PathBuf::from(".config"))
        .join(APP)
}

fn source_shader() -> Result<PathBuf, String> {
    SYSTEM_SHADERS
        .iter()
        .map(PathBuf::from)
        .find(|p| p.is_file())
        .or_else(|| {
            env::current_exe().ok().and_then(|exe| {
                exe.parent()?
                    .parent()?
                    .join("shaders/crt.frag")
                    .canonicalize()
                    .ok()
            })
        })
        .or_else(|| {
            let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("shaders/crt.frag");
            path.is_file().then_some(path)
        })
        .ok_or_else(|| "找不到 crt.frag / crt.frag was not found".into())
}

fn ensure_user_shader(source: &Path, user: &Path) -> Result<(), String> {
    fs::create_dir_all(user.parent().unwrap()).map_err(|e| e.to_string())?;
    if !user.exists() {
        fs::copy(source, user).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn find_value(text: &str, key: &str) -> Result<f32, String> {
    let marker = format!("const float {key}");
    let line = text
        .lines()
        .find(|line| line.trim_start().starts_with(&marker))
        .ok_or_else(|| format!("Shader 参数不存在: {key}"))?;
    let value = line
        .split_once('=')
        .and_then(|(_, right)| right.split(';').next())
        .ok_or_else(|| format!("Shader 参数格式无效: {key}"))?;
    value
        .trim()
        .parse()
        .map_err(|_| format!("Shader 参数值无效: {key}"))
}

fn read_values(path: &Path) -> Result<Vec<f32>, String> {
    let text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    PARAMETERS
        .iter()
        .map(|parameter| find_value(&text, parameter.key))
        .collect()
}

fn replace_value(text: &mut String, key: &str, value: f32) -> Result<(), String> {
    let marker = format!("const float {key}");
    let start = text
        .find(&marker)
        .ok_or_else(|| format!("Shader 参数不存在: {key}"))?;
    let relative_equal = text[start..]
        .find('=')
        .ok_or_else(|| format!("Shader 参数格式无效: {key}"))?;
    let value_start = start + relative_equal + 1;
    let relative_end = text[value_start..]
        .find(';')
        .ok_or_else(|| format!("Shader 参数格式无效: {key}"))?;
    let value_end = value_start + relative_end;
    text.replace_range(value_start..value_end, &format!(" {value:.6}"));
    Ok(())
}

fn write_values(path: &Path, values: &[f32]) -> Result<(), String> {
    let mut text = fs::read_to_string(path).map_err(|e| e.to_string())?;
    for (parameter, value) in PARAMETERS.iter().zip(values) {
        replace_value(&mut text, parameter.key, *value)?;
    }
    let temporary = path.with_extension(format!("frag.{}.tmp", std::process::id()));
    let result = (|| -> io::Result<()> {
        let mut file = fs::File::create(&temporary)?;
        file.write_all(text.as_bytes())?;
        file.sync_all()?;
        fs::rename(&temporary, path)
    })();
    if result.is_err() {
        let _ = fs::remove_file(&temporary);
    }
    result.map_err(|e| e.to_string())
}

fn hyprctl(args: &[&str]) -> Result<Output, String> {
    Command::new("hyprctl")
        .args(args)
        .output()
        .map_err(|e| format!("无法运行 hyprctl: {e}"))
        .and_then(|output| {
            if output.status.success() {
                Ok(output)
            } else {
                Err(String::from_utf8_lossy(&output.stderr).trim().to_owned())
            }
        })
}

fn set_effect(enabled: bool, shader: &Path) -> Result<(), String> {
    let code = if enabled {
        format!("hl.config({{ decoration = {{ screen_shader = \"{}\" }}, debug = {{ damage_tracking = 0, vfr = false }} }})", shader.display())
    } else {
        "hl.config({ decoration = { screen_shader = \"\" }, debug = { damage_tracking = 1, vfr = true } })".into()
    };
    hyprctl(&["eval", &code]).map(|_| ())
}

fn detect_enabled() -> bool {
    hyprctl(&["getoption", "decoration:screen_shader"])
        .map(|output| {
            let text = String::from_utf8_lossy(&output.stdout);
            text.contains("crt.frag") && text.contains("str:")
        })
        .unwrap_or(false)
}

fn values_from_model(model: &ModelRc<f32>) -> Vec<f32> {
    (0..model.row_count())
        .filter_map(|row| model.row_data(row))
        .collect()
}

fn apply(window: &AppWindow, user_shader: &Path) -> Result<(), String> {
    write_values(
        user_shader,
        &values_from_model(&window.get_parameter_values()),
    )?;
    if window.get_effect_enabled() {
        let _ = set_effect(false, user_shader);
        set_effect(true, user_shader)?;
    }
    Ok(())
}

fn set_status(window: &slint::Weak<AppWindow>, message: &str) {
    if let Some(window) = window.upgrade() {
        window.set_status_text(message.into());
    }
}

fn run() -> Result<(), String> {
    if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_none() {
        return Err("请在 Hyprland 会话中运行 / run inside a Hyprland session".into());
    }
    let source = source_shader()?;
    let user_dir = config_dir();
    let user_shader = user_dir.join("crt.frag");
    ensure_user_shader(&source, &user_shader)?;
    let defaults = read_values(&source)?;
    let values = read_values(&user_shader)?;

    let window = AppWindow::new().map_err(|e| e.to_string())?;
    window.set_group_titles(ModelRc::new(VecModel::from(
        GROUPS
            .iter()
            .map(|s| SharedString::from(*s))
            .collect::<Vec<_>>(),
    )));
    window.set_parameter_labels(ModelRc::new(VecModel::from(
        PARAMETERS
            .iter()
            .map(|p| format!("{}  {}", p.label, p.hint).into())
            .collect::<Vec<SharedString>>(),
    )));
    window.set_parameter_minimums(ModelRc::new(VecModel::from(
        PARAMETERS.iter().map(|p| p.min).collect::<Vec<_>>(),
    )));
    window.set_parameter_maximums(ModelRc::new(VecModel::from(
        PARAMETERS.iter().map(|p| p.max).collect::<Vec<_>>(),
    )));
    window.set_parameter_groups(ModelRc::new(VecModel::from(
        PARAMETERS.iter().map(|p| p.group).collect::<Vec<_>>(),
    )));
    window.set_parameter_values(ModelRc::new(VecModel::from(values)));
    window.set_effect_enabled(detect_enabled());

    let (debounce_tx, debounce_rx) = mpsc::channel::<u64>();
    thread::spawn(move || {
        while debounce_rx.recv().is_ok() {
            // Restart the timeout whenever another slider event arrives.
            while debounce_rx.recv_timeout(Duration::from_millis(180)).is_ok() {}
            let _ = slint::invoke_from_event_loop(|| {
                // The callback keeps all model and window access on the UI thread.
                if let Some(window) = APP_WEAK.with(|slot| slot.borrow().upgrade()) {
                    window.invoke_apply_now();
                }
            });
        }
    });

    APP_WEAK.with(|slot| *slot.borrow_mut() = window.as_weak());
    let generation = Rc::new(std::cell::Cell::new(0_u64));
    let weak = window.as_weak();
    let tx = debounce_tx.clone();
    let generation_for_change = generation.clone();
    window.on_parameter_changed(move |index, raw| {
        let Some(window) = weak.upgrade() else {
            return;
        };
        let Some(parameter) = PARAMETERS.get(index as usize) else {
            return;
        };
        let value = (raw / parameter.step).round() * parameter.step;
        window
            .get_parameter_values()
            .set_row_data(index as usize, value);
        if window.get_live_preview() {
            let next = generation_for_change.get().wrapping_add(1);
            generation_for_change.set(next);
            let _ = tx.send(next);
        }
    });

    let weak = window.as_weak();
    let path = user_shader.clone();
    window.on_apply_now(move || {
        let Some(window) = weak.upgrade() else {
            return;
        };
        match apply(&window, &path) {
            Ok(()) => window.set_status_text("已应用 / Applied".into()),
            Err(error) => window.set_status_text(format!("应用失败: {error}").into()),
        }
    });

    let weak = window.as_weak();
    let path = user_shader.clone();
    window.on_toggle_effect(move |enabled| {
        let result = if enabled {
            apply(&weak.upgrade().unwrap(), &path)
        } else {
            set_effect(false, &path)
        };
        set_status(
            &weak,
            if result.is_ok() {
                if enabled {
                    "已启用 / Enabled"
                } else {
                    "已关闭 / Disabled"
                }
            } else {
                "切换失败 / Toggle failed"
            },
        );
    });

    let weak = window.as_weak();
    let defaults = defaults.clone();
    let path = user_shader.clone();
    window.on_reset_defaults(move || {
        let Some(window) = weak.upgrade() else {
            return;
        };
        window.set_parameter_values(ModelRc::new(VecModel::from(defaults.clone())));
        match apply(&window, &path) {
            Ok(()) => window.set_status_text("已恢复默认 / Defaults restored".into()),
            Err(error) => window.set_status_text(format!("恢复失败: {error}").into()),
        }
    });

    let weak = window.as_weak();
    window.on_open_folder(
        move || match Command::new("xdg-open").arg(&user_dir).spawn() {
            Ok(_) => set_status(&weak, "已打开目录 / Folder opened"),
            Err(_) => set_status(&weak, "无法打开目录 / Open failed"),
        },
    );

    window.run().map_err(|e| e.to_string())
}

thread_local! {
    static APP_WEAK: std::cell::RefCell<slint::Weak<AppWindow>> = Default::default();
}

fn run_toggle() -> Result<(), String> {
    if env::var_os("HYPRLAND_INSTANCE_SIGNATURE").is_none() {
        return Err("not running inside a Hyprland session".into());
    }
    let enabled = detect_enabled();
    let shader = source_shader()?;
    set_effect(!enabled, &shader)?;
    let state = if enabled { "OFF" } else { "ON" };
    println!("CRT Shader: {state} (temporary)");
    let color = if enabled {
        "rgb(88cc88)"
    } else {
        "rgb(88aaff)"
    };
    let _ = hyprctl(&[
        "notify",
        "2",
        "2500",
        color,
        &format!("CRT Shader: {state}"),
    ]);
    Ok(())
}

fn main() {
    let toggle = env::args_os()
        .next()
        .and_then(|p| PathBuf::from(p).file_name().map(|n| n.to_owned()))
        .map(|name| name == "hypr-crt-toggle")
        .unwrap_or(false);
    let result = if toggle { run_toggle() } else { run() };
    if let Err(error) = result {
        eprintln!(
            "{}: {error}",
            if toggle {
                "hypr-crt-toggle"
            } else {
                "hypr-crt-control"
            }
        );
        std::process::exit(1);
    }
}
