# 版本更新检测功能设计

## 概述

为 Skills Manager 添加基于 GitHub Releases 的版本更新检测与自动下载安装功能。使用 Tauri 官方 `tauri-plugin-updater` 插件实现。

## 需求

- 应用启动时自动检查是否有新版本
- 设置页面提供手动"检查更新"按钮
- 发现新版本时弹出更新对话框，显示版本号和更新内容
- 支持下载进度显示
- 下载完成后提示用户重启安装
- 网络异常时静默跳过自动检查，手动检查时显示错误提示

## 架构

```
GitHub Releases (静态 JSON endpoint)
        │
        ▼
tauri-plugin-updater (Rust 插件)
        │
        ▼
前端 composable: useUpdate.ts
        │
        ├── App.vue (启动时自动检查)
        └── SettingsDialog.vue (手动检查按钮 + 版本信息)
```

## 组件设计

### 1. Rust 后端

#### 依赖变更（Cargo.toml）

- 添加 `tauri-plugin-updater = "2"`

#### 插件注册（lib.rs）

- 在 `run()` 函数中注册 `tauri_plugin_updater::Builder::new()` 插件

#### Tauri 配置（tauri.conf.json）

- 添加 `plugins.updater` 配置节：
  - `endpoints`：指向 GitHub Releases 的 JSON endpoint
  - `pubkey`：签名公钥（如启用签名验证）

#### 权限（capabilities/default.json）

- 添加 `updater:default` 权限

### 2. 前端 composable：`src/composables/useUpdate.ts`

模块级单例状态，与现有 composables 模式一致。

**响应式状态：**
- `updateAvailable: Ref<boolean>` — 是否有可用更新
- `latestVersion: Ref<string>` — 最新版本号
- `updateBody: Ref<string>` — 更新说明（changelog）
- `checking: Ref<boolean>` — 是否正在检查
- `downloading: Ref<boolean>` — 是否正在下载
- `downloadProgress: Ref<number>` — 下载进度（0-100）
- `error: Ref<string>` — 错误信息

**方法：**
- `checkForUpdate(): Promise<void>` — 检查更新
- `downloadAndInstall(): Promise<void>` — 下载并安装更新

**使用插件：** `@tauri-apps/plugin-updater`

### 3. UI 变更

#### UpdateDialog.vue（新增组件）

更新提示/下载对话框：
- 显示当前版本和新版本号
- 显示更新说明（changelog/body）
- "立即更新"按钮 → 触发下载
- 下载进度条
- 下载完成后"重启并安装"按钮
- "稍后提醒"关闭按钮

#### SettingsDialog.vue（修改）

在版本区域增加：
- "检查更新"按钮
- 更新状态显示（已是最新 / 有新版本 / 正在检查）
- 有更新时点击跳转 UpdateDialog

#### App.vue（修改）

- 在 `onMounted` 中调用 `checkForUpdate()`
- 监听 `updateAvailable`，有更新时弹出 `UpdateDialog`

### 4. 数据流

1. 应用启动 → App.vue `onMounted` 调用 `checkForUpdate()`
2. 插件请求配置的 endpoint 获取版本信息
3. 比较当前版本与最新版本
4. 有新版本 → 设置 `updateAvailable = true`，弹出 UpdateDialog
5. 用户点击"立即更新" → 调用 `downloadAndInstall()`
6. 显示下载进度
7. 下载完成 → 调用插件 `install()` 并提示重启

### 5. 错误处理

- **网络失败（自动检查）：** 静默跳过，不弹出提示
- **网络失败（手动检查）：** 通过 Toast 显示错误信息
- **无新版本：** 手动检查时 Toast 提示"已是最新版本"
- **下载中断：** 显示错误，允许重新检查/重试

## GitHub Releases 配置

需要在仓库中配置更新 endpoint。Tauri v2 updater 支持两种方式：

1. **静态 JSON 文件**：在仓库中维护 `latest.json`，发布时更新
2. **GitHub Actions 生成**：使用 `tauri-action` 自动在 release 时生成更新 manifest

推荐使用方式 2（`tauri-action`），发布流程自动化程度更高。

endpoint URL 格式示例：
```
https://github.com/satrong/skills-manager/releases/latest/download/latest.json
```

## 文件变更清单

| 文件 | 变更类型 | 说明 |
|---|---|---|
| `src-tauri/Cargo.toml` | 修改 | 添加 tauri-plugin-updater 依赖 |
| `src-tauri/src/lib.rs` | 修改 | 注册 updater 插件 |
| `src-tauri/tauri.conf.json` | 修改 | 添加 plugins.updater 配置 |
| `src-tauri/capabilities/default.json` | 修改 | 添加 updater 权限 |
| `package.json` | 修改 | 添加 @tauri-apps/plugin-updater |
| `src/composables/useUpdate.ts` | 新增 | 更新逻辑 composable |
| `src/components/UpdateDialog.vue` | 新增 | 更新对话框组件 |
| `src/App.vue` | 修改 | 启动时检查更新 + 弹出更新对话框 |
| `src/components/SettingsDialog.vue` | 修改 | 添加手动检查更新按钮 |

## 约束

- 签名验证：Tauri updater 默认需要签名，开发阶段可配置为不验证签名，生产环境建议启用
- 平台支持：需确保 GitHub Releases 包含各平台（macOS、Windows、Linux）的构建产物
- 更新 endpoint 的 JSON 格式必须符合 Tauri v2 updater 规范
