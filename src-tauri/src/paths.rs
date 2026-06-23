use std::path::PathBuf;
use std::process::Command;

/// 创建一个已隐藏控制台窗口的子进程 Command（Windows 专用优化）
/// 在 Windows 上，默认会为子进程弹出一个 cmd 黑框，影响体验。
/// 通过 CREATE_NO_WINDOW (0x08000000) 标志隐藏它。
pub fn silent_cmd(program: &str) -> Command {
    let mut cmd = Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    cmd
}

/// 获取用户 home 目录 (Windows: C:\Users\<user>)
pub fn home_dir() -> PathBuf {
    PathBuf::from(std::env::var("USERPROFILE").unwrap_or_else(|_| {
        std::env::var("HOME").unwrap_or_else(|_| "C:\\".to_string())
    }))
}

/// 工具自身数据目录 ~/.aikit
pub fn app_dir() -> PathBuf {
    home_dir().join(".aikit")
}

pub fn repos_dir() -> PathBuf {
    app_dir().join("repos")
}

/// SSOT 统一技能目录（技能先装到这里，再分发到各工具）
pub fn ssot_dir() -> PathBuf {
    app_dir().join("skills")
}

pub fn backups_dir() -> PathBuf {
    app_dir().join("backups")
}

pub fn logs_dir() -> PathBuf {
    app_dir().join("logs")
}

pub fn config_file() -> PathBuf {
    app_dir().join("config.json")
}

/// AI 工具信息
pub struct Tool {
    pub id: &'static str,
    pub name: &'static str,
}

pub const TOOLS: &[Tool] = &[
    Tool { id: "claude-code", name: "Claude Code" },
    Tool { id: "opencode",    name: "OpenCode" },
    Tool { id: "openclaw",    name: "OpenClaw" },
    Tool { id: "codex",       name: "Codex" },
    Tool { id: "hermes",      name: "Hermes" },
    Tool { id: "trae-cn",     name: "Trae CN" },
];

/// 检测工具是否已安装
pub fn is_tool_installed(tool_id: &str) -> bool {
    match tool_id {
        "claude-code" => check_cmd("claude") || check_dir(".claude"),
        "opencode"    => check_cmd("opencode")
                         || check_nested_dir(&[".config", "opencode"])
                         || check_nested_dir(&[".local", "share", "opencode"]),
        "openclaw"    => check_cmd("openclaw") || check_dir(".openclaw"),
        "codex"       => check_cmd("codex") || check_dir(".codex"),
        "hermes"      => check_cmd("hermes") || check_dir(".hermes"),
        "trae-cn"     => check_dir(".trae-cn") || check_appdata_dir("Trae CN"),
        _ => false,
    }
}

/// 检查命令是否在 PATH 中
fn check_cmd(cmd: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        silent_cmd("where")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
    #[cfg(not(target_os = "windows"))]
    {
        silent_cmd("which")
            .arg(cmd)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

/// 检查 home 目录下是否有某个隐藏目录
fn check_dir(dir_name: &str) -> bool {
    home_dir().join(dir_name).exists()
}

/// 检查 home 目录下多层嵌套目录是否存在（如 .config/opencode）
fn check_nested_dir(parts: &[&str]) -> bool {
    let mut p = home_dir();
    for part in parts { p = p.join(part); }
    p.exists()
}

/// 检查 AppData/Roaming 下是否有某个目录
fn check_appdata_dir(dir_name: &str) -> bool {
    if let Ok(appdata) = std::env::var("APPDATA") {
        PathBuf::from(appdata).join(dir_name).exists()
    } else {
        false
    }
}

/// 各工具的默认配置根目录
pub fn default_tool_base_dir(tool_id: &str) -> PathBuf {
    let home = home_dir();
    match tool_id {
        "claude-code" => home.join(".claude"),
        "opencode"    => home.join(".config").join("opencode"),
        "openclaw"    => home.join(".openclaw"),
        "codex"       => home.join(".codex"),
        "hermes"      => home.join(".hermes"),
        "trae-cn"     => home.join(".trae-cn"),
        _ => home,
    }
}

/// 获取工具的 skills 目录（考虑自定义路径）
pub fn get_tool_skills_dir(tool_id: &str, custom_dirs: &serde_json::Value) -> PathBuf {
    let base = if let Some(custom) = custom_dirs.get(tool_id) {
        if let Some(s) = custom.as_str() {
            PathBuf::from(s)
        } else {
            default_tool_base_dir(tool_id)
        }
    } else {
        default_tool_base_dir(tool_id)
    };
    base.join("skills")
}

/// 确保目录存在
pub fn ensure_dir(dir: &std::path::Path) {
    if !dir.exists() {
        std::fs::create_dir_all(dir).ok();
    }
}

/// 初始化所有应用目录
pub fn init_app_dirs() {
    ensure_dir(&app_dir());
    ensure_dir(&repos_dir());
    ensure_dir(&ssot_dir());
    ensure_dir(&backups_dir());
    ensure_dir(&logs_dir());
}
