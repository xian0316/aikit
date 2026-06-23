use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::paths;
use crate::logger;

/// Clone 仓库到本地缓存
pub fn clone_repo(
    owner: &str,
    name: &str,
    branch: &str,
    repo_id: &str,
) -> Result<PathBuf, String> {
    paths::ensure_dir(&paths::repos_dir());
    let local_path = paths::repos_dir().join(repo_id);

    // 如果已存在先删除
    if local_path.exists() {
        fs::remove_dir_all(&local_path).map_err(|e| e.to_string())?;
    }

    let url = format!("https://github.com/{}/{}.git", owner, name);
    let output = Command::new("git")
        .args([
            "clone",
            "--branch",
            branch,
            "--depth",
            "1",
            &url,
            local_path.to_str().unwrap_or(""),
        ])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(err.to_string());
    }

    logger::log(
        "info",
        "repo-clone",
        serde_json::json!({ "owner": owner, "name": name, "branch": branch }),
    );
    Ok(local_path)
}

/// Pull 更新
pub fn pull_repo(repo_id: &str) -> Result<String, String> {
    let local_path = paths::repos_dir().join(repo_id);
    if !local_path.exists() {
        return Err("仓库缓存不存在".to_string());
    }

    let output = Command::new("git")
        .args(["pull", "origin", "--force"])
        .current_dir(&local_path)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(err.to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    logger::log("info", "repo-pull", serde_json::json!({ "repoId": repo_id }));
    Ok(stdout)
}

/// 删除本地仓库缓存
pub fn delete_repo_cache(repo_id: &str) {
    let local_path = paths::repos_dir().join(repo_id);
    if local_path.exists() {
        let _ = fs::remove_dir_all(&local_path);
    }
}

/// 获取仓库本地路径
pub fn get_repo_path(repo_id: &str) -> PathBuf {
    paths::repos_dir().join(repo_id)
}
