use serde_json::{json, Value};
use std::fs;
use std::sync::Mutex;

use crate::paths;

/// 全局配置（运行时缓存在内存中）
static CONFIG: Mutex<Option<Value>> = Mutex::new(None);

/// 默认配置
fn default_config() -> Value {
    json!({
        "repos": [],
        "installed": [],
        "settings": {
            "installMode": "symlink",
            "logEnabled": true,
            "logLevel": "info",
            "toolDirs": {}
        }
    })
}

/// 加载配置（启动时调用一次）
pub fn load() {
    let path = paths::config_file();
    let config = if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_else(default_config)
    } else {
        default_config()
    };
    *CONFIG.lock().unwrap() = Some(config);
}

/// 保存配置到磁盘
pub fn save() {
    let config = CONFIG.lock().unwrap();
    if let Some(ref c) = *config {
        paths::ensure_dir(&paths::app_dir());
        let json_str = serde_json::to_string_pretty(c).unwrap_or_default();
        let _ = fs::write(paths::config_file(), json_str);
    }
}

/// 读取配置值（浅拷贝）
pub fn get() -> Value {
    let config = CONFIG.lock().unwrap();
    config.clone().unwrap_or_else(default_config)
}

/// 获取子键
pub fn get_key(key: &str) -> Value {
    get().get(key).cloned().unwrap_or(Value::Null)
}

/// 设置子键并保存
pub fn set_key(key: &str, value: Value) {
    let mut config = get();
    if let Some(obj) = config.as_object_mut() {
        obj.insert(key.to_string(), value);
    }
    *CONFIG.lock().unwrap() = Some(config);
    save();
}

/// 获取 settings 子对象
pub fn get_settings() -> Value {
    get_key("settings")
}

/// 设置 settings 部分字段（合并）
pub fn set_settings(partial: Value) {
    let mut settings = get_settings();
    if let (Some(s), Some(p)) = (settings.as_object_mut(), partial.as_object()) {
        for (k, v) in p {
            s.insert(k.clone(), v.clone());
        }
    }
    set_key("settings", settings);
}
