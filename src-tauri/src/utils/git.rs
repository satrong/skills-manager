use std::path::Path;
use std::process::Command;

/// 克隆仓库到指定目录
pub fn clone_repo(url: &str, target_dir: &Path) -> Result<(), String> {
    if target_dir.exists() {
        return Err(format!("目录已存在: {}", target_dir.display()));
    }
    let parent = target_dir.parent()
        .ok_or_else(|| "无效路径".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|e| format!("创建目录失败: {}", e))?;

    let output = Command::new("git")
        .args(["clone", url, &target_dir.to_string_lossy()])
        .output()
        .map_err(|e| format!("执行 git 失败 (是否已安装 git?): {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git clone 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// 在已有仓库目录执行 git pull
pub fn pull_repo(repo_dir: &Path) -> Result<String, String> {
    if !repo_dir.exists() {
        return Err(format!("仓库目录不存在: {}", repo_dir.display()));
    }
    let output = Command::new("git")
        .args(["pull"])
        .current_dir(repo_dir)
        .output()
        .map_err(|e| format!("执行 git 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!(
            "git pull 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
