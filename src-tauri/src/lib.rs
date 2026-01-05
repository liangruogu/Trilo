mod tray;
mod req;
mod clip;
mod config;
use tauri::{Manager, Emitter};
use tauri::AppHandle;
use config::ConfigManager;
use tauri_plugin_global_shortcut::{ShortcutState, GlobalShortcutExt};
use req::fetch_translation;
use clip::{read_clipboard, set_clipboard, copy_content};
use std::str::FromStr;

#[tauri::command]
fn save_api_key(app: AppHandle, key: String) -> Result<(), String> {
    ConfigManager::save_api_key(&app, key).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_app_id(app: AppHandle, app_id: String) -> Result<(), String> {
    ConfigManager::save_app_id(&app, app_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_shortcut(app: AppHandle, shortcut: String) -> Result<(), String> {
    let old_shortcut = ConfigManager::get_shortcut(&app);
    
    // 尝试解析新快捷键，确保格式正确
    let shortcut_obj = tauri_plugin_global_shortcut::Shortcut::from_str(&shortcut)
        .map_err(|_| "Invalid shortcut format".to_string())?;

    // 如果新旧快捷键不同，则更新注册
    if old_shortcut != shortcut {
        let _ = app.global_shortcut().unregister_all();
        app.global_shortcut().register(shortcut_obj).map_err(|e| e.to_string())?;
        ConfigManager::save_shortcut(&app, shortcut).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn save_theme(app: AppHandle, theme: String) -> Result<(), String> {
    ConfigManager::save_theme(&app, theme.clone()).map_err(|e| e.to_string())?;
    app.emit("theme-changed", theme).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<serde_json::Value, String> {
    let key = ConfigManager::get_api_key(&app);
    let app_id = ConfigManager::get_app_id(&app);
    let shortcut = ConfigManager::get_shortcut(&app);
    let theme = ConfigManager::get_theme(&app);
    Ok(serde_json::json!({
        "apiKey": key,
        "appId": app_id,
        "shortcut": shortcut,
        "theme": theme
    }))
}

#[tauri::command]
async fn translate(app: tauri::AppHandle, text: String) -> Result<String, String> {
    let key = ConfigManager::get_api_key(&app);
    let app_id = ConfigManager::get_app_id(&app);
    if key.is_empty() {
        return Err("API Key is empty".to_string());
    }
    // 这里执行你的翻译逻辑...
    let translated_text = fetch_translation(&key, &app_id, &text).map_err(|e| e.to_string())?;
    Ok(translated_text) 
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new()
        .with_handler(|app, shortcut, event|{
            if event.state == ShortcutState::Pressed {
                // 获取当前配置的快捷键
                let configured_shortcut_str = ConfigManager::get_shortcut(app);
                // 简单判断：如果触发的快捷键与配置的一致（或者我们只注册了一个快捷键），则执行
                // 这里我们假设只注册了一个快捷键，或者直接执行逻辑
                // 更严谨的做法是比较 shortcut 和 configured_shortcut_str
                if let Ok(configured) = tauri_plugin_global_shortcut::Shortcut::from_str(&configured_shortcut_str) {
                    if shortcut == &configured {
                        crate::req::handle_translation_flow(app.clone());
                    }
                }
            }
        }).build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, Some(vec![])))
        .invoke_handler(tauri::generate_handler![save_api_key, save_app_id, save_shortcut, save_theme, get_settings, translate])
        .setup(|app| {
            tray::create_tray(app.handle())?;
            
            // 注册快捷键
            let shortcut = ConfigManager::get_shortcut(app.handle());
            if !shortcut.is_empty() {
                if let Ok(shortcut_obj) = tauri_plugin_global_shortcut::Shortcut::from_str(&shortcut) {
                    if let Err(e) = app.handle().global_shortcut().register(shortcut_obj) {
                        eprintln!("Failed to register shortcut '{}': {}", shortcut, e);
                        // 注册失败时不崩溃，允许用户后续手动修改
                    }
                }
            }

            let window = app.get_webview_window("main").unwrap();
            let w = window.clone();
            window.on_window_event(move |event| {
                if let tauri::WindowEvent::Focused(false) = event {
                    let _ = w.hide();
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
