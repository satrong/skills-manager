use crate::commands::config::{load_config_from_disk, save_config_to_disk};
use crate::commands::skill::parse_skills_from_repo_url;
use crate::utils::{junction, paths};
use std::path::PathBuf;

/// 检查 junction 是否已存在
#[tauri::command]
pub async fn check_junction_exists(link_path: String) -> bool {
    let path = PathBuf::from(&link_path);
    matches!(
        junction::check_status(&path),
        junction::JunctionStatus::IsJunction | junction::JunctionStatus::IsDirectory
    )
}

/// 安装技能（创建 Junction 链接）
#[tauri::command]
pub async fn install_skill(
    skill_id: String,
    repo_url: String,
    install_type: String,
    tool_type: String,
    target_path: String,
    overwrite: bool,
    remember_path: bool,
) -> Result<(), String> {
    let skills = parse_skills_from_repo_url(&repo_url)?;
    let skill = skills.iter()
        .find(|s| s.id == skill_id)
        .ok_or_else(|| format!("技能不存在: {}", skill_id))?;

    let install_dir = if install_type == "project" {
        PathBuf::from(&target_path).join(".skills")
    } else {
        if target_path.is_empty() {
            let expanded = paths::default_tool_path(&tool_type)
                .ok_or_else(|| format!("未知工具: {}", tool_type))?;
            PathBuf::from(expanded)
        } else {
            PathBuf::from(paths::expand_path(&target_path)?)
        }
    };

    let link_path = install_dir.join(&skill_id);

    match junction::check_status(&link_path) {
        junction::JunctionStatus::IsJunction | junction::JunctionStatus::IsDirectory => {
            if !overwrite {
                return Err(format!(
                    "JUNCTION_EXISTS:{}",
                    link_path.to_string_lossy()
                ));
            }
            junction::remove_junction(&link_path)?;
        }
        junction::JunctionStatus::NotExists => {}
    }

    junction::create_junction(&link_path, &skill.source_path)?;

    if remember_path && install_type == "global" && !target_path.is_empty() {
        let mut config = load_config_from_disk()?;
        config.tool_paths.insert(tool_type, target_path);
        save_config_to_disk(&config)?;
    }

    Ok(())
}

/// 卸载技能（删除 Junction 链接）
#[tauri::command]
pub async fn uninstall_skill(link_path: String) -> Result<(), String> {
    let path = PathBuf::from(&link_path);
    match junction::check_status(&path) {
        junction::JunctionStatus::IsJunction => junction::remove_junction(&path),
        junction::JunctionStatus::NotExists => Err("链接不存在".to_string()),
        junction::JunctionStatus::IsDirectory => Err("该路径是普通目录，不是链接".to_string()),
    }
}
