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
        #[cfg(windows)]
        return metadata.file_type().is_symlink() && metadata.is_dir();
        #[cfg(not(windows))]
        return metadata.file_type().is_symlink();
    }
    false
}

/// 删除 Junction/Symlink 链接（不删除目标）
pub fn remove_junction(path: &Path) -> Result<(), String> {
    #[cfg(windows)]
    {
        std::fs::remove_dir(path).map_err(|e| format!("删除链接失败: {}", e))
    }
    #[cfg(not(windows))]
    {
        std::fs::remove_file(path).map_err(|e| format!("删除链接失败: {}", e))
    }
}

/// 创建 Junction/Symlink 链接
pub fn create_junction(link_path: &Path, target_path: &Path) -> Result<(), String> {
    if !target_path.exists() {
        return Err(format!("源目录不存在: {}", target_path.display()));
    }

    if let Some(parent) = link_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    #[cfg(windows)]
    {
        junction_crate::create(target_path, link_path)
            .map_err(|e| format!("创建 Junction 失败: {}", e))
    }

    #[cfg(not(windows))]
    {
        std::os::unix::fs::symlink(target_path, link_path)
            .map_err(|e| format!("创建符号链接失败: {}", e))
    }
}
