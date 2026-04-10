# Skills Manager 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 构建一个 Tauri v2 桌面应用，用于管理 AI 编程工具的技能，支持从 GitHub 仓库克隆技能列表，并通过 Windows Junction 链接将技能安装到目标工具目录。

**Architecture:** 前端使用 Vue 3 + TypeScript，通过 Tauri invoke 调用 Rust 后端命令；Rust 后端负责 Git 操作、Junction 链接创建和配置文件读写；配置存储在 `%USERPROFILE%\.skills-manager\config.json`，仓库克隆到 `%USERPROFILE%\.skills-manager\repos\`。

**Tech Stack:** Vue 3 (Composition API, script setup), TypeScript, Rust, Tauri v2, serde/serde_json, tokio, dirs crate, Windows PowerShell (Junction 创建)

---

## 文件结构映射

### 新建文件

**Frontend:**
- `src/types/index.ts` — TypeScript 类型定义 (Repo, Skill, InstallRequest, AppConfig, ToolType, InstallType)
- `src/utils/toolPaths.ts` — 各工具默认技能目录路径配置
- `src/composables/useRepos.ts` — 仓库数据状态管理 + Tauri invoke 调用
- `src/composables/useSkills.ts` — 技能列表数据管理
- `src/composables/useInstall.ts` — 技能安装逻辑 + 状态管理
- `src/components/RepoManager.vue` — 添加/删除仓库弹窗组件
- `src/components/RepoGroup.vue` — 单个仓库分组展示组件
- `src/components/SkillCard.vue` — 单个技能卡片组件
- `src/components/SkillDialog.vue` — 技能安装配置弹窗组件

**Backend (Rust):**
- `src-tauri/src/models/mod.rs` — 模型模块导出
- `src-tauri/src/models/repo.rs` — Repo、Skill、AppConfig、InstallType、ToolType 结构体
- `src-tauri/src/commands/mod.rs` — 命令模块导出
- `src-tauri/src/commands/repo.rs` — 仓库 CRUD 命令 (add_repo, remove_repo, update_repo, update_all_repos, list_repos)
- `src-tauri/src/commands/skill.rs` — 技能列表读取命令 (list_skills)
- `src-tauri/src/commands/install.rs` — 技能安装命令 (install_skill, check_junction_exists)
- `src-tauri/src/commands/config.rs` — 配置读写命令 (load_config, save_config, get_tool_path, set_tool_path)
- `src-tauri/src/utils/mod.rs` — 工具模块导出
- `src-tauri/src/utils/git.rs` — git clone/pull 封装 (调用系统 git 命令)
- `src-tauri/src/utils/junction.rs` — Windows Junction 链接创建/删除
- `src-tauri/src/utils/paths.rs` — 路径解析工具 (home dir, config dir, repos dir)

### 修改文件

- `src-tauri/src/lib.rs` — 注册所有 Tauri 命令，移除 greet 示例
- `src-tauri/Cargo.toml` — 添加依赖: tokio, dirs
- `src/App.vue` — 完全重写为主界面布局

---

## Phase 1: 基础数据层

### Task 1: TypeScript 类型定义

**Files:**
- Create: `src/types/index.ts`

- [ ] **Step 1: 创建类型文件**

```typescript
// src/types/index.ts
export type ToolType =
  | 'claude-code'
  | 'cursor'
  | 'codex'
  | 'opencode'
  | 'qoder'
  | 'kilo'
  | 'custom';

export type InstallType = 'global' | 'project';

export interface Repo {
  url: string;
  localPath: string;
  name: string;
  lastUpdate: string;
  skills: Skill[];
}

export interface Skill {
  id: string;
  name: string;
  description: string;
  repoUrl: string;
  sourcePath: string;
  version?: string;
  author?: string;
  tags?: string[];
}

export interface InstallRequest {
  skillId: string;
  repoUrl: string;
  installType: InstallType;
  toolType: ToolType;
  targetPath: string;
  rememberPath?: boolean;
}

export interface AppConfig {
  repos: Omit<Repo, 'skills'>[];
  toolPaths: Partial<Record<ToolType, string>>;
}
```

- [ ] **Step 2: 类型检查**

```bash
cd d:/code/dev/app/skills-manager
pnpm vue-tsc --noEmit
```

期望输出: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/types/index.ts
git commit -m "feat: add TypeScript type definitions"
```

---

### Task 2: 工具路径配置

**Files:**
- Create: `src/utils/toolPaths.ts`

- [ ] **Step 1: 创建工具路径配置文件**

```typescript
// src/utils/toolPaths.ts
import type { ToolType } from '../types';

export const TOOL_LABELS: Record<ToolType, string> = {
  'claude-code': 'Claude Code',
  'cursor': 'Cursor',
  'codex': 'Codex',
  'opencode': 'Opencode',
  'qoder': 'Qoder',
  'kilo': 'Kilo Code',
  'custom': '自定义',
};

// 默认路径模板 (使用 %USERPROFILE% 占位符，由 Rust 后端解析)
export const DEFAULT_TOOL_PATHS: Record<Exclude<ToolType, 'custom'>, string> = {
  'claude-code': '%USERPROFILE%\\.claude\\skills',
  'cursor': '%USERPROFILE%\\.cursor\\skills',
  'codex': '%USERPROFILE%\\.codex\\skills',
  'opencode': '%USERPROFILE%\\.config\\opencode\\skills',
  'qoder': '%USERPROFILE%\\.qoder\\skills',
  'kilo': '%USERPROFILE%\\.kilocode\\skills',
};
```

- [ ] **Step 2: 类型检查**

```bash
pnpm vue-tsc --noEmit
```

期望输出: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/utils/toolPaths.ts
git commit -m "feat: add tool paths configuration"
```

---

### Task 3: Rust 依赖和数据模型

**Files:**
- Modify: `src-tauri/Cargo.toml`
- Create: `src-tauri/src/models/mod.rs`
- Create: `src-tauri/src/models/repo.rs`

- [ ] **Step 1: 更新 Cargo.toml 添加依赖**

修改 `src-tauri/Cargo.toml` 的 `[dependencies]` 部分:

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
dirs = "5"
```

- [ ] **Step 2: 创建模型目录和 mod.rs**

```rust
// src-tauri/src/models/mod.rs
pub mod repo;
pub use repo::*;
```

- [ ] **Step 3: 创建数据结构体**

```rust
// src-tauri/src/models/repo.rs
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
```

- [ ] **Step 4: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望输出: 无错误，tokio 和 dirs 依赖被下载

- [ ] **Step 5: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/models/
git commit -m "feat: add Rust data models and dependencies"
```

---

### Task 4: 路径工具函数

**Files:**
- Create: `src-tauri/src/utils/mod.rs`
- Create: `src-tauri/src/utils/paths.rs`

- [ ] **Step 1: 创建 utils 模块**

```rust
// src-tauri/src/utils/mod.rs
pub mod paths;
pub mod git;
pub mod junction;
```

- [ ] **Step 2: 创建路径工具**

```rust
// src-tauri/src/utils/paths.rs
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
```

- [ ] **Step 3: 运行单元测试**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo test utils::paths
```

期望输出:
```
test utils::paths::tests::test_repo_dir_name ... ok
test utils::paths::tests::test_repo_dir_name_invalid ... ok
```

- [ ] **Step 4: 更新 lib.rs 声明新模块**（让后续 cargo check 能找到模块）

修改 `src-tauri/src/lib.rs`：

```rust
// src-tauri/src/lib.rs
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 5: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望输出: 无错误（models 和 utils 模块被识别）

- [ ] **Step 6: Commit**

```bash
git add src-tauri/src/utils/ src-tauri/src/lib.rs
git commit -m "feat: add path utility functions with tests"
```

---

## Phase 2: 配置管理

### Task 5: 配置读写命令

**Files:**
- Create: `src-tauri/src/commands/mod.rs`
- Create: `src-tauri/src/commands/config.rs`
- Create: `src-tauri/src/utils/git.rs` (占位)
- Create: `src-tauri/src/utils/junction.rs` (占位)

- [ ] **Step 1: 创建 git.rs 和 junction.rs 占位文件**

```rust
// src-tauri/src/utils/git.rs
// 占位，Task 7 实现
```

```rust
// src-tauri/src/utils/junction.rs
// 占位，Task 12 实现
```

- [ ] **Step 2: 创建命令模块**

```rust
// src-tauri/src/commands/mod.rs
pub mod config;
pub mod repo;
pub mod skill;
pub mod install;
```

- [ ] **Step 3: 实现配置命令**

```rust
// src-tauri/src/commands/config.rs
use crate::models::AppConfig;
use crate::utils::paths;
use std::fs;

// pub(crate) 供其他命令模块内部调用（非 Tauri 命令）
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
```

- [ ] **Step 4: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望输出: 无错误

- [ ] **Step 5: 更新 lib.rs 声明 commands 模块**

修改 `src-tauri/src/lib.rs` 添加命令模块声明（保持 invoke_handler 为空，后续 Task 11 填充）：

```rust
// src-tauri/src/lib.rs
mod commands;
mod models;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 6: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望输出: 无错误

- [ ] **Step 7: Commit**

```bash
git add src-tauri/src/commands/ src-tauri/src/utils/git.rs src-tauri/src/utils/junction.rs src-tauri/src/lib.rs
git commit -m "feat: add config management commands"
```

---

## Phase 3: 仓库管理

### Task 6: Git 工具封装

**Files:**
- Modify: `src-tauri/src/utils/git.rs`

- [ ] **Step 1: 实现 Git 工具函数**

```rust
// src-tauri/src/utils/git.rs
use std::path::Path;
use std::process::Command;

/// 克隆仓库到指定目录
pub fn clone_repo(url: &str, target_dir: &Path) -> Result<(), String> {
    if target_dir.exists() {
        return Err(format!("目录已存在: {}", target_dir.display()));
    }
    let parent = target_dir.parent()
        .ok_or_else(|| "无效路径".to_string())?;
    std::fs::create_dir_all(parent)
        .map_err(|e| format!("创建目录失败: {}", e))?;

    let output = Command::new("git")
        .args(["clone", url, &target_dir.to_string_lossy()])
        .output()
        .map_err(|e| format!("执行 git 失败 (是否已安装 git?): {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "git clone 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// 在已有仓库目录执行 git pull
pub fn pull_repo(repo_dir: &Path) -> Result<String, String> {
    if !repo_dir.exists() {
        return Err(format!("仓库目录不存在: {}", repo_dir.display()));
    }
    let output = Command::new("git")
        .args(["pull"])
        .current_dir(repo_dir)
        .output()
        .map_err(|e| format!("执行 git 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    if output.status.success() {
        Ok(stdout)
    } else {
        Err(format!(
            "git pull 失败: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}
```

- [ ] **Step 2: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/utils/git.rs
git commit -m "feat: add git clone/pull utilities"
```

---

### Task 7: 仓库管理命令

**Files:**
- Create: `src-tauri/src/commands/repo.rs`

- [ ] **Step 1: 占位 skill.rs 和 install.rs**

```rust
// src-tauri/src/commands/skill.rs
// 占位，Task 9 实现
```

```rust
// src-tauri/src/commands/install.rs
// 占位，Task 13 实现
```

- [ ] **Step 2: 实现仓库命令**

```rust
// src-tauri/src/commands/repo.rs
use crate::models::{AppConfig, Repo};
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
```

- [ ] **Step 3: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

- [ ] **Step 4: Commit**

```bash
git add src-tauri/src/commands/repo.rs src-tauri/src/commands/skill.rs src-tauri/src/commands/install.rs
git commit -m "feat: add repository management commands"
```

---

### Task 8: 技能列表解析命令

**Files:**
- Modify: `src-tauri/src/commands/skill.rs`

- [ ] **Step 1: 实现技能解析**

```rust
// src-tauri/src/commands/skill.rs
use crate::models::{Skill, SkillIndex};
use crate::commands::config::load_config_from_disk;
use std::fs;
use std::path::Path;

/// 从仓库目录解析技能列表
/// 优先读取 skills.json 索引文件，否则扫描目录
fn parse_skills_from_repo(repo_dir: &Path, repo_url: &str) -> Vec<Skill> {
    // 尝试读取 skills.json 索引文件
    let index_file = repo_dir.join("skills.json");
    if index_file.exists() {
        if let Ok(content) = fs::read_to_string(&index_file) {
            if let Ok(index) = serde_json::from_str::<SkillIndex>(&content) {
                return index.skills.into_iter().map(|entry| Skill {
                    id: entry.id,
                    name: entry.name,
                    description: entry.description,
                    repo_url: repo_url.to_string(),
                    source_path: repo_dir.join(&entry.path),
                    version: entry.version,
                    author: entry.author,
                    tags: entry.tags,
                }).collect();
            }
        }
    }

    // 回退：扫描顶层目录，每个子目录视为一个技能
    scan_skills_from_dir(repo_dir, repo_url)
}

/// 扫描目录结构识别技能（没有 skills.json 时的回退方案）
fn scan_skills_from_dir(repo_dir: &Path, repo_url: &str) -> Vec<Skill> {
    let mut skills = vec![];

    let entries = match fs::read_dir(repo_dir) {
        Ok(e) => e,
        Err(_) => return skills,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        // 跳过隐藏目录和 .git
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str.starts_with('.') {
            continue;
        }
        if path.is_dir() {
            // 读取目录内的第一个 .md 文件作为描述
            let description = read_first_md_description(&path)
                .unwrap_or_else(|| format!("技能: {}", name_str));
            skills.push(Skill {
                id: name_str.to_string(),
                name: name_str.to_string(),
                description,
                repo_url: repo_url.to_string(),
                source_path: path,
                version: None,
                author: None,
                tags: None,
            });
        }
    }

    skills
}

fn read_first_md_description(dir: &Path) -> Option<String> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).ok()?;
            // 取第一个非空、非 # 开头的行作为描述
            return content.lines()
                .find(|l| !l.trim().is_empty() && !l.starts_with('#'))
                .map(|l| l.trim().to_string());
        }
    }
    None
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
```

- [ ] **Step 2: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/skill.rs
git commit -m "feat: add skill list parsing from repo directory"
```

---

## Phase 4: 技能安装

### Task 9: Junction 工具实现

**Files:**
- Modify: `src-tauri/src/utils/junction.rs`

- [ ] **Step 1: 实现 Junction 工具**

```rust
// src-tauri/src/utils/junction.rs
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
        // 也检查作为 junction 存在但指向不存在目标的情况
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
    // Junction 是一种 reparse point，通过 FILE_ATTRIBUTE_REPARSE_POINT 标志识别
    // 属性值 0x400 = FILE_ATTRIBUTE_REPARSE_POINT
    if let Ok(metadata) = std::fs::symlink_metadata(path) {
        let attrs = metadata.file_attributes();
        return (attrs & 0x400) != 0 && metadata.is_dir();
    }
    false
}

/// 删除 Junction 链接（不删除目标）
pub fn remove_junction(path: &Path) -> Result<(), String> {
    // 使用 rmdir 命令删除 junction（不递归删除内容）
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
/// link_path: junction 所在位置（新建）
/// target_path: junction 指向的目标目录
pub fn create_junction(link_path: &Path, target_path: &Path) -> Result<(), String> {
    if !target_path.exists() {
        return Err(format!("源目录不存在: {}", target_path.display()));
    }

    // 确保父目录存在
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
```

- [ ] **Step 2: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望: 可能有 `windows` cfg 警告，无错误

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/utils/junction.rs
git commit -m "feat: add Windows Junction link utilities"
```

---

### Task 10: 技能安装命令

**Files:**
- Modify: `src-tauri/src/commands/install.rs`

- [ ] **Step 1: 实现安装命令**

```rust
// src-tauri/src/commands/install.rs
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
/// install_type: "global" | "project"
/// tool_type: "claude-code" | "cursor" | ... | "custom"
/// target_path: 目标技能目录（工具的 skills 目录或项目路径）
/// overwrite: 是否覆盖已存在的 junction
/// remember_path: 是否将 target_path 保存为该工具的默认路径
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
    // 获取技能源路径
    let skills = parse_skills_from_repo_url(&repo_url)?;
    let skill = skills.iter()
        .find(|s| s.id == skill_id)
        .ok_or_else(|| format!("技能不存在: {}", skill_id))?;

    // 确定安装目标目录
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

    // 检查冲突
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

    // 保存路径偏好
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
        junction::JunctionStatus::IsDirectory => Err("该路径是普通目录，不是 Junction 链接".to_string()),
    }
}
```

- [ ] **Step 2: 编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/commands/install.rs
git commit -m "feat: add skill install/uninstall commands"
```

---

### Task 11: 注册所有 Tauri 命令

**Files:**
- Modify: `src-tauri/src/lib.rs`

- [ ] **Step 1: 重写 lib.rs**

```rust
// src-tauri/src/lib.rs
mod commands;
mod models;
mod utils;

use commands::{config, install, repo, skill};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // 配置
            config::load_config,
            config::save_config,
            config::get_tool_path,
            config::set_tool_path,
            // 仓库
            repo::add_repo,
            repo::remove_repo,
            repo::update_repo,
            repo::update_all_repos,
            repo::list_repos,
            // 技能
            skill::list_skills,
            // 安装
            install::install_skill,
            install::uninstall_skill,
            install::check_junction_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

- [ ] **Step 2: 完整编译检查**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo check
```

期望输出: 无错误

- [ ] **Step 3: Commit**

```bash
git add src-tauri/src/lib.rs
git commit -m "feat: register all Tauri commands in lib.rs"
```

---

## Phase 5: 前端界面

### Task 12: useRepos 和 useSkills 组合函数

**Files:**
- Create: `src/composables/useRepos.ts`
- Create: `src/composables/useSkills.ts`

- [ ] **Step 1: 创建 useRepos.ts**

```typescript
// src/composables/useRepos.ts
import { ref, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Repo } from '../types';

const repos = ref<Repo[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

export function useRepos() {
  async function loadRepos() {
    loading.value = true;
    error.value = null;
    try {
      const result = await invoke<Repo[]>('list_repos');
      repos.value = result.map(r => ({ ...r, skills: [] }));
    } catch (e) {
      error.value = String(e);
    } finally {
      loading.value = false;
    }
  }

  async function addRepo(url: string): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      const repo = await invoke<Repo>('add_repo', { url });
      repos.value.push({ ...repo, skills: [] });
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      loading.value = false;
    }
  }

  async function removeRepo(url: string): Promise<void> {
    await invoke('remove_repo', { url });
    repos.value = repos.value.filter(r => r.url !== url);
  }

  async function updateRepo(url: string): Promise<string> {
    return await invoke<string>('update_repo', { url });
  }

  async function updateAllRepos(): Promise<string[]> {
    return await invoke<string[]>('update_all_repos');
  }

  return {
    repos: readonly(repos),
    loading: readonly(loading),
    error: readonly(error),
    loadRepos,
    addRepo,
    removeRepo,
    updateRepo,
    updateAllRepos,
  };
}
```

- [ ] **Step 2: 创建 useSkills.ts**

```typescript
// src/composables/useSkills.ts
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Skill } from '../types';

export function useSkills() {
  const skillsByRepo = ref<Record<string, Skill[]>>({});
  const loading = ref(false);

  async function loadSkills(repoUrl: string): Promise<Skill[]> {
    loading.value = true;
    try {
      const skills = await invoke<Skill[]>('list_skills', { repoUrl });
      skillsByRepo.value[repoUrl] = skills;
      return skills;
    } finally {
      loading.value = false;
    }
  }

  return {
    skillsByRepo,
    loading,
    loadSkills,
  };
}
```

- [ ] **Step 3: 类型检查**

```bash
cd d:/code/dev/app/skills-manager
pnpm vue-tsc --noEmit
```

- [ ] **Step 4: Commit**

```bash
git add src/composables/useRepos.ts src/composables/useSkills.ts
git commit -m "feat: add useRepos and useSkills composables"
```

---

### Task 13: useInstall 组合函数

**Files:**
- Create: `src/composables/useInstall.ts`

- [ ] **Step 1: 创建 useInstall.ts**

```typescript
// src/composables/useInstall.ts
import { invoke } from '@tauri-apps/api/core';
import type { InstallType, ToolType } from '../types';

export function useInstall() {
  async function getToolPath(toolType: ToolType): Promise<string> {
    return await invoke<string>('get_tool_path', { toolType });
  }

  async function setToolPath(toolType: ToolType, path: string): Promise<void> {
    await invoke('set_tool_path', { toolType, path });
  }

  async function checkJunctionExists(linkPath: string): Promise<boolean> {
    return await invoke<boolean>('check_junction_exists', { linkPath });
  }

  async function installSkill(params: {
    skillId: string;
    repoUrl: string;
    installType: InstallType;
    toolType: ToolType;
    targetPath: string;
    overwrite?: boolean;
    rememberPath?: boolean;
  }): Promise<void> {
    await invoke('install_skill', {
      skillId: params.skillId,
      repoUrl: params.repoUrl,
      installType: params.installType,
      toolType: params.toolType,
      targetPath: params.targetPath,
      overwrite: params.overwrite ?? false,
      rememberPath: params.rememberPath ?? false,
    });
  }

  async function uninstallSkill(linkPath: string): Promise<void> {
    await invoke('uninstall_skill', { linkPath });
  }

  return {
    getToolPath,
    setToolPath,
    checkJunctionExists,
    installSkill,
    uninstallSkill,
  };
}
```

- [ ] **Step 2: 类型检查**

```bash
pnpm vue-tsc --noEmit
```

- [ ] **Step 3: Commit**

```bash
git add src/composables/useInstall.ts
git commit -m "feat: add useInstall composable"
```

---

### Task 14: SkillCard 和 RepoGroup 组件

**Files:**
- Create: `src/components/SkillCard.vue`
- Create: `src/components/RepoGroup.vue`

- [ ] **Step 1: 创建 SkillCard.vue**

```vue
<!-- src/components/SkillCard.vue -->
<script setup lang="ts">
import type { Skill } from '../types';

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  install: [skill: Skill];
}>();
</script>

<template>
  <div class="skill-card" @click="emit('install', props.skill)">
    <div class="skill-name">{{ skill.name }}</div>
    <div class="skill-description">{{ skill.description }}</div>
    <div class="skill-meta">
      <span v-if="skill.version" class="skill-version">v{{ skill.version }}</span>
      <span v-if="skill.author" class="skill-author">{{ skill.author }}</span>
    </div>
    <div class="skill-tags" v-if="skill.tags?.length">
      <span v-for="tag in skill.tags" :key="tag" class="tag">{{ tag }}</span>
    </div>
    <button class="install-btn">安装</button>
  </div>
</template>

<style scoped>
.skill-card {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  padding: 16px;
  cursor: pointer;
  transition: box-shadow 0.2s, border-color 0.2s;
  background: #fff;
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.skill-card:hover {
  box-shadow: 0 2px 8px rgba(0,0,0,0.12);
  border-color: #646cff;
}
.skill-name {
  font-weight: 600;
  font-size: 1rem;
}
.skill-description {
  font-size: 0.85rem;
  color: #666;
  flex: 1;
}
.skill-meta {
  font-size: 0.75rem;
  color: #999;
  display: flex;
  gap: 8px;
}
.skill-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.tag {
  background: #f0f0ff;
  color: #646cff;
  border-radius: 4px;
  padding: 2px 6px;
  font-size: 0.75rem;
}
.install-btn {
  margin-top: 4px;
  padding: 6px 16px;
  background: #646cff;
  color: #fff;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  font-size: 0.85rem;
  align-self: flex-end;
}
.install-btn:hover {
  background: #535bf2;
}

@media (prefers-color-scheme: dark) {
  .skill-card { background: #1a1a2e; border-color: #333; }
  .skill-description { color: #aaa; }
  .skill-meta { color: #777; }
  .tag { background: #2d2d4e; color: #a0a0ff; }
}
</style>
```

- [ ] **Step 2: 创建 RepoGroup.vue**

```vue
<!-- src/components/RepoGroup.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { Repo, Skill } from '../types';
import { useSkills } from '../composables/useSkills';
import SkillCard from './SkillCard.vue';

const props = defineProps<{
  repo: Repo;
}>();

const emit = defineEmits<{
  installSkill: [skill: Skill];
  removeRepo: [url: string];
  updateRepo: [url: string];
}>();

const { loadSkills } = useSkills();
const skills = ref<Skill[]>([]);
const loading = ref(false);
const collapsed = ref(false);

onMounted(async () => {
  loading.value = true;
  try {
    skills.value = await loadSkills(props.repo.url);
  } finally {
    loading.value = false;
  }
});
</script>

<template>
  <div class="repo-group">
    <div class="repo-header" @click="collapsed = !collapsed">
      <div class="repo-info">
        <span class="repo-name">{{ repo.name }}</span>
        <span class="repo-url">{{ repo.url }}</span>
        <span class="skill-count" v-if="!loading">{{ skills.length }} 个技能</span>
      </div>
      <div class="repo-actions" @click.stop>
        <button @click="emit('updateRepo', repo.url)" title="更新">↻ 更新</button>
        <button class="danger" @click="emit('removeRepo', repo.url)" title="删除">✕ 删除</button>
      </div>
      <span class="collapse-icon">{{ collapsed ? '▶' : '▼' }}</span>
    </div>

    <div v-if="!collapsed" class="repo-skills">
      <div v-if="loading" class="loading">加载技能中...</div>
      <div v-else-if="skills.length === 0" class="empty">未找到技能</div>
      <div v-else class="skills-grid">
        <SkillCard
          v-for="skill in skills"
          :key="skill.id"
          :skill="skill"
          @install="emit('installSkill', $event)"
        />
      </div>
    </div>
  </div>
</template>

<style scoped>
.repo-group {
  border: 1px solid #e0e0e0;
  border-radius: 8px;
  overflow: hidden;
}
.repo-header {
  display: flex;
  align-items: center;
  padding: 12px 16px;
  background: #f8f8f8;
  cursor: pointer;
  gap: 12px;
}
.repo-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
}
.repo-name {
  font-weight: 600;
  font-size: 1rem;
}
.repo-url {
  font-size: 0.75rem;
  color: #888;
}
.skill-count {
  font-size: 0.75rem;
  color: #646cff;
}
.repo-actions {
  display: flex;
  gap: 8px;
}
.repo-actions button {
  padding: 4px 10px;
  font-size: 0.8rem;
  border-radius: 4px;
  cursor: pointer;
  border: 1px solid #ddd;
  background: #fff;
}
.repo-actions button.danger {
  color: #d32f2f;
  border-color: #f44336;
}
.repo-actions button.danger:hover {
  background: #ffebee;
}
.collapse-icon {
  font-size: 0.75rem;
  color: #999;
}
.repo-skills {
  padding: 16px;
}
.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 12px;
}
.loading, .empty {
  text-align: center;
  color: #999;
  padding: 24px;
}

@media (prefers-color-scheme: dark) {
  .repo-group { border-color: #333; }
  .repo-header { background: #1a1a2e; }
  .repo-actions button { background: #222; border-color: #444; color: #eee; }
}
</style>
```

- [ ] **Step 3: 类型检查**

```bash
pnpm vue-tsc --noEmit
```

- [ ] **Step 4: Commit**

```bash
git add src/components/SkillCard.vue src/components/RepoGroup.vue
git commit -m "feat: add SkillCard and RepoGroup components"
```

---

### Task 15: RepoManager 弹窗组件

**Files:**
- Create: `src/components/RepoManager.vue`

- [ ] **Step 1: 创建 RepoManager.vue**

```vue
<!-- src/components/RepoManager.vue -->
<script setup lang="ts">
import { ref } from 'vue';

const emit = defineEmits<{
  add: [url: string];
  close: [];
}>();

const url = ref('');
const error = ref('');

function validate(value: string): string {
  if (!value.trim()) return '请输入仓库 URL';
  if (!value.startsWith('https://github.com/')) {
    return '请输入有效的 GitHub 仓库 URL (https://github.com/...)';
  }
  return '';
}

function handleSubmit() {
  const err = validate(url.value);
  if (err) {
    error.value = err;
    return;
  }
  emit('add', url.value.trim());
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal">
      <h2>添加技能仓库</h2>
      <p class="hint">输入 GitHub 仓库 URL，应用将克隆该仓库到本地</p>
      <form @submit.prevent="handleSubmit">
        <div class="field">
          <label>仓库 URL</label>
          <input
            v-model="url"
            type="text"
            placeholder="https://github.com/username/skills-repo"
            :class="{ error: error }"
            @input="error = ''"
            autofocus
          />
          <span v-if="error" class="error-msg">{{ error }}</span>
        </div>
        <div class="actions">
          <button type="button" @click="emit('close')">取消</button>
          <button type="submit" class="primary">克隆并添加</button>
        </div>
      </form>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  width: 460px;
  max-width: 90vw;
}
h2 { margin: 0 0 8px; }
.hint { color: #888; font-size: 0.85rem; margin-bottom: 20px; }
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 20px;
}
label { font-size: 0.85rem; font-weight: 500; }
input {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.95rem;
  width: 100%;
  box-sizing: border-box;
}
input.error { border-color: #f44336; }
.error-msg { color: #f44336; font-size: 0.8rem; }
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
button {
  padding: 8px 20px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid #ddd;
  background: #f5f5f5;
}
button.primary {
  background: #646cff;
  color: #fff;
  border-color: #646cff;
}
button.primary:hover { background: #535bf2; }

@media (prefers-color-scheme: dark) {
  .modal { background: #1e1e2e; }
  input { background: #2a2a3e; border-color: #444; color: #eee; }
  button { background: #2a2a3e; border-color: #444; color: #eee; }
}
</style>
```

- [ ] **Step 2: 类型检查**

```bash
pnpm vue-tsc --noEmit
```

- [ ] **Step 3: Commit**

```bash
git add src/components/RepoManager.vue
git commit -m "feat: add RepoManager dialog component"
```

---

### Task 16: SkillDialog 安装弹窗组件

**Files:**
- Create: `src/components/SkillDialog.vue`

- [ ] **Step 1: 创建 SkillDialog.vue**

```vue
<!-- src/components/SkillDialog.vue -->
<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { Skill, InstallType, ToolType } from '../types';
import { TOOL_LABELS } from '../utils/toolPaths';
import { useInstall } from '../composables/useInstall';

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  close: [];
  installed: [skillId: string];
}>();

const { getToolPath, setToolPath, installSkill } = useInstall();

const installType = ref<InstallType>('global');
const toolType = ref<ToolType>('claude-code');
const targetPath = ref('');
const projectPath = ref('');
const rememberPath = ref(false);
const loading = ref(false);
const error = ref('');
const overwriteConfirm = ref(false);

const tools: { value: ToolType; label: string }[] = (
  Object.entries(TOOL_LABELS) as [ToolType, string][]
).map(([value, label]) => ({ value, label }));

// 加载工具默认路径
watch(toolType, async (tool) => {
  if (installType.value === 'global') {
    targetPath.value = await getToolPath(tool);
  }
}, { immediate: true });

watch(installType, async (type) => {
  if (type === 'global') {
    targetPath.value = await getToolPath(toolType.value);
  } else {
    targetPath.value = '';
  }
});

const previewPath = computed(() => {
  if (installType.value === 'project') {
    const base = projectPath.value || '<项目路径>';
    return `${base}\\.skills\\${props.skill.id}`;
  }
  const base = targetPath.value || '<工具技能路径>';
  return `${base}\\${props.skill.id}`;
});

async function handleInstall() {
  error.value = '';
  loading.value = true;

  const resolvedTargetPath = installType.value === 'project'
    ? projectPath.value
    : targetPath.value;

  if (installType.value === 'project' && !projectPath.value) {
    error.value = '请输入项目路径';
    loading.value = false;
    return;
  }

  try {
    await installSkill({
      skillId: props.skill.id,
      repoUrl: props.skill.repoUrl,
      installType: installType.value,
      toolType: toolType.value,
      targetPath: resolvedTargetPath,
      overwrite: overwriteConfirm.value,
      rememberPath: rememberPath.value,
    });

    emit('installed', props.skill.id);
    emit('close');
  } catch (e) {
    const msg = String(e);
    if (msg.includes('JUNCTION_EXISTS')) {
      overwriteConfirm.value = false;
      error.value = '该技能已安装。是否覆盖？';
    } else {
      error.value = msg;
    }
  } finally {
    loading.value = false;
  }
}

async function handleOverwrite() {
  overwriteConfirm.value = true;
  await handleInstall();
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')">
    <div class="modal">
      <h2>安装技能: {{ skill.name }}</h2>
      <p class="desc">{{ skill.description }}</p>

      <!-- 安装类型 -->
      <div class="section">
        <label>安装位置</label>
        <div class="radio-group">
          <label class="radio">
            <input type="radio" v-model="installType" value="global" />
            全局安装（工具配置目录）
          </label>
          <label class="radio">
            <input type="radio" v-model="installType" value="project" />
            项目安装（.skills 目录）
          </label>
        </div>
      </div>

      <!-- 全局：选择工具 -->
      <div v-if="installType === 'global'" class="section">
        <label>目标工具</label>
        <select v-model="toolType">
          <option v-for="tool in tools" :key="tool.value" :value="tool.value">
            {{ tool.label }}
          </option>
        </select>
        <label class="path-label">技能目录路径</label>
        <input v-model="targetPath" type="text" placeholder="工具技能目录路径" />
        <label class="checkbox">
          <input type="checkbox" v-model="rememberPath" />
          记住此路径
        </label>
      </div>

      <!-- 项目：输入项目路径 -->
      <div v-else class="section">
        <label>项目路径</label>
        <input
          v-model="projectPath"
          type="text"
          placeholder="例: D:\MyProject"
        />
      </div>

      <!-- 安装路径预览 -->
      <div class="preview">
        <span>安装到:</span>
        <code>{{ previewPath }}</code>
      </div>

      <!-- 错误 + 覆盖确认 -->
      <div v-if="error" class="error-box">
        <span>{{ error }}</span>
        <button v-if="error.includes('已安装')" @click="handleOverwrite" class="overwrite-btn">
          覆盖安装
        </button>
      </div>

      <div class="actions">
        <button @click="emit('close')">取消</button>
        <button class="primary" @click="handleInstall" :disabled="loading">
          {{ loading ? '安装中...' : '安装' }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0,0,0,0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: #fff;
  border-radius: 12px;
  padding: 24px;
  width: 520px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; }
.desc { color: #888; font-size: 0.85rem; margin: 0; }
.section { display: flex; flex-direction: column; gap: 8px; }
label { font-size: 0.85rem; font-weight: 500; }
.radio-group { display: flex; flex-direction: column; gap: 6px; }
.radio { font-weight: normal; display: flex; align-items: center; gap: 8px; cursor: pointer; }
.path-label { margin-top: 8px; }
select, input[type="text"] {
  padding: 8px 12px;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
}
.checkbox { font-weight: normal; display: flex; align-items: center; gap: 6px; cursor: pointer; }
.preview {
  background: #f5f5ff;
  border-radius: 6px;
  padding: 10px 14px;
  font-size: 0.85rem;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.preview span { color: #888; }
.preview code { word-break: break-all; color: #333; }
.error-box {
  background: #ffebee;
  border-radius: 6px;
  padding: 10px 14px;
  color: #c62828;
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 12px;
}
.overwrite-btn {
  padding: 4px 12px;
  background: #d32f2f;
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  white-space: nowrap;
}
.actions { display: flex; justify-content: flex-end; gap: 8px; }
button {
  padding: 8px 20px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid #ddd;
  background: #f5f5f5;
}
button.primary { background: #646cff; color: #fff; border-color: #646cff; }
button.primary:hover:not(:disabled) { background: #535bf2; }
button:disabled { opacity: 0.6; cursor: not-allowed; }

@media (prefers-color-scheme: dark) {
  .modal { background: #1e1e2e; }
  select, input[type="text"] { background: #2a2a3e; border-color: #444; color: #eee; }
  .preview { background: #2a2a4e; }
  .preview code { color: #ccc; }
}
</style>
```

- [ ] **Step 2: 类型检查**

```bash
pnpm vue-tsc --noEmit
```

- [ ] **Step 3: Commit**

```bash
git add src/components/SkillDialog.vue
git commit -m "feat: add SkillDialog install component"
```

---

### Task 17: 重写主应用 App.vue

**Files:**
- Modify: `src/App.vue`

- [ ] **Step 1: 重写 App.vue**

```vue
<!-- src/App.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue';
import type { Skill } from './types';
import { useRepos } from './composables/useRepos';
import RepoGroup from './components/RepoGroup.vue';
import RepoManager from './components/RepoManager.vue';
import SkillDialog from './components/SkillDialog.vue';

const { repos, loading, error, loadRepos, addRepo, removeRepo, updateRepo, updateAllRepos } = useRepos();

const showAddRepo = ref(false);
const selectedSkill = ref<Skill | null>(null);
const statusMessage = ref('');

onMounted(() => loadRepos());

async function handleAddRepo(url: string) {
  try {
    await addRepo(url);
    showAddRepo.value = false;
    setStatus('仓库添加成功');
  } catch (e) {
    // error 已在 useRepos 中设置
  }
}

async function handleUpdateAll() {
  const results = await updateAllRepos();
  setStatus(results.join('\n') || '所有仓库已更新');
}

async function handleUpdateRepo(url: string) {
  const result = await updateRepo(url);
  setStatus(result || '更新完成');
}

async function handleRemoveRepo(url: string) {
  if (!confirm('确定删除该仓库？本地克隆的文件也会被删除。')) return;
  await removeRepo(url);
  setStatus('仓库已删除');
}

function setStatus(msg: string) {
  statusMessage.value = msg;
  setTimeout(() => { statusMessage.value = ''; }, 4000);
}
</script>

<template>
  <div class="app">
    <!-- 顶部栏 -->
    <header class="toolbar">
      <h1>Skills Manager</h1>
      <div class="toolbar-actions">
        <button @click="showAddRepo = true">+ 添加仓库</button>
        <button @click="handleUpdateAll" :disabled="repos.length === 0">↻ 全部更新</button>
      </div>
    </header>

    <!-- 状态消息 -->
    <div v-if="statusMessage" class="status-bar">{{ statusMessage }}</div>
    <div v-if="error" class="error-bar">{{ error }}</div>

    <!-- 主内容 -->
    <main class="content">
      <div v-if="loading" class="loading">加载中...</div>
      <div v-else-if="repos.length === 0" class="empty">
        <p>暂无技能仓库</p>
        <button @click="showAddRepo = true">+ 添加第一个仓库</button>
      </div>
      <div v-else class="repo-list">
        <RepoGroup
          v-for="repo in repos"
          :key="repo.url"
          :repo="repo"
          @install-skill="selectedSkill = $event"
          @remove-repo="handleRemoveRepo"
          @update-repo="handleUpdateRepo"
        />
      </div>
    </main>

    <!-- 弹窗 -->
    <RepoManager
      v-if="showAddRepo"
      @add="handleAddRepo"
      @close="showAddRepo = false"
    />

    <SkillDialog
      v-if="selectedSkill"
      :skill="selectedSkill"
      @close="selectedSkill = null"
      @installed="setStatus('技能安装成功')"
    />
  </div>
</template>

<style>
* { box-sizing: border-box; }
body { margin: 0; font-family: Inter, Avenir, Helvetica, Arial, sans-serif; }
</style>

<style scoped>
.app {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background: #f9f9f9;
}
.toolbar {
  display: flex;
  align-items: center;
  padding: 12px 20px;
  background: #fff;
  border-bottom: 1px solid #e0e0e0;
  gap: 16px;
}
h1 { margin: 0; font-size: 1.2rem; flex: 1; }
.toolbar-actions { display: flex; gap: 8px; }
.toolbar-actions button {
  padding: 6px 16px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid #ddd;
  background: #fff;
  font-size: 0.9rem;
}
.toolbar-actions button:first-child {
  background: #646cff;
  color: #fff;
  border-color: #646cff;
}
.toolbar-actions button:first-child:hover { background: #535bf2; }
.toolbar-actions button:disabled { opacity: 0.5; cursor: not-allowed; }
.status-bar {
  background: #e8f5e9;
  color: #2e7d32;
  padding: 8px 20px;
  font-size: 0.85rem;
  white-space: pre-line;
}
.error-bar {
  background: #ffebee;
  color: #c62828;
  padding: 8px 20px;
  font-size: 0.85rem;
}
.content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}
.repo-list { display: flex; flex-direction: column; gap: 16px; }
.loading, .empty {
  text-align: center;
  padding: 60px 20px;
  color: #999;
}
.empty button {
  margin-top: 12px;
  padding: 8px 20px;
  background: #646cff;
  color: #fff;
  border: none;
  border-radius: 6px;
  cursor: pointer;
}

@media (prefers-color-scheme: dark) {
  .app { background: #121212; }
  .toolbar { background: #1e1e2e; border-color: #333; }
  h1 { color: #eee; }
  .toolbar-actions button { background: #2a2a3e; border-color: #444; color: #eee; }
}
</style>
```

- [ ] **Step 2: 完整类型检查**

```bash
cd d:/code/dev/app/skills-manager
pnpm vue-tsc --noEmit
```

期望输出: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/App.vue
git commit -m "feat: rewrite App.vue as main skills manager UI"
```

---

## Phase 6: 集成验证

### Task 18: 完整构建验证

- [ ] **Step 1: 后端完整编译**

```bash
cd d:/code/dev/app/skills-manager/src-tauri
cargo build
```

期望输出: 编译成功，无错误

- [ ] **Step 2: 前端完整构建**

```bash
cd d:/code/dev/app/skills-manager
pnpm build
```

期望输出: 构建成功，dist/ 目录生成

- [ ] **Step 3: 启动开发模式（手动验证）**

```bash
cd d:/code/dev/app/skills-manager
pnpm tauri dev
```

**手动验证清单:**
- [ ] 应用窗口正常打开，显示"暂无技能仓库"
- [ ] 点击"添加仓库"，弹窗打开
- [ ] 输入有效 GitHub URL（如 `https://github.com/anthropics/claude-code-skills`），点击克隆
  - 如果克隆成功：仓库出现在列表中，技能卡片显示
  - 如果 URL 无效：显示错误信息
- [ ] 点击技能卡片上的"安装"按钮，安装弹窗打开
- [ ] 安装弹窗中切换"全局/项目"选项，路径预览更新
- [ ] 点击安装，检查目标目录是否创建了 Junction 链接

- [ ] **Step 4: 最终 Commit**

```bash
git add -A
git commit -m "feat: complete skills manager implementation"
```

---

## 注意事项

### Junction 链接权限
Windows 创建 Junction 链接通常需要管理员权限或开发者模式。如果安装失败，用户需要：
- 以管理员身份运行应用，或
- 在 Windows 设置中启用"开发者模式"

### Git 依赖
应用依赖系统安装的 `git`。如果 git 未安装，`add_repo` 命令会返回明确错误信息。

### 仓库索引格式
技能仓库根目录应包含 `skills.json` 文件（见设计文档附录）。如果没有，应用会扫描顶层子目录作为技能。

### AppConfig 序列化
Rust 的 `AppConfig` 使用 `#[serde(rename_all = "camelCase")]`，JSON 中字段为 `camelCase`，与 TypeScript 类型一致。
