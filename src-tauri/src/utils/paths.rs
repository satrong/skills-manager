use std::path::PathBuf;

/// 获取 skills-manager 配置目录: %USERPROFILE%\.skills-manager\
pub fn config_dir() -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|h| h.join(".skills-manager"))
        .ok_or_else(|| "无法获取用户主目录".to_string())
}

/// 获取 config.json 路径
pub fn config_file() -> Result<PathBuf, String> {
    config_dir().map(|d| d.join("config.json"))
}

/// 获取仓库存储目录: %USERPROFILE%\.skills-manager\repos\
pub fn repos_dir() -> Result<PathBuf, String> {
    config_dir().map(|d| d.join("repos"))
}

/// 从 GitHub URL 生成本地仓库目录名
/// 例如: "https://github.com/anthropics/skills" -> "anthropics-skills"
pub fn repo_dir_name(url: &str) -> Result<String, String> {
    let url = url.trim_end_matches('/').trim_end_matches(".git");
    let parts: Vec<&str> = url.rsplitn(3, '/').collect();
    if parts.len() < 2 {
        return Err(format!("无效的 GitHub URL: {}", url));
    }
    let repo_name = parts[0];
    let user_name = parts[1];
    Ok(format!("{}-{}", user_name, repo_name))
}

/// 展开 %USERPROFILE% 到实际路径
pub fn expand_path(path: &str) -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户主目录".to_string())?;
    Ok(path.replace("%USERPROFILE%", &home.to_string_lossy()))
}

/// 获取工具默认技能目录
pub fn default_tool_path(tool: &str) -> Option<String> {
    let path = match tool {
        "claude-code" => "%USERPROFILE%\\.claude\\skills",
        "cursor" => "%USERPROFILE%\\.cursor\\skills",
        "codex" => "%USERPROFILE%\\.codex\\skills",
        "opencode" => "%USERPROFILE%\\.config\\opencode\\skills",
        "qoder" => "%USERPROFILE%\\.qoder\\skills",
        "kilo" => "%USERPROFILE%\\.kilocode\\skills",
        _ => return None,
    };
    expand_path(path).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_dir_name() {
        assert_eq!(
            repo_dir_name("https://github.com/anthropics/skills").unwrap(),
            "anthropics-skills"
        );
        assert_eq!(
            repo_dir_name("https://github.com/anthropics/skills.git").unwrap(),
            "anthropics-skills"
        );
        assert_eq!(
            repo_dir_name("https://github.com/anthropics/skills/").unwrap(),
            "anthropics-skills"
        );
    }

    #[test]
    fn test_repo_dir_name_invalid() {
        assert!(repo_dir_name("not-a-url").is_err());
    }
}
