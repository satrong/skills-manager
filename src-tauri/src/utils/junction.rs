use std::path::Path;
use std::process::Command;

pub enum JunctionStatus {
    NotExists,
    IsJunction,
    IsDirectory,
}

/// 检查路径的 Junction 状态
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

/// 检查路径是否是 Junction 链接
fn is_junction(path: &Path) -> bool {
    use std::os::windows::fs::MetadataExt;
    if let Ok(metadata) = std::fs::symlink_metadata(path) {
        let attrs = metadata.file_attributes();
        return (attrs & 0x400) != 0 && metadata.is_dir();
    }
    false
}

/// 删除 Junction 链接（不删除目标）
pub fn remove_junction(path: &Path) -> Result<(), String> {
    let output = Command::new("cmd")
        .args(["/C", "rmdir", &path.to_string_lossy()])
        .output()
        .map_err(|e| format!("执行 rmdir 失败: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "删除 junction 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// 创建 Junction 链接
pub fn create_junction(link_path: &Path, target_path: &Path) -> Result<(), String> {
    if !target_path.exists() {
        return Err(format!("源目录不存在: {}", target_path.display()));
    }

    if let Some(parent) = link_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("创建父目录失败: {}", e))?;
    }

    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "New-Item -ItemType Junction -Path '{}' -Target '{}'",
                link_path.to_string_lossy(),
                target_path.to_string_lossy()
            ),
        ])
        .output()
        .map_err(|e| format!("执行 PowerShell 失败: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "创建 Junction 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
