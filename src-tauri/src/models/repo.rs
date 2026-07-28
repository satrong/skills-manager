use serde::{Deserialize, Serialize};
use std::path::PathBuf;

fn default_source() -> String {
    "git".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub url: String,
    #[serde(alias = "local_path")]
    pub local_path: PathBuf,
    pub name: String,
    #[serde(alias = "last_update")]
    pub last_update: String,
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skill_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub repo_url: String,
    pub source_path: PathBuf,
    pub version: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteEntry {
    pub skill_id: String,
    pub repo_url: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxyConfig {
    pub enabled: bool,
    pub url: String,
}

impl ProxyConfig {
    /// 返回实际生效的代理地址；未启用或地址为空时返回 None
    pub fn effective(&self) -> Option<&str> {
        if !self.enabled {
            return None;
        }
        let url = self.url.trim();
        if url.is_empty() {
            None
        } else {
            Some(url)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppConfig {
    #[serde(default)]
    pub repos: Vec<Repo>,
    #[serde(default)]
    pub tool_paths: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub project_paths: Vec<String>,
    #[serde(default)]
    pub default_tool_type: Option<String>,
    #[serde(default)]
    pub favorites: Vec<FavoriteEntry>,
    #[serde(default)]
    pub proxy: ProxyConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            repos: vec![],
            tool_paths: std::collections::HashMap::new(),
            project_paths: vec![],
            default_tool_type: Some("claude-code".to_string()),
            favorites: vec![],
            proxy: ProxyConfig::default(),
        }
    }
}

// 技能仓库索引文件格式
#[derive(Debug, Deserialize)]
pub struct SkillIndex {
    pub skills: Vec<SkillIndexEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SkillIndexEntry {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub version: Option<String>,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
}
