use crate::models::Repo;
use crate::utils::{git, paths};
use crate::commands::config::{load_config_from_disk, save_config_to_disk};
use crate::commands::skill::count_skills_from_repo;
use std::fs;
use std::path::PathBuf;

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
        source: "git".to_string(),
        skill_count: None,
    };

    config.repos.push(repo.clone());
    save_config_to_disk(&config)?;

    let mut repo = repo;
    repo.skill_count = Some(count_skills_from_repo(&repo.local_path, &repo.url));
    Ok(repo)
}

#[tauri::command]
pub async fn add_local_dir(path: String) -> Result<Repo, String> {
    let dir_path = PathBuf::from(&path);
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err("指定的目录不存在".to_string());
    }

    let canonical = dir_path.canonicalize()
        .map_err(|e| format!("无法解析路径: {}", e))?;

    let path_str = canonical.to_string_lossy().to_string();

    let mut config = load_config_from_disk()?;

    if config.repos.iter().any(|r| r.local_path == canonical) {
        return Err("该本地目录已添加".to_string());
    }

    let dir_name = canonical.file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "local-dir".to_string());

    let url = format!("local://{}", path_str);

    if config.repos.iter().any(|r| r.url == url) {
        return Err("该本地目录已添加".to_string());
    }

    let repo = Repo {
        url: url.clone(),
        local_path: canonical.clone(),
        name: dir_name,
        last_update: now_timestamp(),
        source: "local".to_string(),
        skill_count: None,
    };

    config.repos.push(repo.clone());
    save_config_to_disk(&config)?;

    let mut repo = repo;
    repo.skill_count = Some(count_skills_from_repo(&repo.local_path, &repo.url));
    Ok(repo)
}

#[tauri::command]
pub async fn remove_repo(url: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;

    let repo = config.repos.iter()
        .find(|r| r.url == url)
        .ok_or_else(|| "仓库不存在".to_string())?
        .clone();

    if repo.source != "local" && repo.local_path.exists() {
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

    if repo.source == "local" {
        if !repo.local_path.exists() {
            return Err("本地目录不存在".to_string());
        }
        repo.last_update = now_timestamp();
        save_config_to_disk(&config)?;
        return Ok("本地目录已刷新".to_string());
    }

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
        if repo.source == "local" {
            if repo.local_path.exists() {
                repo.last_update = now_timestamp();
                results.push(format!("{}: 已刷新", repo.name));
            } else {
                results.push(format!("{}: 本地目录不存在", repo.name));
            }
            continue;
        }
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
    let repos = config.repos.into_iter().map(|mut repo| {
        let count = if repo.local_path.exists() {
            count_skills_from_repo(&repo.local_path, &repo.url)
        } else {
            0
        };
        repo.skill_count = Some(count);
        repo
    }).collect();
    Ok(repos)
}
