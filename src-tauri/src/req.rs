use reqwest::blocking::Client;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::Deserialize;
use tauri::{AppHandle, Emitter, Manager};
use std::thread;
use std::time::Duration;
use enigo::{Mouse, Settings};

#[derive(Deserialize)]
pub struct TranslationResponse {
    #[allow(dead_code)]
    pub from: String,
    #[allow(dead_code)]
    pub to: String,
    #[serde(rename = "tgtText")]
    pub tgt_text: String,
}

pub fn fetch_translation(api_key: &str, app_id: &str, src_text: &str) -> Result<String, Box<dyn std::error::Error>> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs().to_string();

    let param_str = format!(
        "apikey={}&appId={}&from=en&srcText={}&timestamp={}&to=zh",
        api_key, app_id, src_text, timestamp
    );
    let auth_str = format!("{:x}", md5::compute(param_str));

    let client = Client::new();
    let params = [
        ("from", "en"),
        ("to", "zh"),
        ("appId", app_id),
        ("timestamp", &timestamp),
        ("srcText", src_text),
        ("authStr", &auth_str),
    ];

    let res = client.post("https://api.niutrans.com/v2/text/translate")
        .form(&params)
        .send()?
        .json::<TranslationResponse>()?;

    Ok(res.tgt_text)
}

pub fn handle_translation_flow(handle: AppHandle) {
    // 开启异步线程，避免阻塞 UI
    thread::spawn(move || {
        let window = handle.get_webview_window("main").unwrap();
        let result = (|| -> Result<String, Box<dyn std::error::Error>> {
            let old_content = crate::read_clipboard().unwrap_or_default();
            println!("Handle_translation_flow started");
            // 1. 模拟复制按键
            crate::copy_content()?; 

            // 2. 等待系统写入剪贴板 (150ms 是经验安全值)
            thread::sleep(Duration::from_millis(150));

            // 3. 读取剪贴板文字
            let text = crate::read_clipboard()?;

            let _ = crate::set_clipboard(&old_content);
            
            // 如果内容为空，直接返回提示
            if text.trim().is_empty() {
                return Err("未检测到选中文本".into());
            }
            let enigo = enigo::Enigo::new(&Settings::default())?;
            let (mouse_x, mouse_y) = enigo.location()?;

            let _ = window.set_position(tauri::PhysicalPosition::new(mouse_x + 10, mouse_y+10));
            let _ = window.show();
            let _ = window.set_focus();

            // 4. 调用 API 翻译
            // 假设你的 fetch_translation 在本模块中
            let key = crate::config::ConfigManager::get_api_key(&handle);
            let app_id = crate::config::ConfigManager::get_app_id(&handle);
            if key.is_empty() {
                return Err("API Key is empty".into());
            }
            let translated = crate::req::fetch_translation(&key, &app_id, &text)?;
            
            Ok(translated)
        })();

        // 5. 统一处理结果并发送给前端
        match result {
            Ok(msg) => {
                let _ = handle.emit("shortcut-event", msg);
            }
            Err(e) => {
                let _ = handle.emit("shortcut-event", format!("错误: {}", e));
            }
        }
    });
}

