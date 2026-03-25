#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    collections::HashMap,
    env,
    path::PathBuf,
    sync::{Mutex, OnceLock},
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

static SHELL_CWD: OnceLock<Mutex<HashMap<String, PathBuf>>> = OnceLock::new();

fn shell_cwd_map() -> &'static Mutex<HashMap<String, PathBuf>> {
    SHELL_CWD.get_or_init(|| Mutex::new(HashMap::new()))
}

fn escape_for_single_quotes(raw: &str) -> String {
    raw.replace('\'', "'\"'\"'")
}

#[tauri::command]
fn execute_command(command: String, shell: String) -> Result<String, String> {
    let trimmed = command.trim();
    if trimmed.is_empty() {
        return Err("命令不能为空".to_string());
    }

    let (shell_binary, init_script) = match shell.trim() {
        "zsh" => (
            "zsh",
            "if [ -f ~/.zprofile ]; then source ~/.zprofile; fi\nif [ -f ~/.zshrc ]; then source ~/.zshrc; fi",
        ),
        _ => (
            "bash",
            "if [ -f ~/.bash_profile ]; then source ~/.bash_profile; fi\nif [ -f ~/.bashrc ]; then source ~/.bashrc; fi",
        ),
    };

    let shell_key = shell.trim().to_string();
    let cwd = {
        let mut cwd_map = shell_cwd_map()
            .lock()
            .map_err(|_| "无法读取终端状态，请重试".to_string())?;

        let entry = cwd_map
            .entry(shell_key.clone())
            .or_insert_with(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        entry.clone()
    };
    let cwd_escaped = escape_for_single_quotes(&cwd.to_string_lossy());
    let marker = "__ECHOX_CWD_MARKER__";

    let wrapped_command = format!(
        "{init_script}\ncd '{cwd_escaped}' || exit 1\n{trimmed}\nstatus=$?\nprintf '\\n{marker}%s\\n' \"$PWD\"\nexit $status"
    );

    let output = std::process::Command::new(shell_binary)
        .arg("-c")
        .arg(wrapped_command)
        .output()
        .map_err(|err| format!("执行失败: {err}"))?;

    let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if let Some(marker_pos) = stdout.rfind(marker) {
        let cwd_start = marker_pos + marker.len();
        let new_cwd = stdout[cwd_start..].trim().to_string();
        stdout.truncate(marker_pos);
        if !new_cwd.is_empty() {
            if let Ok(mut cwd_map) = shell_cwd_map().lock() {
                cwd_map.insert(shell_key, PathBuf::from(new_cwd));
            }
        }
    }

    if output.status.success() {
        if stdout.trim().is_empty() && stderr.trim().is_empty() {
            Ok("(命令执行完成，无输出)".to_string())
        } else {
            Ok(format!("{stdout}{stderr}").trim_end().to_string())
        }
    } else {
        let status = output.status.code().unwrap_or(-1);
        let details = format!("{stdout}{stderr}").trim().to_string();
        if details.is_empty() {
            Err(format!("命令执行失败，退出码: {status}"))
        } else {
            Err(format!("命令执行失败，退出码: {status}\n{details}"))
        }
    }
}

fn build_tray(app: &AppHandle) -> Result<(), tauri::Error> {
    let show = MenuItemBuilder::with_id("show", "显示主窗口").build(app)?;
    let quit = MenuItemBuilder::with_id("quit", "退出 EchoX").build(app)?;
    let menu = MenuBuilder::new(app).items(&[&show, &quit]).build()?;

    let mut tray_builder = TrayIconBuilder::with_id("echox-tray")
        .menu(&menu)
        .tooltip("EchoX 正在后台运行，右键可退出")
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
            "quit" => {
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.unminimize();
                    let _ = window.set_focus();
                }
            }
        });

    if let Some(icon) = app.default_window_icon().cloned() {
        tray_builder = tray_builder.icon(icon);
    }

    tray_builder.build(app)?;

    Ok(())
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            build_tray(app.handle())?;

            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![execute_command])
        .run(tauri::generate_context!())
        .expect("error while running EchoX");
}
