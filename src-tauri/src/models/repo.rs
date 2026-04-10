use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub url: String,
    pub local_path: PathBuf,
    pub name: String,
    pub last_update: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
pub struct AppConfig {
    #[serde(default)]
    pub repos: Vec<Repo>,
    #[serde(default)]
    pub tool_paths: std::collections::HashMap<String, String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            repos: vec![],
            tool_paths: std::collections::HashMap::new(),
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
