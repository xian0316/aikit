use serde_json::{json, Value};
use std::fs;
use std::path::Path;

use crate::git;
use crate::logger;
use crate::paths;
use crate::skills;
use crate::store;

// ═══ 仓库管理 ═══

#[tauri::command]
pub async fn repo_list() -> Result<Value, String> {
    Ok(store::get_key("repos"))
}

#[tauri::command]
pub async fn repo_add(
    url: String,
    branch: Option<String>,
    subdir: Option<String>,
) -> Result<Value, String> {
    let branch = branch.unwrap_or_else(|| "main".to_string());

    // 从 URL 解析 owner 和 name
    let (owner, name) = parse_github_url(&url)?;

    let repos = store::get_key("repos");
    let id = format!("{}__{}", owner, name);

    // 去重
    if let Some(arr) = repos.as_array() {
        for r in arr {
            if r.get("id").and_then(|v| v.as_str()) == Some(&id) {
                return Err("该仓库已存在".to_string());
            }
        }
    }

    git::clone_repo(&owner, &name, &branch, &id)?;

    let repo = json!({
        "id": id,
        "owner": owner,
        "name": name,
        "branch": branch,
        "subdir": subdir.unwrap_or_default(),
        "addedAt": format!("{}", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs()).unwrap_or(0))
    });

    let mut repos_arr = repos.clone();
    if let Some(arr) = repos_arr.as_array_mut() {
        arr.push(repo.clone());
    }
    store::set_key("repos", repos_arr);

    logger::log("info", "repo-add", json!({ "owner": owner, "name": name }));
    Ok(repo)
}

/// 从 GitHub URL 解析 owner 和 name
fn parse_github_url(url: &str) -> Result<(String, String), String> {
    // 匹配 github.com/owner/name
    if let Some(pos) = url.find("github.com/") {
        let rest = &url[pos + 11..];
        let rest = rest.split(['?', '#']).next().unwrap_or(rest);
        let parts: Vec<&str> = rest.split('/').filter(|s| !s.is_empty()).collect();
        if parts.len() >= 2 {
            let owner = parts[0].to_string();
            let name = parts[1].trim_end_matches(".git").to_string();
            return Ok((owner, name));
        }
    }

    // 支持 owner/name 简写
    let trimmed = url.trim();
    let parts: Vec<&str> = trimmed.split('/').filter(|s| !s.is_empty()).collect();
    if parts.len() == 2 && !parts[0].contains(':') && !parts[0].contains('.') {
        let owner = parts[0].to_string();
        let name = parts[1].trim_end_matches(".git").to_string();
        return Ok((owner, name));
    }

    Err("无法解析 GitHub 地址".to_string())
}

#[tauri::command]
pub async fn repo_delete(repo_id: String) -> Result<(), String> {
    let repos = store::get_key("repos");
    let mut filtered = json!([]);
    if let (Some(arr), Some(out)) = (repos.as_array(), filtered.as_array_mut()) {
        for r in arr {
            if r.get("id").and_then(|v| v.as_str()) != Some(&repo_id) {
                out.push(r.clone());
            }
        }
    }
    store::set_key("repos", filtered);
    git::delete_repo_cache(&repo_id);
    logger::log("info", "repo-delete", json!({ "repoId": repo_id }));
    Ok(())
}

#[tauri::command]
pub async fn repo_pull(repo_id: String) -> Result<String, String> {
    git::pull_repo(&repo_id)
}

// ═══ 技能扫描 ═══

#[tauri::command]
pub async fn skills_scan(repo_id: String) -> Result<Value, String> {
    let repos = store::get_key("repos");
    let repo = repos.as_array()
        .and_then(|arr| arr.iter().find(|r| r.get("id").and_then(|v| v.as_str()) == Some(&repo_id)))
        .cloned()
        .ok_or("仓库不存在")?;

    let subdir = repo.get("subdir").and_then(|v| v.as_str()).unwrap_or("");
    let skills_list = skills::scan_skills(&repo_id, subdir);
    let mut skills_json = serde_json::to_value(&skills_list).unwrap_or(json!([]));
    let installed = store::get_key("installed");

    // 标记安装状态
    if let Some(arr) = skills_json.as_array_mut() {
        for skill in arr.iter_mut() {
            let skill_id = skill.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let record = installed.as_array()
                .and_then(|a| a.iter().find(|i| i.get("id").and_then(|v| v.as_str()) == Some(skill_id)));

            if let Some(rec) = record {
                skill["installed"] = json!(true);
                skill["installedTools"] = rec.get("tools").cloned().unwrap_or(json!([]));
                let rec_hash = rec.get("hash").and_then(|v| v.as_str()).unwrap_or("");
                let skill_hash = skill.get("hash").and_then(|v| v.as_str()).unwrap_or("");
                skill["hasUpdate"] = json!(rec_hash != skill_hash);
            } else {
                skill["installed"] = json!(false);
                skill["installedTools"] = json!([]);
                skill["hasUpdate"] = json!(false);
            }
        }
    }

    Ok(json!({ "skills": skills_json }))
}

// ═══ 安装/卸载（SSOT 模式）═══

/// 安装技能到 SSOT 统一目录（不选工具）
#[tauri::command]
pub async fn skills_install(skill: Value) -> Result<Value, String> {
    let dir_path = skill.get("dirPath").and_then(|v| v.as_str()).ok_or("缺少技能路径")?;
    let dir_name = skill.get("dirName").and_then(|v| v.as_str()).ok_or("缺少技能名")?;

    // 检查 SSOT 是否已存在
    let ssot_dest = paths::ssot_dir().join(dir_name);
    let already_installed = ssot_dest.exists();

    if already_installed {
        return Ok(json!({ "alreadyInstalled": true }));
    }

    skills::install_to_ssot(dir_path, dir_name)?;

    // 记录到 installed
    let mut installed = store::get_key("installed");
    let skill_id = skill.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let skill_hash = skill.get("hash").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let repo_id = skill.get("repoId").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let ts = format!("{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0));

    let new_record = json!({
        "id": skill_id,
        "repoId": repo_id,
        "skillName": skill.get("name").and_then(|v| v.as_str()).unwrap_or(dir_name),
        "dirName": dir_name,
        "hash": skill_hash,
        "apps": {},  // 空对象，初始不同步到任何工具
        "installedAt": ts,
    });

    if let Some(arr) = installed.as_array_mut() {
        arr.push(new_record);
    }
    store::set_key("installed", installed);

    Ok(json!({ "ok": true }))
}

/// 切换工具同步状态
#[tauri::command]
pub async fn skills_toggle_app(skill_name: String, tool_id: String, enabled: bool) -> Result<(), String> {
    skills::toggle_app(&skill_name, &tool_id, enabled)?;

    // 更新 installed 记录中的 apps
    let mut installed = store::get_key("installed");
    if let Some(arr) = installed.as_array_mut() {
        if let Some(record) = arr.iter_mut().find(|i| i.get("dirName").and_then(|v| v.as_str()) == Some(&skill_name)) {
            if let Some(apps) = record.get_mut("apps").and_then(|v| v.as_object_mut()) {
                apps.insert(tool_id, json!(enabled));
            }
        }
    }
    store::set_key("installed", installed);

    Ok(())
}

/// 从 SSOT 卸载
#[tauri::command]
pub async fn skills_uninstall(skill_name: String) -> Result<(), String> {
    skills::uninstall_from_ssot(&skill_name)?;

    // 从 installed 记录中删除
    let mut installed = store::get_key("installed");
    if let Some(arr) = installed.as_array_mut() {
        arr.retain(|i| i.get("dirName").and_then(|v| v.as_str()) != Some(&skill_name));
    }
    store::set_key("installed", installed);

    Ok(())
}

// ═══ 更新检测 ═══

#[tauri::command]
pub async fn skills_check_updates() -> Result<Value, String> {
    let repos = store::get_key("repos");
    let installed = store::get_key("installed");
    let mut updates = Vec::new();

    if let Some(arr) = repos.as_array() {
        for repo in arr {
            let repo_id = repo.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let subdir = repo.get("subdir").and_then(|v| v.as_str()).unwrap_or("");
            let scanned = skills::scan_skills(repo_id, subdir);

            for skill in scanned.iter() {
                let skill_id = skill.get("id").and_then(|v| v.as_str()).unwrap_or("");
                if let Some(inst_arr) = installed.as_array() {
                    if let Some(rec) = inst_arr.iter().find(|i| i.get("id").and_then(|v| v.as_str()) == Some(skill_id)) {
                        let rec_hash = rec.get("hash").and_then(|v| v.as_str()).unwrap_or("");
                        let skill_hash = skill.get("hash").and_then(|v| v.as_str()).unwrap_or("");
                        if rec_hash != skill_hash {
                            updates.push(json!({
                                "id": skill_id,
                                "skillName": skill.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                                "dirName": skill.get("dirName").and_then(|v| v.as_str()).unwrap_or(""),
                                "repoId": repo_id,
                                "repoName": format!("{}/{}", repo.get("owner").and_then(|v| v.as_str()).unwrap_or(""), repo.get("name").and_then(|v| v.as_str()).unwrap_or("")),
                            }));
                        }
                    }
                }
            }
        }
    }

    Ok(json!({ "updates": updates }))
}

/// 应用单个技能更新：重新从仓库复制到 SSOT，并刷新 hash
#[tauri::command]
pub async fn skills_apply_update(dir_name: String, repo_id: String) -> Result<(), String> {
    let repos = store::get_key("repos");
    let repo = repos
        .as_array()
        .and_then(|arr| arr.iter().find(|r| r.get("id").and_then(|v| v.as_str()) == Some(&repo_id)))
        .ok_or("仓库不存在")?;

    let subdir = repo.get("subdir").and_then(|v| v.as_str()).unwrap_or("");

    let skills = skills::scan_skills(&repo_id, subdir);
    let skill = skills
        .iter()
        .find(|s| s.get("dirName").and_then(|v| v.as_str()) == Some(&dir_name))
        .ok_or("技能不存在于仓库")?;

    let dir_path = skill.get("dirPath").and_then(|v| v.as_str()).ok_or("缺少技能路径")?;
    let new_hash = skill.get("hash").and_then(|v| v.as_str()).unwrap_or("").to_string();

    skills::install_to_ssot(dir_path, &dir_name)?;

    // 更新 SSOT 后，重新同步所有已启用的工具目录
    // （symlink 模式 junction 自动生效，copy 模式需重新复制）
    let mut installed = store::get_key("installed");
    let mut resynced: Vec<String> = Vec::new();
    if let Some(arr) = installed.as_array_mut() {
        if let Some(rec) = arr.iter_mut().find(|i| i.get("dirName").and_then(|v| v.as_str()) == Some(&dir_name)) {
            rec["hash"] = json!(new_hash);
            if let Some(apps) = rec.get("apps").and_then(|v| v.as_object()) {
                for (tool_id, enabled) in apps.iter() {
                    if enabled.as_bool() == Some(true) {
                        if let Err(e) = skills::toggle_app(&dir_name, tool_id, true) {
                            logger::log("warn", "skill-update-resync", json!({ "toolId": tool_id, "error": e }));
                        } else {
                            resynced.push(tool_id.clone());
                        }
                    }
                }
            }
        }
    }
    store::set_key("installed", installed);

    logger::log("info", "skill-update", json!({ "dirName": dir_name, "repoId": repo_id, "resynced": resynced }));
    Ok(())
}

// ═══ 已安装 ═══

#[tauri::command]
pub async fn installed_list() -> Result<Value, String> {
    Ok(store::get_key("installed"))
}

// ═══ 工具列表 ═══

#[tauri::command]
pub async fn tools_list() -> Result<Value, String> {
    let tools: Vec<Value> = paths::TOOLS.iter().map(|t| {
        json!({ "id": t.id, "name": t.name, "installed": paths::is_tool_installed(t.id) })
    }).collect();
    Ok(json!(tools))
}

/// 检测工具是否安装
#[tauri::command]
pub async fn tools_check_installed(tool_id: String) -> Result<bool, String> {
    Ok(paths::is_tool_installed(&tool_id))
}

// ═══ 设置 ═══

#[tauri::command]
pub async fn settings_get() -> Result<Value, String> {
    Ok(store::get_settings())
}

#[tauri::command]
pub async fn settings_set(partial: Value) -> Result<Value, String> {
    store::set_settings(partial);
    Ok(store::get_settings())
}

// ═══ 日志 ═══

#[tauri::command]
pub async fn logs_read(limit: Option<usize>, level: Option<String>) -> Result<Value, String> {
    let logs = logger::read_logs(limit.unwrap_or(200), level.as_deref());
    Ok(json!(logs))
}

#[tauri::command]
pub async fn logs_clear() -> Result<(), String> {
    logger::clear_logs();
    Ok(())
}

// ═══ 目录操作 ═══

#[tauri::command]
pub async fn dir_open(dir_path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&dir_path)
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub async fn dir_list() -> Result<Value, String> {
    let settings = store::get_settings();
    let custom_dirs = settings.get("toolDirs").cloned().unwrap_or(json!({}));

    let tools_map: serde_json::Map<String, Value> = paths::TOOLS.iter().map(|t| {
        let dir = paths::get_tool_skills_dir(t.id, &custom_dirs);
        (t.id.to_string(), json!(dir.to_string_lossy()))
    }).collect();

    Ok(json!({
        "app": {
            "appDir": paths::app_dir().to_string_lossy(),
            "reposDir": paths::repos_dir().to_string_lossy(),
            "backupsDir": paths::backups_dir().to_string_lossy(),
            "logsDir": paths::logs_dir().to_string_lossy(),
        },
        "tools": Value::Object(tools_map),
    }))
}

#[tauri::command]
pub async fn dir_size(dir_path: String) -> Result<Value, String> {
    fn walk_size(dir: &Path) -> u64 {
        let mut size = 0;
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    size += walk_size(&path);
                } else if let Ok(meta) = fs::metadata(&path) {
                    size += meta.len();
                }
            }
        }
        size
    }

    let size = walk_size(Path::new(&dir_path));
    Ok(json!({ "size": size }))
}

// ═══ skills.sh 搜索 ═══

#[tauri::command]
pub async fn skills_sh_search(query: String) -> Result<Value, String> {
    if query.trim().is_empty() {
        return Ok(json!({ "skills": [] }));
    }

    let url = format!("https://skills.sh/api/search?q={}&limit=50", urlencode(&query));

    let response = reqwest_blocking_get(&url).map_err(|e| e.to_string())?;

    Ok(response)
}

#[tauri::command]
pub async fn skills_sh_trending() -> Result<Value, String> {
    let url = "https://skills.sh/api/trending?limit=50";
    let response = reqwest_blocking_get(url).map_err(|e| e.to_string())?;
    Ok(response)
}

/// URL 编码
fn urlencode(s: &str) -> String {
    s.chars().map(|c| match c {
        ' ' => "+".to_string(),
        c if c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.' || c == '~' => c.to_string(),
        c => format!("%{:02X}", c as u8),
    }).collect()
}

/// 用 std HTTP 客户端请求（不依赖 reqwest crate）
fn reqwest_blocking_get(url: &str) -> Result<Value, Box<dyn std::error::Error>> {
    // 使用 PowerShell 的 Invoke-RestMethod 作为 HTTP 客户端
    let output = crate::paths::silent_cmd("powershell")
        .args([
            "-NoProfile", "-Command",
            &format!(
                "$env:HTTP_PROXY='http://127.0.0.1:7897'; $env:HTTPS_PROXY='http://127.0.0.1:7897'; [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; Invoke-RestMethod -Uri '{}' -UseBasicParsing | ConvertTo-Json -Depth 10",
                url.replace('\'', "''")
            ),
        ])
        .output()?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("HTTP request failed: {}", err).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let json: Value = serde_json::from_str(stdout.trim())?;
    Ok(json)
}

// ═══ 导入/导出 ═══

#[tauri::command]
pub async fn config_export(file_path: String) -> Result<(), String> {
    let data = json!({
        "version": 1,
        "repos": store::get_key("repos"),
        "installed": store::get_key("installed"),
        "settings": store::get_settings(),
    });
    fs::write(&file_path, serde_json::to_string_pretty(&data).unwrap_or_default())
        .map_err(|e| e.to_string())?;
    logger::log("info", "config-export", json!({ "filePath": file_path }));
    Ok(())
}

#[tauri::command]
pub async fn config_import(file_path: String) -> Result<Value, String> {
    let content = fs::read_to_string(&file_path).map_err(|e| e.to_string())?;
    let data: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    if let Some(repos) = data.get("repos") {
        store::set_key("repos", repos.clone());
    }
    if let Some(settings) = data.get("settings") {
        store::set_key("settings", settings.clone());
    }
    if let Some(installed) = data.get("installed") {
        store::set_key("installed", installed.clone());
    }
    logger::log("info", "config-import", json!({ "filePath": file_path }));
    Ok(data)
}
