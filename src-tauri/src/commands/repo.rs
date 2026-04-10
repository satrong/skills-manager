use crate::models::Repo;
use crate::utils::{git, paths};
use crate::commands::config::{load_config_from_disk, save_config_to_disk};
use std::fs;

fn now_timestamp() -> String {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

#[tauri::command]
pub async fn add_repo(url: String) -> Result<Repo, String> {
    if !url.starts_with("https://github.com/") {
        return Err("目前仅支持 GitHub 仓库 URL (https://github.com/...)".to_string());
    }

    let mut config = load_config_from_disk()?;

    if config.repos.iter().any(|r| r.url == url) {
        return Err("该仓库已添加".to_string());
    }

    let dir_name = paths::repo_dir_name(&url)?;
    let repos_dir = paths::repos_dir()?;
    let local_path = repos_dir.join(&dir_name);

    git::clone_repo(&url, &local_path)?;

    let repo = Repo {
        url: url.clone(),
        local_path: local_path.clone(),
        name: dir_name,
        last_update: now_timestamp(),
    };

    config.repos.push(repo.clone());
    save_config_to_disk(&config)?;

    Ok(repo)
}

#[tauri::command]
pub async fn remove_repo(url: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;

    let repo = config.repos.iter()
        .find(|r| r.url == url)
        .ok_or_else(|| "仓库不存在".to_string())?
        .clone();

    if repo.local_path.exists() {
        fs::remove_dir_all(&repo.local_path)
            .map_err(|e| format!("删除仓库目录失败: {}", e))?;
    }

    config.repos.retain(|r| r.url != url);
    save_config_to_disk(&config)
}

#[tauri::command]
pub async fn update_repo(url: String) -> Result<String, String> {
    let mut config = load_config_from_disk()?;

    let repo = config.repos.iter_mut()
        .find(|r| r.url == url)
        .ok_or_else(|| "仓库不存在".to_string())?;

    let result = git::pull_repo(&repo.local_path)?;
    repo.last_update = now_timestamp();

    save_config_to_disk(&config)?;

    Ok(result)
}

#[tauri::command]
pub async fn update_all_repos() -> Result<Vec<String>, String> {
    let mut config = load_config_from_disk()?;
    let mut results = vec![];

    for repo in config.repos.iter_mut() {
        match git::pull_repo(&repo.local_path) {
            Ok(msg) => {
                repo.last_update = now_timestamp();
                results.push(format!("{}: {}", repo.name, msg.trim()));
            }
            Err(e) => {
                results.push(format!("{}: 更新失败 - {}", repo.name, e));
            }
        }
    }

    save_config_to_disk(&config)?;

    Ok(results)
}

#[tauri::command]
pub async fn list_repos() -> Result<Vec<Repo>, String> {
    let config = load_config_from_disk()?;
    Ok(config.repos)
}
