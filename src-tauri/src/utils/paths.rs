use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct ToolsConfig {
    tools: std::collections::HashMap<String, ToolDef>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ToolDef {
    #[allow(dead_code)]
    label: String,
    global_path: Option<String>,
    project_dir: Option<String>,
}

static TOOLS: OnceLock<ToolsConfig> = OnceLock::new();

fn tools_config() -> &'static ToolsConfig {
    TOOLS.get_or_init(|| {
        serde_json::from_str(include_str!("../../../shared/tools.json"))
            .expect("解析 tools.json 失败")
    })
}

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

/// 展开 %USERPROFILE% 到实际路径，并规范化路径分隔符
pub fn expand_path(path: &str) -> Result<String, String> {
    let home = dirs::home_dir().ok_or_else(|| "无法获取用户主目录".to_string())?;
    let expanded = path.replace("%USERPROFILE%", &home.to_string_lossy());
    Ok(PathBuf::from(&expanded).to_string_lossy().to_string())
}

/// 获取工具默认技能目录（从 shared/tools.json 读取配置）
pub fn default_tool_path(tool: &str) -> Option<String> {
    let def = tools_config().tools.get(tool)?;
    let global_path = def.global_path.as_ref()?;
    let home = dirs::home_dir()?;
    Some(home.join(global_path).to_string_lossy().to_string())
}

/// 获取工具在项目中的配置目录（相对于项目根目录，从 shared/tools.json 读取配置）
pub fn project_tool_dir(tool: &str) -> Option<String> {
    let def = tools_config().tools.get(tool)?;
    let project_dir = def.project_dir.as_ref()?;
    Some(PathBuf::from(project_dir).to_string_lossy().to_string())
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
