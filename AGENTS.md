# AGENTS.md

面向在该仓库中工作的智能编码代理的指南。

## 项目概述

Tauri v2 桌面应用，用于管理 AI 编码工具（Claude Code、Cursor、Codex、OpenCode、Qoder、Kilo）的技能（插件/规则）。前端使用 Vue 3 + TypeScript，后端使用 Rust。界面语言为简体中文。

## 构建与开发命令

```bash
# 开发模式（Vite 开发服务器运行在 1420 端口）
pnpm dev

# 类型检查 Vue/TS 文件（不输出）
vue-tsc --noEmit

# 生产构建（类型检查 + vite 构建）
pnpm build

# 以开发模式运行 Tauri（自动启动 pnpm dev）
pnpm tauri dev

# 构建桌面应用
pnpm tauri build
```

### Rust 后端

```bash
# 检查 Rust 编译
cargo check
# 从项目根目录使用：
cargo check --manifest-path src-tauri/Cargo.toml

# 运行所有 Rust 测试
cargo test --manifest-path src-tauri/Cargo.toml

# 按名称运行单个 Rust 测试
cargo test --manifest-path src-tauri/Cargo.toml test_repo_dir_name

# 运行指定模块中的测试
cargo test --manifest-path src-tauri/Cargo.toml --lib utils::paths::tests
```

### 代码检查 / 类型检查

- **前端类型检查：** `vue-tsc --noEmit`（严格模式，不允许未使用的局部变量和参数）
- **Rust：** 未发现 clippy 配置，但提交前应运行 `cargo clippy --manifest-path src-tauri/Cargo.toml`
- 未配置 ESLint 或 Prettier —— 严格遵循现有代码风格

### 测试

未配置前端测试框架。测试仅存在于 Rust 端（如 `src-tauri/src/utils/paths.rs`）。没有统一的前端测试命令。

## 架构

```
src/                        # Vue 3 前端
  main.ts                   # 入口文件
  App.vue                   # 根组件
  types/index.ts            # 共享 TypeScript 接口和类型
  composables/              # Vue 组合式函数（共享响应式状态）
    useRepos.ts             # 仓库增删改查（模块级单例状态）
    useSkills.ts            # 技能列表与缓存
    useInstall.ts           # 通过 Tauri invoke 安装/卸载
    useSettings.ts          # 应用设置（默认工具、项目路径）
    useToast.ts             # Toast 通知状态
    useTheme.ts             # 亮色/暗色/自动主题切换
    useSearch.ts            # 搜索组合式函数
  components/               # Vue SFC 组件
  utils/                    # 纯工具函数

src-tauri/src/              # Rust 后端
  main.rs                   # 入口文件 → 调用 lib::run()
  lib.rs                    # Tauri 构建器，注册所有命令
  commands/                 # #[tauri::command] 处理函数
    config.rs               # 配置加载/保存、工具路径、项目路径
    repo.rs                 # 仓库添加/删除/更新/列表（git clone/pull）
    skill.rs                # 技能列表与搜索（解析 SKILL.md 或 skills.json）
    install.rs              # 通过文件系统 junction 安装/卸载
  models/
    repo.rs                 # Serde 结构体：Repo、Skill、AppConfig、SkillIndex
  utils/
    paths.rs                # 配置目录、工具路径、路径展开
    git.rs                  # git clone/pull 辅助函数
    junction.rs             # Junction（符号链接）创建/删除/检查
```

### 前端 ↔ 后端通信

- 前端通过 `@tauri-apps/api/core` 的 `invoke('command_name', { camelCaseParams })` 调用 Rust
- Rust 命令使用 `#[tauri::command]` 并在 `lib.rs` 的 `tauri::generate_handler![]` 中注册
- Rust 结构体字段使用 `#[serde(rename_all = "camelCase")]` 以匹配 JS 命名规范
- Rust 中的命令参数名必须使用 snake_case；Tauri 自动从 JS 的 camelCase 转换

## 代码风格

### TypeScript / Vue

- 使用 `<script setup lang="ts">` 的 **Vue 3 组合式 API** —— 始终使用此模式
- **导入顺序：** 先导入 Vue，再导入外部包，最后用 `../` 相对路径导入本地模块
- 仅用于类型的导入使用 `import type { ... }`
- 组合式函数**不使用默认导出** —— 使用命名导出 `export function useXxx()`
- **类型：** 在 `src/types/index.ts` 中定义接口；联合类型用 `type`，对象用 `interface`
- **响应式：** 使用 `ref<T>()` 并显式指定泛型类型；派生状态使用 `computed()`
- **状态模式：** 组合式函数使用模块级 ref（单例）—— 状态在组件挂载间持久化
- **错误处理：** 组合式函数暴露响应式的 `error` ref；调用方通过 watch 或 try/catch 处理
- 已启用 **noUnusedLocals + noUnusedParameters** —— 不允许未使用的变量
- **分号：** 使用不一致；组合式函数中倾向不加分号以匹配主要风格
- **字符串：** TS 中倾向使用单引号，HTML 模板中使用双引号
- **模板引用：** 使用 `ref<HTMLElement | null>(null)`

### CSS

- SFC 中使用 `<style scoped>` 块的局部样式
- 使用 CSS 自定义属性（变量）进行主题化：`var(--border)`、`var(--text-primary)` 等
- 无 CSS 预处理器 —— 仅使用纯 CSS
- 使用主题系统中的 `var(--primary)`、`var(--danger)` 等变量

### Rust

- **Serde：** 所有 API 结构体派生 `Serialize, Deserialize` 并添加 `#[serde(rename_all = "camelCase")]`
- **错误处理：** Tauri 命令返回 `Result<T, String>` —— 使用 `.map_err(|e| format!("中文描述: {}", e))?`
- **可见性：** 内部辅助函数使用 `pub(crate)` 或私有；仅 `#[tauri::command]` 函数为 `pub`
- **异步：** 所有命令均为 `pub async fn`，即使不使用 async await
- **注释：** Rust 代码库中使用中文注释 —— 保持此惯例
- **模块结构：** `mod.rs` 使用 `pub use module::*` 重新导出
- **格式化：** 标准 `rustfmt` 风格 —— 运行 `cargo fmt --manifest-path src-tauri/Cargo.toml`

## 关键约定

- Vite 开发服务器必须运行在 **1420 端口**（Tauri 硬编码要求）
- 应用配置存储在 `~/.skills-manager/config.json`
- 仓库克隆到 `~/.skills-manager/repos/`
- 技能通过文件系统 junction 安装（非复制/符号链接）
- 技能元数据来自 `skills.json` 索引文件或每个技能目录中的 `SKILL.md` frontmatter
- 支持的工具类型：`claude-code`、`cursor`、`codex`、`opencode`、`qoder`、`kilo`、`custom`
- 面向用户的消息和错误字符串使用中文
