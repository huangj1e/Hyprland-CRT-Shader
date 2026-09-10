use slint::{Color, Model, ModelRc, SharedString, VecModel};
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
    labels: [&'static str; 4],
    min: f32,
    max: f32,
    step: f32,
    group: i32,
}

const LANGUAGES: &[&str] = &["中文", "English", "日本語", "한국어"];
const GROUPS: [[&str; 4]; 5] = [
    ["信号故障", "Signal glitch", "信号グリッチ", "신호 글리치"],
    [
        "抖动与波浪",
        "Shake and waves",
        "揺れと波形",
        "흔들림과 파동",
    ],
    [
        "滚动同步撕裂",
        "Rolling sync tear",
        "ローリング同期",
        "롤링 동기 찢김",
    ],
    ["RGB 与噪声", "RGB and noise", "RGBとノイズ", "RGB와 노이즈"],
    ["CRT 显示", "CRT display", "CRT表示", "CRT 디스플레이"],
];
const UI_TEXT: [[&str; 4]; 9] = [
    [
        "Hyprland CRT Shader 控制面板",
        "Hyprland CRT Shader Control",
        "Hyprland CRT Shader コントロール",
        "Hyprland CRT Shader 제어판",
    ],
    ["语言", "Language", "言語", "언어"],
    [
        "实时预览",
        "Live preview",
        "ライブプレビュー",
        "실시간 미리보기",
    ],
    ["启用效果", "Effect enabled", "エフェクト有効", "효과 사용"],
    [
        "修改用户 Shader 副本；实时预览会在短暂防抖后重新编译。",
        "Edit the user shader copy; live preview recompiles after a short debounce.",
        "ユーザー用Shaderを編集し、短い待機後に再コンパイルします。",
        "사용자 Shader를 편집하며 짧은 지연 후 다시 컴파일합니다.",
    ],
    ["立即应用", "Apply now", "今すぐ適用", "지금 적용"],
    [
        "恢复默认",
        "Reset defaults",
        "デフォルトに戻す",
        "기본값 복원",
    ],
    [
        "打开 Shader 目录",
        "Open Shader folder",
        "Shaderフォルダーを開く",
        "Shader 폴더 열기",
    ],
    [
        "Ctrl+R：恢复默认参数　Ctrl+Shift+E：紧急关闭特效",
        "Ctrl+R: reset defaults   Ctrl+Shift+E: emergency disable",
        "Ctrl+R：デフォルト　Ctrl+Shift+E：緊急無効化",
        "Ctrl+R: 기본값　Ctrl+Shift+E: 긴급 비활성화",
    ],
];

const PARAMETERS: &[Parameter] = &[
    Parameter {
        key: "GLITCH_INTERVAL",
        labels: ["故障周期", "Glitch interval", "グリッチ間隔", "글리치 간격"],
        min: 1.0,
        max: 120.0,
        step: 0.5,
        group: 0,
    },
    Parameter {
        key: "GLITCH_DURATION",
        labels: [
            "故障持续时间",
            "Glitch duration",
            "グリッチ持続時間",
            "글리치 지속 시간",
        ],
        min: 0.05,
        max: 3.0,
        step: 0.05,
        group: 0,
    },
    Parameter {
        key: "GLITCH_POWER",
        labels: [
            "周期性故障强度",
            "Glitch power",
            "グリッチ強度",
            "글리치 강도",
        ],
        min: 0.0,
        max: 1.5,
        step: 0.01,
        group: 0,
    },
    Parameter {
        key: "BASE_GLITCH",
        labels: [
            "常态信号不稳",
            "Base instability",
            "基本信号の不安定さ",
            "기본 신호 불안정",
        ],
        min: 0.0,
        max: 0.6,
        step: 0.01,
        group: 0,
    },
    Parameter {
        key: "SHAKE_BASE_PIXELS",
        labels: [
            "常态抖动像素",
            "Idle shake",
            "常態シェイク",
            "평상시 흔들림",
        ],
        min: 0.0,
        max: 4.0,
        step: 0.05,
        group: 1,
    },
    Parameter {
        key: "SHAKE_GLITCH_PIXELS",
        labels: [
            "故障抖动像素",
            "Glitch shake",
            "グリッチシェイク",
            "글리치 흔들림",
        ],
        min: 0.0,
        max: 20.0,
        step: 0.1,
        group: 1,
    },
    Parameter {
        key: "WAVE_BASE_PIXELS",
        labels: ["常态水平波纹", "Idle wave", "常態波形", "평상시 파동"],
        min: 0.0,
        max: 5.0,
        step: 0.05,
        group: 1,
    },
    Parameter {
        key: "WAVE_GLITCH_PIXELS",
        labels: ["故障水平波纹", "Glitch wave", "グリッチ波形", "글리치 파동"],
        min: 0.0,
        max: 30.0,
        step: 0.1,
        group: 1,
    },
    Parameter {
        key: "ROLLING_TEAR_STRENGTH",
        labels: [
            "滚动撕裂强度",
            "Rolling tear strength",
            "ローリング強度",
            "롤링 찢김 강도",
        ],
        min: 0.0,
        max: 2.0,
        step: 0.01,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_WIDTH",
        labels: ["撕裂带高度比例", "Tear band width", "ティア幅", "찢김 폭"],
        min: 0.005,
        max: 0.30,
        step: 0.005,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_SPEED",
        labels: [
            "向下滚动速度",
            "Rolling speed",
            "スクロール速度",
            "롤링 속도",
        ],
        min: 0.0,
        max: 2.0,
        step: 0.01,
        group: 2,
    },
    Parameter {
        key: "ROLLING_TEAR_PIXELS",
        labels: ["撕裂水平错位", "Tear displacement", "水平ずれ", "수평 변위"],
        min: 0.0,
        max: 50.0,
        step: 0.25,
        group: 2,
    },
    Parameter {
        key: "RGB_SHIFT_BASE_PIXELS",
        labels: [
            "常态 RGB 分离",
            "Idle RGB shift",
            "常態RGBずれ",
            "평상시 RGB 분리",
        ],
        min: 0.0,
        max: 8.0,
        step: 0.05,
        group: 3,
    },
    Parameter {
        key: "RGB_SHIFT_GLITCH",
        labels: [
            "故障 RGB 分离",
            "Glitch RGB shift",
            "グリッチRGBずれ",
            "글리치 RGB 분리",
        ],
        min: 0.0,
        max: 40.0,
        step: 0.25,
        group: 3,
    },
    Parameter {
        key: "BLOCK_NOISE_STRENGTH",
        labels: [
            "块状噪声强度",
            "Block noise",
            "ブロックノイズ",
            "블록 노이즈",
        ],
        min: 0.0,
        max: 1.0,
        step: 0.01,
        group: 3,
    },
    Parameter {
        key: "WHITE_NOISE_STRENGTH",
        labels: [
            "白噪点强度",
            "White noise",
            "ホワイトノイズ",
            "화이트 노이즈",
        ],
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 3,
    },
    Parameter {
        key: "HORIZONTAL_LINE",
        labels: [
            "高频水平暗纹",
            "Horizontal noise",
            "水平ノイズ",
            "수평 노이즈",
        ],
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 3,
    },
    Parameter {
        key: "CURVATURE",
        labels: ["屏幕曲率", "Curvature", "曲率", "곡률"],
        min: 0.0,
        max: 0.20,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "SCANLINE_STRENGTH",
        labels: ["扫描线强度", "Scanline strength", "走査線", "스캔라인"],
        min: 0.0,
        max: 0.40,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "RGB_MASK_STRENGTH",
        labels: [
            "RGB 荧光粉强度",
            "Phosphor mask",
            "RGBマスク",
            "RGB 인광 마스크",
        ],
        min: 0.0,
        max: 0.30,
        step: 0.005,
        group: 4,
    },
    Parameter {
        key: "VIGNETTE_STRENGTH",
        labels: ["暗角强度", "Vignette", "ビネット", "비네트"],
        min: 0.0,
        max: 0.70,
        step: 0.01,
        group: 4,
    },
    Parameter {
        key: "FLICKER_STRENGTH",
        labels: ["亮度闪烁", "Flicker", "ちらつき", "플리커"],
        min: 0.0,
        max: 0.08,
        step: 0.001,
        group: 4,
    },
    Parameter {
        key: "OVERSCAN",
        labels: ["画面裁边/缩小", "Overscan", "オーバースキャン", "오버스캔"],
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

fn parse_color(value: &str, fallback: Color) -> Color {
    let hex = value.trim().trim_start_matches('#');
    if hex.len() != 6 {
        return fallback;
    }
    u32::from_str_radix(hex, 16)
        .ok()
        .map(|rgb| Color::from_rgb_u8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8))
        .unwrap_or(fallback)
}

fn omarchy_colors() -> (Color, Color, Color, Color, Color) {
    let defaults = ["#8dc7ff", "#11151c", "#202936", "#eef4ff", "#9daabe"];
    let mut values = defaults.map(str::to_owned);
    if let Some(home) = env::var_os("HOME") {
        let path = PathBuf::from(home).join(".local/state/omarchy/current/theme/colors.toml");
        if let Ok(text) = fs::read_to_string(path) {
            for line in text.lines() {
                let Some((key, rest)) = line.split_once('=') else {
                    continue;
                };
                let value = rest
                    .split('"')
                    .nth(1)
                    .or_else(|| rest.split_whitespace().next())
                    .unwrap_or("");
                let slot = match key.trim() {
                    "accent" => Some(0),
                    "background" => Some(1),
                    "foreground" => Some(3),
                    "color8" => Some(4),
                    _ => None,
                };
                if let Some(slot) = slot {
                    values[slot] = value.to_owned();
                }
            }
        }
    }
    if let Ok(accent) = env::var("OMARCHY_ACCENT") {
        values[0] = accent;
    }
    let colors = values
        .iter()
        .zip(defaults)
        .map(|(value, fallback)| parse_color(value, parse_color(fallback, Color::default())))
        .collect::<Vec<_>>();
    (
        colors[0],
        colors[1],
        colors[1].brighter(0.08),
        colors[3],
        colors[4],
    )
}

fn apply_theme(window: &AppWindow) {
    let (accent, background, panel, text, muted) = omarchy_colors();
    window.set_accent_color(accent);
    window.set_window_background(background);
    window.set_panel_color(panel);
    window.set_text_color(text);
    window.set_muted_color(muted);
}

fn localized_text(index: usize, language: usize) -> SharedString {
    UI_TEXT[index][language.min(3)].into()
}

fn update_language(window: &AppWindow, language: usize) {
    let language = language.min(3);
    window.set_language_index(language as i32);
    window.set_window_title(localized_text(0, language));
    window.set_language_label(localized_text(1, language));
    window.set_live_preview_label(localized_text(2, language));
    window.set_enabled_label(localized_text(3, language));
    window.set_description_text(localized_text(4, language));
    window.set_apply_label(localized_text(5, language));
    window.set_reset_label(localized_text(6, language));
    window.set_folder_label(localized_text(7, language));
    window.set_shortcut_label(localized_text(8, language));
    window.set_group_titles(ModelRc::new(VecModel::from(
        GROUPS
            .iter()
            .map(|group| group[language].into())
            .collect::<Vec<SharedString>>(),
    )));
    window.set_parameter_labels(ModelRc::new(VecModel::from(
        PARAMETERS
            .iter()
            .map(|p| p.labels[language].into())
            .collect::<Vec<SharedString>>(),
    )));
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
    window.set_languages(ModelRc::new(VecModel::from(
        LANGUAGES
            .iter()
            .map(|s| (*s).into())
            .collect::<Vec<SharedString>>(),
    )));
    window.set_parameter_minimums(ModelRc::new(VecModel::from(
        PARAMETERS.iter().map(|p| p.min).collect::<Vec<_>>(),
    )));
    window.set_parameter_maximums(ModelRc::new(VecModel::from(
        PARAMETERS.iter().map(|p| p.max).collect::<Vec<_>>(),
    )));
    window.set_parameter_group_starts(ModelRc::new(VecModel::from(
        (0..GROUPS.len())
            .map(|group| {
                PARAMETERS
                    .iter()
                    .position(|p| p.group == group as i32)
                    .unwrap_or(0) as i32
            })
            .collect::<Vec<_>>(),
    )));
    window.set_parameter_group_counts(ModelRc::new(VecModel::from(
        (0..GROUPS.len())
            .map(|group| {
                PARAMETERS
                    .iter()
                    .filter(|p| p.group == group as i32)
                    .count() as i32
            })
            .collect::<Vec<_>>(),
    )));
    window.set_parameter_values(ModelRc::new(VecModel::from(values)));
    window.set_effect_enabled(detect_enabled());
    window.set_live_preview(true);
    apply_theme(&window);
    let theme_path = env::var_os("HOME")
        .map(|home| PathBuf::from(home).join(".local/state/omarchy/current/theme/colors.toml"));
    let theme_weak = window.as_weak();
    thread::spawn(move || {
        let mut last_modified = None;
        loop {
            let modified = theme_path
                .as_ref()
                .and_then(|path| fs::metadata(path).ok())
                .and_then(|metadata| metadata.modified().ok());
            if modified.is_some() && modified != last_modified {
                last_modified = modified;
                let weak = theme_weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(window) = weak.upgrade() {
                        apply_theme(&window);
                    }
                });
            }
            thread::sleep(Duration::from_secs(1));
        }
    });
    update_language(&window, 1);

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
    window.on_language_changed({
        let weak = window.as_weak();
        move |index| {
            if let Some(window) = weak.upgrade() {
                update_language(&window, index as usize);
            }
        }
    });

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
    let path = user_shader.clone();
    window.on_emergency_disable(move || {
        if let Err(error) = set_effect(false, &path) {
            set_status(&weak, &format!("Emergency disable failed: {error}"));
        } else if let Some(window) = weak.upgrade() {
            window.set_effect_enabled(false);
            window.set_status_text("Effect disabled / 特效已关闭".into());
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

fn main() {
    if let Err(error) = run() {
        eprintln!("hyprland-crt-shader: {error}");
        std::process::exit(1);
    }
}
