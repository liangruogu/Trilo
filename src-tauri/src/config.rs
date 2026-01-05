use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt; 
use serde_json::json;

pub struct ConfigManager;

impl ConfigManager {
    const STORE_PATH: &'static str = "settings.json";
    const KEY_NAME: &'static str = "api-key";
    const APP_ID_NAME: &'static str = "app-id";
    const THEME_KEY: &'static str = "theme";
    const SHORTCUT_KEY: &'static str = "shortcut";

    pub fn save_api_key<R: Runtime>(app: &AppHandle<R>, key: String) -> Result<(), String> {
        // 获取 store 实例
        let store = app.store(ConfigManager::STORE_PATH)
            .map_err(|e| e.to_string())?;
        
        // 设置值
        store.set(ConfigManager::KEY_NAME.to_string(), json!({ "value": key }));
        
        // 物理保存，手动将插件错误转为 String
        store.save().map_err(|e| e.to_string())?;
        
        Ok(())
    }

    pub fn get_api_key<R: Runtime>(app: &AppHandle<R>) -> String {
        if let Ok(store) = app.store(ConfigManager::STORE_PATH) {
            if let Some(v) = store.get(ConfigManager::KEY_NAME) {
                return v.get("value")
                    .and_then(|val| val.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }
        "".to_string()
    }

    pub fn save_app_id<R: Runtime>(app: &AppHandle<R>, app_id: String) -> Result<(), String> {
        let store = app.store(ConfigManager::STORE_PATH)
            .map_err(|e| e.to_string())?;
        store.set(ConfigManager::APP_ID_NAME.to_string(), json!({ "value": app_id }));
        store.save().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_app_id<R: Runtime>(app: &AppHandle<R>) -> String {
        if let Ok(store) = app.store(ConfigManager::STORE_PATH) {
            if let Some(v) = store.get(ConfigManager::APP_ID_NAME) {
                return v.get("value")
                    .and_then(|val| val.as_str())
                    .unwrap_or("")
                    .to_string();
            }
        }
        "".to_string()
    }

    pub fn save_shortcut<R: Runtime>(app: &AppHandle<R>, shortcut: String) -> Result<(), String> {
        let store = app.store(ConfigManager::STORE_PATH)
            .map_err(|e| e.to_string())?;
        store.set(ConfigManager::SHORTCUT_KEY.to_string(), json!({ "value": shortcut }));
        store.save().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_shortcut<R: Runtime>(app: &AppHandle<R>) -> String {
        if let Ok(store) = app.store(ConfigManager::STORE_PATH) {
            if let Some(v) = store.get(ConfigManager::SHORTCUT_KEY) {
                return v.get("value")
                    .and_then(|val| val.as_str())
                    .unwrap_or("Ctrl+Q")
                    .to_string();
            }
        }
        "Ctrl+Q".to_string()
    }

    pub fn save_theme<R: Runtime>(app: &AppHandle<R>, theme: String) -> Result<(), String> {
        let store = app.store(ConfigManager::STORE_PATH)
            .map_err(|e| e.to_string())?;
        store.set(ConfigManager::THEME_KEY.to_string(), json!({ "value": theme }));
        store.save().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_theme<R: Runtime>(app: &AppHandle<R>) -> String {
        if let Ok(store) = app.store(ConfigManager::STORE_PATH) {
            if let Some(v) = store.get(ConfigManager::THEME_KEY) {
                return v.get("value")
                    .and_then(|val| val.as_str())
                    .unwrap_or("system")
                    .to_string();
            }
        }
        "system".to_string()
    }
}