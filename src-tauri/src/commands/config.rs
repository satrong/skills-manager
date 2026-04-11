use crate::models::AppConfig;
use crate::utils::paths;
use std::fs;

pub(crate) fn load_config_from_disk() -> Result<AppConfig, String> {
    let config_file = paths::config_file()?;
    if !config_file.exists() {
        return Ok(AppConfig::default());
    }
    let content = fs::read_to_string(&config_file)
        .map_err(|e| format!("读取配置文件失败: {}", e))?;
    serde_json::from_str(&content)
        .map_err(|e| format!("解析配置文件失败: {}", e))
}

pub(crate) fn save_config_to_disk(config: &AppConfig) -> Result<(), String> {
    let config_dir = paths::config_dir()?;
    fs::create_dir_all(&config_dir)
        .map_err(|e| format!("创建配置目录失败: {}", e))?;
    let config_file = paths::config_file()?;
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("序列化配置失败: {}", e))?;
    fs::write(&config_file, content)
        .map_err(|e| format!("写入配置文件失败: {}", e))
}

#[tauri::command]
pub async fn load_config() -> Result<AppConfig, String> {
    load_config_from_disk()
}

#[tauri::command]
pub async fn save_config(config: AppConfig) -> Result<(), String> {
    save_config_to_disk(&config)
}

#[tauri::command]
pub async fn get_tool_path(tool_type: String) -> Result<String, String> {
    let config = load_config_from_disk()?;
    if let Some(path) = config.tool_paths.get(&tool_type) {
        return Ok(path.clone());
    }
    paths::default_tool_path(&tool_type)
        .ok_or_else(|| format!("未知工具类型: {}", tool_type))
}

#[tauri::command]
pub async fn set_tool_path(tool_type: String, path: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;
    config.tool_paths.insert(tool_type, path);
    save_config_to_disk(&config)
}

#[tauri::command]
pub async fn get_project_paths() -> Result<Vec<String>, String> {
    let config = load_config_from_disk()?;
    Ok(config.project_paths)
}

#[tauri::command]
pub async fn add_project_path(path: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;
    config.project_paths.retain(|p| p != &path);
    config.project_paths.insert(0, path);
    if config.project_paths.len() > 20 {
        config.project_paths.truncate(20);
    }
    save_config_to_disk(&config)
}
