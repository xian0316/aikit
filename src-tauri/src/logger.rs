use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

use crate::paths;
use crate::store;

fn log_file() -> PathBuf {
    paths::logs_dir().join("aikit.log")
}

/// 日志级别权重
fn level_weight(level: &str) -> u8 {
    match level {
        "error" => 0,
        "warn" => 1,
        "info" => 2,
        "debug" => 3,
        "trace" => 4,
        _ => 2,
    }
}

/// 写日志
pub fn log(level: &str, action: &str, detail: Value) {
    let settings = store::get_settings();

    let log_enabled = settings
        .get("logEnabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    if !log_enabled {
        return;
    }

    let config_level = settings
        .get("logLevel")
        .and_then(|v| v.as_str())
        .unwrap_or("info");
    if level_weight(level) > level_weight(config_level) {
        return;
    }

    let entry = json!({
        "time": chrono_now(),
        "level": level,
        "action": action,
        "detail": detail,
    });

    let line = format!("{}\n", entry);
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file())
        .and_then(|mut f| {
            use std::io::Write;
            f.write_all(line.as_bytes())
        });
}

/// 读取日志
pub fn read_logs(limit: usize, level_filter: Option<&str>) -> Vec<Value> {
    let path = log_file();
    if !path.exists() {
        return vec![];
    }

    let content = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(_) => return vec![],
    };

    let mut entries: Vec<Value> = content
        .trim()
        .lines()
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();

    if let Some(lf) = level_filter {
        if lf != "all" {
            entries.retain(|e| e.get("level").and_then(|v| v.as_str()) == Some(lf));
        }
    }

    entries.sort_by(|a, b| {
        let ta = a.get("time").and_then(|v| v.as_str()).unwrap_or("");
        let tb = b.get("time").and_then(|v| v.as_str()).unwrap_or("");
        tb.cmp(ta)
    });

    entries.truncate(limit);
    entries
}

/// 清空日志
pub fn clear_logs() -> bool {
    fs::write(log_file(), "").is_ok()
}

/// 简易时间戳（不依赖 chrono crate）
fn chrono_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // ISO 格式近似值（足够排序）
    format!("{}", secs)
}
