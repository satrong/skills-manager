# AI 技能管理应用设计文档

**日期**: 2025-04-10
**版本**: 0.1.0
**状态**: 设计阶段

---

## 1. 项目概述

### 1.1 目标

开发一个桌面应用，用于管理 AI 编程工具的技能（skills）。用户可以从 GitHub 仓库克隆技能列表，并将技能通过 Junction 链接方式安装到目标工具的技能目录。

### 1.2 技术栈

- **前端**: Vue 3 + TypeScript + Vite
- **后端**: Rust + Tauri v2
- **平台**: Windows 10+

---

## 2. 功能需求

### 2.1 仓库管理

- 添加 GitHub 技能仓库 URL
- 删除已添加的仓库
- 克隆仓库到本地
- 更新仓库（git pull）
- 支持多仓库合并显示

### 2.2 技能展示

- 按仓库分组显示技能列表
- 解析仓库索引文件识别技能
- 显示技能名称、描述、版本等信息

### 2.3 技能安装

- 选择全局安装或项目安装
- 选择目标工具（Claude Code、Cursor、Codex、Opencode、Qoder、Kilo）
- 支持自定义目标路径
- 使用 Windows Junction 链接而非复制
- 记住自定义路径配置

### 2.4 错误处理

- 仓库克隆失败
- Junction 链接已存在（提示是否覆盖）
- 项目路径无效
- 更新仓库失败

---

## 3. 架构设计

### 3.1 前端结构

```
src/
├── App.vue                    # 主应用入口
├── main.ts                    # Vue 初始化
├── components/
│   ├── RepoGroup.vue          # 仓库分组组件
│   ├── SkillCard.vue          # 技能卡片组件
│   ├── SkillDialog.vue        # 安装弹窗组件
│   └── RepoManager.vue        # 仓库管理组件
├── composables/
│   ├── useRepos.ts            # 仓库数据管理
│   ├── useSkills.ts           # 技能数据管理
│   └── useInstall.ts          # 安装逻辑
├── types/
│   └── index.ts               # TypeScript 类型定义
└── utils/
    └── toolPaths.ts           # 工具路径配置
```

### 3.2 后端结构

```
src-tauri/src/
├── lib.rs                     # Tauri 命令注册
├── commands/
│   ├── mod.rs                 # 命令模块导出
│   ├── repo.rs                # 仓库克隆/更新/删除
│   ├── skill.rs               # 技能列表读取/解析
│   ├── install.rs             # 技能安装（junction 创建）
│   └── config.rs              # 配置读写
├── models/
│   ├── mod.rs                 # 数据模型
│   └── repo.rs                # 仓库、技能结构体
└── utils/
    ├── mod.rs                 # 工具模块
    ├── git.rs                 # Git 操作封装
    ├── junction.rs            # Junction 链接创建
    └── paths.rs               # 路径工具
```

### 3.3 数据流

```
用户操作 → Vue 组件 → Tauri invoke → Rust 命令 → 文件系统/Git
                ↑                           ↓
                └─────────── 响应 ──────────┘
```

---

## 4. 工具路径配置

### 4.1 默认全局路径

| 工具 | 技能目录路径 |
|------|-------------|
| Claude Code | `%USERPROFILE%\.claude\skills` |
| Cursor | `%USERPROFILE%\.cursor\skills` |
| Codex | `%USERPROFILE%\.codex\skills` |
| Opencode | `%USERPROFILE%\.config\opencode\skills` |
| Qoder | `%USERPROFILE%\.qoder\skills` |
| Kilo | `%USERPROFILE%\.kilocode\skills` |

### 4.2 项目级安装

- 路径: `<项目路径>\.skills\`
- 用户手动输入完整项目路径

---

## 5. 数据模型

### 5.1 TypeScript 类型

```typescript
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
  repos: Repo[];
  toolPaths: Record<ToolType, string>;
}
```

### 5.2 Rust 结构体

```rust
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
pub enum InstallType {
    Global,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolType {
    ClaudeCode,
    Cursor,
    Codex,
    Opencode,
    Qoder,
    Kilo,
    Custom(String),
}
```

---

## 6. Tauri 命令接口

### 6.1 仓库管理

```rust
#[tauri::command]
async fn add_repo(url: String) -> Result<Repo, String>

#[tauri::command]
async fn remove_repo(url: String) -> Result<(), String>

#[tauri::command]
async fn update_repo(url: String) -> Result<String, String>

#[tauri::command]
async fn update_all_repos() -> Result<Vec<String>, String>

#[tauri::command]
async fn list_repos() -> Result<Vec<Repo>, String>
```

### 6.2 技能管理

```rust
#[tauri::command]
async fn list_skills(repo_url: String) -> Result<Vec<Skill>, String>
```

### 6.3 技能安装

```rust
#[tauri::command]
async fn install_skill(
    skill_id: String,
    repo_url: String,
    install_type: InstallType,
    tool_type: ToolType,
    target_path: String,
    remember_path: bool,
) -> Result<(), String>
```

### 6.4 配置管理

```rust
#[tauri::command]
async fn load_config() -> Result<AppConfig, String>

#[tauri::command]
async fn save_config(config: AppConfig) -> Result<(), String>

#[tauri::command]
async fn get_tool_path(tool_type: ToolType) -> Result<String, String>

#[tauri::command]
async fn set_tool_path(tool_type: ToolType, path: String) -> Result<(), String>
```

---

## 7. UI 设计

### 7.1 主界面

- **布局**: 单一列表 + 仓库分组
- **顶部**: 标题 + [添加仓库] [全部更新] 按钮
- **内容**: 按仓库分组，每组内显示技能卡片网格

### 7.2 安装弹窗

- 选择安装类型（全局/项目）
- 选择目标工具
- 自定义目标路径
- 选项：记住路径
- 显示最终安装位置预览

### 7.3 添加仓库弹窗

- 输入仓库 URL
- 支持的格式提示
- 验证 URL 有效性

---

## 8. 存储设计

### 8.1 配置文件

- **位置**: `%USERPROFILE%\.skills-manager\config.json`
- **内容**: 仓库列表、工具路径配置

### 8.2 仓库存储

- **位置**: `%USERPROFILE%\.skills-manager\repos\`
- **命名**: `<仓库名>-<用户名>.git`

### 8.3 配置文件结构

```json
{
  "repos": [
    {
      "url": "https://github.com/anthropics/skills",
      "localPath": "C:\\Users\\...\\.skills-manager\\repos\\anthropics-skills",
      "name": "anthropics-skills",
      "lastUpdate": "2025-04-10T10:30:00Z"
    }
  ],
  "toolPaths": {
    "claude-code": "C:\\Users\\chc\\.claude\\skills",
    "cursor": "C:\\Users\\chc\\.cursor\\skills",
    "custom-tool-1": "D:\\tools\\my-tool\\skills"
  }
}
```

---

## 9. 安装机制

### 9.1 Junction 创建

使用 PowerShell 命令创建链接：

```powershell
New-Item -ItemType Junction -Path "<link_path>" -Target "<source_path>"
```

### 9.2 链接规则

- **全局安装**: `<工具技能路径>\<技能ID>` → `<仓库本地路径>\<技能源路径>`
- **项目安装**: `<项目路径>\.skills\<技能ID>` → `<仓库本地路径>\<技能源路径>`

### 9.3 冲突处理

- 如果目标位置已存在 Junction，提示用户选择：
  - 覆盖（删除旧链接，创建新链接）
  - 取消操作

---

## 10. 错误处理

### 10.1 错误类型

```rust
pub enum AppError {
    RepoCloneFailed(String),
    RepoUpdateFailed(String),
    JunctionExists(String),
    InvalidProjectPath(String),
    GitNotFound,
    IndexFileNotFound,
    InvalidUrl(String),
}
```

### 10.2 用户提示

- 克隆失败：显示具体错误信息，建议检查 URL 或网络
- 链接已存在：弹窗询问是否覆盖
- 路径无效：高亮输入框，提示正确格式
- 更新失败：显示哪个仓库更新失败及原因

---

## 11. 开发计划

1. **Phase 1**: 基础架构搭建
   - 设置项目结构
   - 实现配置读写
   - 实现基础 UI 布局

2. **Phase 2**: 仓库管理
   - 添加/删除仓库
   - Git 克隆和更新
   - 技能列表解析

3. **Phase 3**: 安装功能
   - 安装弹窗 UI
   - Junction 创建逻辑
   - 错误处理

4. **Phase 4**: 完善和优化
   - UI 细节优化
   - 错误提示完善
   - 配置持久化

---

## 12. 附录

### 12.1 示例仓库

- https://github.com/anthropics/skills

### 12.2 索引文件格式

技能仓库应包含索引文件（如 `skills.json`）：

```json
{
  "skills": [
    {
      "id": "code-review",
      "name": "Code Review",
      "description": "代码审查技能",
      "path": "src/skills/code-review",
      "version": "1.0.0"
    }
  ]
}
```

如果没有索引文件，则扫描目录结构识别技能。
