use std::path::Path;

pub enum JunctionStatus {
    NotExists,
    IsJunction,
    IsDirectory,
}

/// 检查路径的 Junction/Symlink 状态
pub fn check_status(path: &Path) -> JunctionStatus {
    if !path.exists() {
        if is_junction(path) {
            return JunctionStatus::IsJunction;
        }
        return JunctionStatus::NotExists;
    }
    if is_junction(path) {
        JunctionStatus::IsJunction
    } else {
        JunctionStatus::IsDirectory
    }
}

/// 检查路径是否是 Junction/Symlink 链接
fn is_junction(path: &Path) -> bool {
    if let Ok(metadata) = std::fs::symlink_metadata(path) {
        return metadata.file_type().is_symlink() && metadata.is_dir();
    }
    false
}

/// 删除 Junction/Symlink 链接（不删除目标）
pub fn remove_junction(path: &Path) -> Result<(), String> {
    std::fs::remove_dir(path)
        .map_err(|e| format!("删除链接失败: {}", e))
}

/// 创建 Junction/Symlink 链接
pub fn create_junction(link_path: &Path, target_path: &Path) -> Result<(), String> {
    if !target_path.exists() {
        return Err(format!("源目录不存在: {}", target_path.display()));
    }

    if let Some(parent) = link_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    #[cfg(windows)]
    {
        let output = std::process::Command::new("cmd")
            .args([
                "/C",
                "mklink",
                "/J",
                &link_path.to_string_lossy(),
                &target_path.to_string_lossy(),
            ])
            .output()
            .map_err(|e| format!("执行 mklink 失败: {}", e))?;

        if output.status.success() {
            Ok(())
        } else {
            Err(format!(
                "创建 Junction 失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(target_path, link_path)
            .map_err(|e| format!("创建符号链接失败: {}", e))
    }
}
