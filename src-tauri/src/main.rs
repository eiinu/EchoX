#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

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
        .run(tauri::generate_context!())
        .expect("error while running EchoX");
}
