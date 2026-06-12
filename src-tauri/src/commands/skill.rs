use crate::models::{Skill, SkillIndex};
use crate::commands::config::load_config_from_disk;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

/// 根据技能名称去重，保留首次出现的技能
fn dedup_skills_by_name(skills: Vec<Skill>) -> Vec<Skill> {
    let mut seen = HashSet::new();
    skills.into_iter().filter(|s| seen.insert(s.name.clone())).collect()
}

/// 从仓库目录解析技能列表
/// 优先读取 skills.json 索引文件，否则通过 SKILL.md frontmatter 识别
fn parse_skills_from_repo(repo_dir: &Path, repo_url: &str) -> Vec<Skill> {
    // 尝试读取 skills.json 索引文件
    let index_file = repo_dir.join("skills.json");
    if index_file.exists() {
        if let Ok(content) = fs::read_to_string(&index_file) {
            if let Ok(index) = serde_json::from_str::<SkillIndex>(&content) {
                let skills: Vec<Skill> = index.skills.into_iter().map(|entry| Skill {
                    id: entry.id,
                    name: entry.name,
                    description: entry.description,
                    repo_url: repo_url.to_string(),
                    source_path: repo_dir.join(&entry.path),
                    version: entry.version,
                    author: entry.author,
                    tags: entry.tags,
                }).collect();
                return dedup_skills_by_name(skills);
            }
        }
    }

    // 回退：扫描仓库中所有 SKILL.md 文件，通过 frontmatter 识别技能
    let skills = scan_skills_from_skill_md(repo_dir, repo_url);
    dedup_skills_by_name(skills)
}

/// 解析文件开头的 frontmatter
/// 格式: `---` 包裹的 YAML 键值对，要求包含必填的 name 和 description
fn parse_frontmatter(content: &str) -> Option<HashMap<String, String>> {
    let lines: Vec<&str> = content.lines().collect();

    if lines.is_empty() || lines[0].trim() != "---" {
        return None;
    }

    // 从第二行开始查找闭合的 ---
    let end = lines[1..].iter().position(|l| l.trim() == "---")?;

    let mut fields = HashMap::new();
    let frontmatter = &lines[1..end + 1];
    let mut i = 0;
    while i < frontmatter.len() {
        let line = frontmatter[i];
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim().to_string();
            if !key.is_empty() {
                let value = value.trim();
                if matches!(value, "|" | "|-" | "|+" | ">" | ">-" | ">+") {
                    let folded = value.starts_with('>');
                    let mut block_lines = Vec::new();
                    i += 1;

                    while i < frontmatter.len() {
                        let block_line = frontmatter[i];
                        if !block_line.trim().is_empty()
                            && !block_line.starts_with(' ')
                            && !block_line.starts_with('\t')
                        {
                            break;
                        }

                        block_lines.push(block_line.trim_start());
                        i += 1;
                    }

                    let value = if folded {
                        block_lines.join(" ")
                    } else {
                        block_lines.join("\n")
                    };
                    fields.insert(key, value.trim().to_string());
                    continue;
                }

                fields.insert(key, value.to_string());
            }
        }
        i += 1;
    }

    // 必须包含 name 和 description
    if fields.contains_key("name") && fields.contains_key("description") {
        Some(fields)
    } else {
        None
    }
}

/// 递归查找目录下所有 SKILL.md 文件
fn find_skill_md_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut results = vec![];

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return results,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        // 跳过 .git 目录
        if name_str == ".git" {
            continue;
        }

        if path.is_dir() {
            results.extend(find_skill_md_files(&path));
        } else if name_str == "SKILL.md" {
            results.push(path);
        }
    }

    results
}

/// 扫描仓库中所有 SKILL.md 文件，通过 frontmatter 识别技能
fn scan_skills_from_skill_md(repo_dir: &Path, repo_url: &str) -> Vec<Skill> {
    let mut skills = vec![];

    let skill_files = find_skill_md_files(repo_dir);

    for skill_md_path in skill_files {
        if let Ok(content) = fs::read_to_string(&skill_md_path) {
            if let Some(fields) = parse_frontmatter(&content) {
                let skill_dir = skill_md_path.parent().unwrap_or(repo_dir);
                let dir_name = skill_dir.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| "unknown".to_string());

                skills.push(Skill {
                    id: dir_name.clone(),
                    name: fields.get("name").cloned().unwrap_or(dir_name.clone()),
                    description: fields.get("description").cloned().unwrap_or_default(),
                    repo_url: repo_url.to_string(),
                    source_path: skill_dir.to_path_buf(),
                    version: fields.get("version").cloned(),
                    author: fields.get("author").cloned(),
                    tags: None,
                });
            }
        }
    }

    skills
}

pub(crate) fn count_skills_from_repo(repo_dir: &Path, _repo_url: &str) -> u32 {
    let index_file = repo_dir.join("skills.json");
    if index_file.exists() {
        if let Ok(content) = fs::read_to_string(&index_file) {
            if let Ok(index) = serde_json::from_str::<SkillIndex>(&content) {
                return index.skills.len() as u32;
            }
        }
    }
    find_skill_md_files(repo_dir).len() as u32
}

/// 供 install.rs 内部调用的辅助函数
pub(crate) fn parse_skills_from_repo_url(repo_url: &str) -> Result<Vec<Skill>, String> {
    let config = load_config_from_disk()?;
    let repo = config.repos.iter()
        .find(|r| r.url == repo_url)
        .ok_or_else(|| format!("仓库不存在: {}", repo_url))?;
    if !repo.local_path.exists() {
        return Err(format!("仓库目录不存在: {}", repo.local_path.display()));
    }
    Ok(parse_skills_from_repo(&repo.local_path, repo_url))
}

#[tauri::command]
pub async fn list_skills(repo_url: String) -> Result<Vec<Skill>, String> {
    parse_skills_from_repo_url(&repo_url)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub skill: Skill,
    pub repo_name: String,
    pub repo_url: String,
}

#[tauri::command]
pub async fn search_skills(query: String) -> Result<Vec<SearchResult>, String> {
    let query = query.to_lowercase();
    let config = load_config_from_disk()?;

    let mut results = Vec::new();

    for repo in &config.repos {
        if !repo.local_path.exists() {
            continue;
        }

        let skills = parse_skills_from_repo(&repo.local_path, &repo.url);

        for skill in skills {
            let name_match = skill.name.to_lowercase().contains(&query);
            let desc_match = skill.description.to_lowercase().contains(&query);
            let tag_match = skill.tags.as_ref().map_or(false, |tags| {
                tags.iter().any(|t| t.to_lowercase().contains(&query))
            });

            if name_match || desc_match || tag_match {
                results.push(SearchResult {
                    skill,
                    repo_name: repo.name.clone(),
                    repo_url: repo.url.clone(),
                });
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_frontmatter_reads_block_description() {
        let content = r#"---
name: joeseesun-qiaomu-goal-meta-skill
description: |
  当 SKILL.md 使用多行描述时，也应该提取真实描述。
---

# Goal Meta Skill
"#;

        let fields = parse_frontmatter(content).expect("应该解析 frontmatter");

        assert_eq!(
            fields.get("description").map(String::as_str),
            Some("当 SKILL.md 使用多行描述时，也应该提取真实描述。")
        );
    }
}
