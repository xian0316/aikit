use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::git;
use crate::logger;
use crate::paths;
use crate::store;

/// 安装技能到 SSOT 统一目录（不涉及具体工具）
/// 安装后再通过 toggle_app 分发到各工具
pub fn install_to_ssot(skill_dir: &str, skill_name: &str) -> Result<(), String> {
    let ssot = paths::ssot_dir();
    paths::ensure_dir(&ssot);
    let dest = ssot.join(skill_name);

    // 如果已存在先删除
    if dest.exists() {
        fs::remove_dir_all(&dest).map_err(|e| e.to_string())?;
    }

    copy_dir_recursive(Path::new(skill_dir), &dest)?;

    logger::log("info", "install-ssot", json!({ "skillName": skill_name }));
    Ok(())
}

/// 切换工具同步状态（启用/禁用）
/// enabled=true → symlink/copy 到工具目录
/// enabled=false → 从工具目录删除
pub fn toggle_app(skill_name: &str, tool_id: &str, enabled: bool) -> Result<(), String> {
    let settings = store::get_settings();
    // 固定使用软链接模式（copy 模式已弃用）
    let mode = "symlink";

    let custom_dirs = settings.get("toolDirs").cloned().unwrap_or(json!({}));
    let target_dir = paths::get_tool_skills_dir(tool_id, &custom_dirs);
    paths::ensure_dir(&target_dir);
    let dest = target_dir.join(skill_name);

    if enabled {
        // 启用：从 SSOT 同步到工具目录
        let ssot_src = paths::ssot_dir().join(skill_name);
        if !ssot_src.exists() {
            return Err("技能在 SSOT 中不存在".to_string());
        }

        if dest.exists() {
            let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));
        }

        if mode == "symlink" {
            #[cfg(target_os = "windows")]
            {
                let output = crate::paths::silent_cmd("cmd")
                    .args(["/c", "mklink", "/J", dest.to_str().unwrap_or(""), ssot_src.to_str().unwrap_or("")])
                    .output()
                    .map_err(|e| e.to_string())?;
                if !output.status.success() {
                    // 回退到 copy
                    copy_dir_recursive(&ssot_src, &dest)?;
                }
            }
            #[cfg(not(target_os = "windows"))]
            {
                std::os::unix::fs::symlink(&ssot_src, &dest).map_err(|e| e.to_string())?;
            }
        } else {
            copy_dir_recursive(&ssot_src, &dest)?;
        }

        logger::log("info", "toggle-app-on", json!({ "skillName": skill_name, "toolId": tool_id }));
    } else {
        // 禁用：从工具目录删除
        if dest.exists() {
            let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));
        }
        logger::log("info", "toggle-app-off", json!({ "skillName": skill_name, "toolId": tool_id }));
    }

    Ok(())
}

/// 从 SSOT 卸载技能（同时清理所有工具目录）
pub fn uninstall_from_ssot(skill_name: &str) -> Result<(), String> {
    let ssot_src = paths::ssot_dir().join(skill_name);

    if !ssot_src.exists() {
        return Err("技能不存在".to_string());
    }

    // 备份
    let _ = backup_skill(skill_name, &ssot_src);

    // 清理所有工具目录
    let settings = store::get_settings();
    let custom_dirs = settings.get("toolDirs").cloned().unwrap_or(json!({}));
    for tool in paths::TOOLS {
        let tool_dir = paths::get_tool_skills_dir(tool.id, &custom_dirs);
        let tool_dest = tool_dir.join(skill_name);
        if tool_dest.exists() {
            let _ = fs::remove_file(&tool_dest).or_else(|_| fs::remove_dir_all(&tool_dest));
        }
    }

    // 删除 SSOT 源
    fs::remove_dir_all(&ssot_src).map_err(|e| e.to_string())?;

    logger::log("info", "uninstall-ssot", json!({ "skillName": skill_name }));
    Ok(())
}

/// 解析 SKILL.md / README.md 的 frontmatter（简易 YAML）
fn parse_frontmatter(file_path: &Path) -> (Value, String) {
    let content = match fs::read_to_string(file_path) {
        Ok(c) => c,
        Err(_) => return (json!({}), String::new()),
    };

    // 检查是否有 frontmatter
    if !content.starts_with("---\n") {
        return (json!({}), content);
    }

    let rest = &content[4..];
    if let Some(end) = rest.find("\n---") {
        let yaml_block = &rest[..end];
        let body = rest[end + 4..].trim().to_string();
        let mut fm = serde_json::Map::new();

        for line in yaml_block.lines() {
            if let Some(pos) = line.find(':') {
                let key = line[..pos].trim().to_string();
                let mut val = line[pos + 1..].trim().to_string();
                // 去引号
                if (val.starts_with('"') && val.ends_with('"'))
                    || (val.starts_with('\'') && val.ends_with('\''))
                {
                    val = val[1..val.len() - 1].to_string();
                }
                fm.insert(key, json!(val));
            }
        }

        (Value::Object(fm), body)
    } else {
        (json!({}), content)
    }
}

/// 计算文件夹的 SHA-256 哈希
pub fn hash_skill_dir(dir: &Path) -> String {
    let mut hasher = Sha256::new();
    let mut files: Vec<PathBuf> = Vec::new();
    collect_files(dir, &mut files);
    files.sort();

    for f in &files {
        if let Ok(rel) = f.strip_prefix(dir) {
            hasher.update(rel.to_string_lossy().as_bytes());
        }
        if let Ok(data) = fs::read(f) {
            hasher.update(&data);
        }
    }

    format!("{:x}", hasher.finalize())
}

fn collect_files(dir: &Path, files: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, files);
            } else {
                files.push(path);
            }
        }
    }
}

/// 扫描仓库中的所有技能
pub fn scan_skills(repo_id: &str, subdir: &str) -> Vec<Value> {
    let repo_path = git::get_repo_path(repo_id);
    let scan_root = if subdir.is_empty() {
        repo_path
    } else {
        repo_path.join(subdir)
    };

    if !scan_root.exists() {
        return vec![];
    }

    let mut skills = Vec::new();
    walk_for_skills(&scan_root, &scan_root, repo_id, &mut skills, 0);

    logger::log(
        "debug",
        "scan-skills",
        json!({ "repoId": repo_id, "count": skills.len() }),
    );
    skills
}

fn walk_for_skills(
    dir: &Path,
    scan_root: &Path,
    repo_id: &str,
    skills: &mut Vec<Value>,
    depth: usize,
) {
    if depth > 5 {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name.starts_with('.') || name == "node_modules" {
            continue;
        }
        if !path.is_dir() {
            continue;
        }

        // 检查是否是技能文件夹
        let skill_file = if path.join("SKILL.md").exists() {
            Some("SKILL.md")
        } else if path.join("README.md").exists() {
            Some("README.md")
        } else {
            None
        };

        if let Some(sf) = skill_file {
            let file_path = path.join(sf);
            let (fm, body) = parse_frontmatter(&file_path);
            let hash = hash_skill_dir(&path);
            let rel_path = path
                .strip_prefix(scan_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();

            let skill_name = fm
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(&name)
                .to_string();
            let description = fm
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            if description.is_empty() {
                let preview: String = body.chars().take(100).collect();
                let _ = std::mem::replace(&mut String::new(), preview); // borrow trick
            }

            skills.push(json!({
                "id": format!("{}:{}", repo_id, rel_path),
                "name": skill_name,
                "description": fm.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                "version": fm.get("version").and_then(|v| v.as_str()).unwrap_or(""),
                "author": fm.get("author").and_then(|v| v.as_str()).unwrap_or(""),
                "repoId": repo_id,
                "dirPath": path.to_string_lossy(),
                "dirName": name,
                "relPath": rel_path,
                "skillFile": sf,
                "hash": hash,
            }));
        } else {
            walk_for_skills(&path, scan_root, repo_id, skills, depth + 1);
        }
    }
}

/// 安装技能（软链接或复制）
pub fn install_skill(
    skill_dir: &str,
    skill_name: &str,
    tool_id: &str,
    mode: &str,
) -> Result<(), String> {
    let settings = store::get_settings();
    let custom_dirs = settings.get("toolDirs").cloned().unwrap_or(json!({}));
    let target_dir = paths::get_tool_skills_dir(tool_id, &custom_dirs);
    paths::ensure_dir(&target_dir);
    let dest = target_dir.join(skill_name);

    // 同名检查
    let exists = dest.exists();

    if mode == "symlink" {
        if exists {
            let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));
        }
        // Windows junction（不需要管理员权限）
        #[cfg(target_os = "windows")]
        {
            let output = crate::paths::silent_cmd("cmd")
                .args([
                    "/c",
                    "mklink",
                    "/J",
                    dest.to_str().unwrap_or(""),
                    skill_dir,
                ])
                .output()
                .map_err(|e| e.to_string())?;
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            std::os::unix::fs::symlink(skill_dir, &dest).map_err(|e| e.to_string())?;
        }
    } else {
        if exists {
            fs::remove_dir_all(&dest).map_err(|e| e.to_string())?;
        }
        copy_dir_recursive(Path::new(skill_dir), &dest)?;
    }

    logger::log("info", "install", json!({ "skillName": skill_name, "toolId": tool_id, "mode": mode }));
    Ok(())
}

/// 卸载技能（带备份）
pub fn uninstall_skill(skill_name: &str, tool_id: &str) -> Result<(), String> {
    let settings = store::get_settings();
    let custom_dirs = settings.get("toolDirs").cloned().unwrap_or(json!({}));
    let target_dir = paths::get_tool_skills_dir(tool_id, &custom_dirs);
    let dest = target_dir.join(skill_name);

    if !dest.exists() {
        return Err("技能不存在".to_string());
    }

    // 备份
    let _ = backup_skill(skill_name, &dest);

    // 删除
    let _ = fs::remove_file(&dest).or_else(|_| fs::remove_dir_all(&dest));

    logger::log("info", "uninstall", json!({ "skillName": skill_name, "toolId": tool_id }));
    Ok(())
}

/// 备份技能
fn backup_skill(skill_name: &str, skill_path: &Path) -> Result<(), String> {
    let ts = format!("{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0));
    let backup_dir = paths::backups_dir().join(format!("{}_{}", skill_name, ts));
    paths::ensure_dir(&paths::backups_dir());
    copy_dir_recursive(skill_path, &backup_dir)?;
    logger::log("info", "backup", json!({ "skillName": skill_name }));
    Ok(())
}

/// 递归复制目录
fn copy_dir_recursive(src: &Path, dest: &Path) -> Result<(), String> {
    if !dest.exists() {
        fs::create_dir_all(dest).map_err(|e| e.to_string())?;
    }
    for entry in fs::read_dir(src).map_err(|e| e.to_string())?.flatten() {
        let from = entry.path();
        let to = dest.join(entry.file_name());
        if from.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            fs::copy(&from, &to).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
