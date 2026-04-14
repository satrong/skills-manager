# 版本更新检测功能 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 基于 tauri-plugin-updater 实现应用版本更新检测、下载进度显示和安装功能，支持启动时自动检查和设置页手动检查。

**Architecture:** Rust 端注册 `tauri-plugin-updater` 插件，配置 GitHub Releases endpoint。前端新增 `useUpdate` composable 管理更新状态（模块级单例），`UpdateDialog` 组件显示更新/下载界面。`App.vue` 启动时自动检查，`SettingsDialog` 提供手动检查按钮。

**Tech Stack:** tauri-plugin-updater (Rust + JS bindings), Vue 3 Composition API, TypeScript

---

## File Structure

| Action | File | Responsibility |
|--------|------|----------------|
| Modify | `src-tauri/Cargo.toml` | 添加 tauri-plugin-updater 依赖 |
| Modify | `src-tauri/src/lib.rs` | 注册 updater 插件 |
| Modify | `src-tauri/tauri.conf.json` | 添加 plugins.updater 配置 |
| Modify | `src-tauri/capabilities/default.json` | 添加 updater 权限 |
| Modify | `package.json` | 添加 @tauri-apps/plugin-updater |
| Create | `src/composables/useUpdate.ts` | 更新逻辑 composable（检查、下载、状态管理） |
| Create | `src/components/UpdateDialog.vue` | 更新对话框组件（版本信息、changelog、进度条、重启按钮） |
| Modify | `src/App.vue` | 启动时自动检查 + 弹出 UpdateDialog |
| Modify | `src/components/SettingsDialog.vue` | 添加手动检查更新按钮和状态显示 |
| Modify | `src/i18n/zh-CN.ts` | 添加更新相关中文翻译 |
| Modify | `src/i18n/en.ts` | 添加更新相关英文翻译 |
| Modify | `.github/workflows/release.yml` | 升级 tauri-action 版本，添加 tagName 参数以自动生成 latest.json |

---

### Task 1: Rust 后端 — 添加 tauri-plugin-updater 依赖

**Files:**
- Modify: `src-tauri/Cargo.toml:20-27`
- Modify: `src-tauri/src/lib.rs:9-11`
- Modify: `src-tauri/tauri.conf.json` (添加 plugins 配置)
- Modify: `src-tauri/capabilities/default.json:6-10`

- [ ] **Step 1: 在 `Cargo.toml` 添加依赖**

在 `src-tauri/Cargo.toml` 的 `[dependencies]` 中，在 `tauri-plugin-dialog` 行之后添加：

```toml
tauri-plugin-updater = "2"
```

- [ ] **Step 2: 在 `lib.rs` 注册插件**

在 `src-tauri/src/lib.rs` 的 `run()` 函数中，在 `.plugin(tauri_plugin_dialog::init())` 行之后添加：

```rust
        .plugin(tauri_plugin_updater::Builder::new().build())
```

- [ ] **Step 3: 在 `tauri.conf.json` 添加 updater 配置**

在 `src-tauri/tauri.conf.json` 的顶层对象中，`bundle` 之前添加：

```json
  "plugins": {
    "updater": {
      "endpoints": [
        "https://github.com/satrong/skills-manager/releases/latest/download/latest.json"
      ],
      "pubkey": ""
    }
  },
```

- [ ] **Step 4: 在 `capabilities/default.json` 添加权限**

在 `src-tauri/capabilities/default.json` 的 `permissions` 数组中，在 `"dialog:default"` 之后添加：

```json
    "updater:default"
```

- [ ] **Step 5: 验证 Rust 编译**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: 编译成功，无错误

- [ ] **Step 6: Commit**

```bash
git add src-tauri/Cargo.toml src-tauri/src/lib.rs src-tauri/tauri.conf.json src-tauri/capabilities/default.json
git commit -m "feat: add tauri-plugin-updater dependency and configuration"
```

---

### Task 2: 前端 — 安装 @tauri-apps/plugin-updater

**Files:**
- Modify: `package.json`

- [ ] **Step 1: 安装 npm 包**

Run: `pnpm add @tauri-apps/plugin-updater`

- [ ] **Step 2: 验证 package.json 已更新**

Run: `cat package.json`
Expected: `dependencies` 中包含 `"@tauri-apps/plugin-updater": "^2"`

- [ ] **Step 3: Commit**

```bash
git add package.json pnpm-lock.yaml
git commit -m "feat: add @tauri-apps/plugin-updater npm package"
```

---

### Task 3: CI — 修改 release.yml 以支持自动生成 latest.json

**Files:**
- Modify: `.github/workflows/release.yml:151-157`

当前 workflow 使用 `tauri-apps/tauri-action@v0` 且只传了 `releaseId`，未传 `tagName`。`tauri-action` 的 `uploadUpdaterJson` 默认为 `true`，只要 `tauri.conf.json` 配置了 `plugins.updater` 就会自动生成 `latest.json`。但需要传 `tagName` 才能让 `latest.json` 中的下载 URL 指向正确的 tag release（否则会指向 `releases/latest/download/`）。

- [ ] **Step 1: 升级 tauri-action 版本并添加 tagName**

将 `.github/workflows/release.yml` 第 151-157 行：

```yaml
      - name: Build the app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          releaseId: ${{ needs.create-release.outputs.release_id }}
          args: ${{ matrix.args }}
```

替换为：

```yaml
      - name: Build the app
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          tagName: ${{ needs.create-release.outputs.tag_name }}
          releaseId: ${{ needs.create-release.outputs.release_id }}
          args: ${{ matrix.args }}
```

- [ ] **Step 2: 验证 YAML 语法**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"`
Expected: 无输出（语法正确）

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add tagName to tauri-action for latest.json generation"
```

---

### Task 4: 前端 — 添加 i18n 翻译键

**Files:**
- Modify: `src/i18n/zh-CN.ts:93`
- Modify: `src/i18n/en.ts:94`

- [ ] **Step 1: 在 `zh-CN.ts` 添加翻译**

在 `src/i18n/zh-CN.ts` 的 `'settings.version': '版本',` 之后、`'skills.autoAddedBuiltin'` 之前添加：

```typescript
  'settings.checkUpdate': '检查更新',
  'settings.checking': '检查中...',
  'settings.upToDate': '已是最新版本',
  'settings.updateAvailable': '发现新版本',
  'settings.updateError': '检查更新失败',
  'update.title': '发现新版本',
  'update.currentVersion': '当前版本',
  'update.newVersion': '新版本',
  'update.changelog': '更新内容',
  'update.updateNow': '立即更新',
  'update.downloading': '下载中...',
  'update.downloadProgress': '下载中... {progress}%',
  'update.restartInstall': '重启并安装',
  'update.later': '稍后提醒',
  'update.downloadError': '下载失败',
```

- [ ] **Step 2: 在 `en.ts` 添加对应英文翻译**

在 `src/i18n/en.ts` 的 `'settings.version': 'Version',` 之后、`'skills.autoAddedBuiltin'` 之前添加：

```typescript
  'settings.checkUpdate': 'Check for updates',
  'settings.checking': 'Checking...',
  'settings.upToDate': 'Already up to date',
  'settings.updateAvailable': 'Update available',
  'settings.updateError': 'Update check failed',
  'update.title': 'New version available',
  'update.currentVersion': 'Current version',
  'update.newVersion': 'New version',
  'update.changelog': 'What\'s new',
  'update.updateNow': 'Update now',
  'update.downloading': 'Downloading...',
  'update.downloadProgress': 'Downloading... {progress}%',
  'update.restartInstall': 'Restart & install',
  'update.later': 'Remind me later',
  'update.downloadError': 'Download failed',
```

- [ ] **Step 3: 验证类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误（新增键已在 zh-CN 中定义类型，en.ts 使用 `Record<TranslationKeys, string>` 约束）

- [ ] **Step 4: Commit**

```bash
git add src/i18n/zh-CN.ts src/i18n/en.ts
git commit -m "feat: add i18n keys for version update feature"
```

---

### Task 5: 前端 — 创建 useUpdate composable

**Files:**
- Create: `src/composables/useUpdate.ts`

- [ ] **Step 1: 创建 `useUpdate.ts`**

创建 `src/composables/useUpdate.ts`：

```typescript
import { ref } from 'vue'
import { check, type Update } from '@tauri-apps/plugin-updater'

const updateAvailable = ref(false)
const latestVersion = ref('')
const updateBody = ref('')
const checking = ref(false)
const downloading = ref(false)
const downloadProgress = ref(0)
const error = ref('')
let pendingUpdate: Update | null = null
let totalContentLength = 0
let downloadedLength = 0

async function checkForUpdate() {
  checking.value = true
  error.value = ''
  try {
    const update = await check()
    if (update) {
      updateAvailable.value = true
      latestVersion.value = update.version
      updateBody.value = update.body ?? ''
      pendingUpdate = update
    } else {
      updateAvailable.value = false
      latestVersion.value = ''
      updateBody.value = ''
      pendingUpdate = null
    }
  } catch (e) {
    error.value = String(e)
    throw e
  } finally {
    checking.value = false
  }
}

async function downloadAndInstall() {
  if (!pendingUpdate) return
  downloading.value = true
  downloadProgress.value = 0
  error.value = ''
  downloadedLength = 0
  totalContentLength = 0
  try {
    await pendingUpdate.download((event) => {
      switch (event.event) {
        case 'Started':
          totalContentLength = event.data.contentLength ?? 0
          break
        case 'Progress':
          downloadedLength += event.data.chunkLength
          if (totalContentLength > 0) {
            downloadProgress.value = Math.round((downloadedLength / totalContentLength) * 100)
          }
          break
        case 'Finished':
          downloadProgress.value = 100
          break
      }
    })
    await pendingUpdate.install()
  } catch (e) {
    error.value = String(e)
    throw e
  } finally {
    downloading.value = false
  }
}

export function useUpdate() {
  return {
    updateAvailable,
    latestVersion,
    updateBody,
    checking,
    downloading,
    downloadProgress,
    error,
    checkForUpdate,
    downloadAndInstall,
  }
}
```

- [ ] **Step 2: 验证类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/composables/useUpdate.ts
git commit -m "feat: add useUpdate composable for version update logic"
```

---

### Task 6: 前端 — 创建 UpdateDialog 组件

**Files:**
- Create: `src/components/UpdateDialog.vue`

- [ ] **Step 1: 创建 `UpdateDialog.vue`**

创建 `src/components/UpdateDialog.vue`：

```vue
<script setup lang="ts">
import { ref, computed } from 'vue'
import { useUpdate } from '../composables/useUpdate'
import { useI18n } from '../i18n'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  close: []
}>()

const { latestVersion, updateBody, downloading, downloadProgress, downloadAndInstall } = useUpdate()
const { t } = useI18n()

const currentVersion = ref('')
const downloadError = ref('')

async function loadVersion() {
  try {
    currentVersion.value = await invoke<string>('get_app_version')
  } catch {
    currentVersion.value = ''
  }
}
loadVersion()

const progressPercent = computed(() => Math.min(downloadProgress.value, 100))

async function handleUpdate() {
  downloadError.value = ''
  try {
    await downloadAndInstall()
  } catch {
    downloadError.value = t('update.downloadError')
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
    <div class="modal">
      <h2>{{ t('update.title') }}</h2>

      <div class="version-info">
        <div class="version-row">
          <span class="version-label">{{ t('update.currentVersion') }}</span>
          <span class="version-value">v{{ currentVersion }}</span>
        </div>
        <div class="version-row">
          <span class="version-label">{{ t('update.newVersion') }}</span>
          <span class="version-value highlight">v{{ latestVersion }}</span>
        </div>
      </div>

      <div v-if="updateBody" class="changelog">
        <label>{{ t('update.changelog') }}</label>
        <pre class="changelog-body">{{ updateBody }}</pre>
      </div>

      <div v-if="downloading" class="progress-section">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
        </div>
        <span class="progress-text">{{ t('update.downloadProgress', { progress: progressPercent }) }}</span>
      </div>

      <div v-if="downloadError" class="error-box">
        {{ downloadError }}
      </div>

      <div class="actions">
        <button v-if="!downloading" @click="emit('close')">{{ t('update.later') }}</button>
        <button
          v-if="progressPercent >= 100"
          class="primary"
          @click="() => { /* updater handles restart */ }"
        >
          {{ t('update.restartInstall') }}
        </button>
        <button
          v-else
          class="primary"
          @click="handleUpdate"
          :disabled="downloading"
        >
          {{ downloading ? t('update.downloading') : t('update.updateNow') }}
        </button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: var(--bg-scrim);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  width: 460px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; color: var(--text-primary); }
.version-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.version-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.version-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}
.version-value {
  font-size: 0.85rem;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
.version-value.highlight {
  color: var(--primary);
  font-weight: 600;
}
.changelog {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.changelog label {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
}
.changelog-body {
  background: var(--bg-surface-sunken);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 12px;
  font-size: 0.8rem;
  color: var(--text-secondary);
  max-height: 200px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-break: break-word;
  margin: 0;
}
.progress-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.progress-bar {
  height: 6px;
  background: var(--bg-surface-sunken);
  border-radius: 3px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: var(--primary);
  border-radius: 3px;
  transition: width 0.3s ease;
}
.progress-text {
  font-size: 0.8rem;
  color: var(--text-secondary);
  text-align: center;
}
.error-box {
  background: var(--danger-light);
  border-radius: 6px;
  padding: 10px 14px;
  color: var(--danger);
  font-size: 0.85rem;
}
.actions { display: flex; justify-content: flex-end; gap: 8px; }
button {
  padding: 8px 20px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid var(--border);
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  font-size: 0.85rem;
  transition: background 0.15s;
}
button:hover { background: var(--bg-surface); }
button.primary { background: var(--primary); color: #fff; border-color: var(--primary); }
button.primary:hover:not(:disabled) { background: var(--primary-hover); }
button:disabled { opacity: 0.6; cursor: not-allowed; }
</style>
```

- [ ] **Step 2: 验证类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误

- [ ] **Step 3: Commit**

```bash
git add src/components/UpdateDialog.vue
git commit -m "feat: add UpdateDialog component for version update UI"
```

---

### Task 7: 前端 — 修改 App.vue 启动时自动检查更新

**Files:**
- Modify: `src/App.vue:1-27`

- [ ] **Step 1: 更新 `App.vue` script 部分**

将 `src/App.vue` 的 `<script setup lang="ts">` 内容替换为：

```vue
<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useToast } from './composables/useToast'
import { useSkillDialog } from './composables/useSkillDialog'
import { useSettings } from './composables/useSettings'
import { useFavorites } from './composables/useFavorites'
import { useRepos } from './composables/useRepos'
import { useUpdate } from './composables/useUpdate'
import { useI18n } from './i18n'
import IconRail from './components/IconRail.vue'
import SkillDialog from './components/SkillDialog.vue'
import UpdateDialog from './components/UpdateDialog.vue'
import Toast from './components/Toast.vue'

const { addToast } = useToast()
const { selectedSkill } = useSkillDialog()
const { loadSettings } = useSettings()
const { loadFavorites } = useFavorites()
const { error: reposError } = useRepos()
const { updateAvailable, checkForUpdate } = useUpdate()
const { t } = useI18n()

const showUpdateDialog = ref(false)

watch(reposError, (err) => {
  if (err) addToast(err, 'error')
})

watch(updateAvailable, (available) => {
  if (available) {
    showUpdateDialog.value = true
  }
})

onMounted(async () => {
  await Promise.all([loadSettings(), loadFavorites()])
  try {
    await checkForUpdate()
  } catch {
    // 自动检查失败时静默跳过
  }
})
</script>
```

- [ ] **Step 2: 更新 `App.vue` template 部分**

将 `src/App.vue` 的 `<template>` 内容替换为：

```html
<template>
  <div class="app-layout">
    <IconRail />
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>

  <SkillDialog
    v-if="selectedSkill"
    :skill="selectedSkill"
    @close="selectedSkill = null"
    @installed="addToast(t('app.installSuccess'), 'success')"
  />

  <UpdateDialog
    v-if="showUpdateDialog"
    @close="showUpdateDialog = false"
  />

  <Toast />
</template>
```

`<style scoped>` 部分保持不变。

- [ ] **Step 3: 验证类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误

- [ ] **Step 4: Commit**

```bash
git add src/App.vue
git commit -m "feat: add auto update check on app startup with UpdateDialog"
```

---

### Task 8: 前端 — 修改 SettingsDialog 添加手动检查更新

**Files:**
- Modify: `src/components/SettingsDialog.vue:1-118`

- [ ] **Step 1: 更新 `SettingsDialog.vue` script 部分**

将整个 `<script setup lang="ts">` 块替换为：

```vue
<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ToolType } from '../types';
import { TOOL_LABELS, getToolLabel } from '../utils/toolPaths';
import { useSettings } from '../composables/useSettings';
import { useUpdate } from '../composables/useUpdate';
import { useToast } from '../composables/useToast';
import { useI18n } from '../i18n';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits<{
  close: [];
}>();

const { defaultToolType, setDefaultToolType, clearProjectPaths } = useSettings();
const { addToast } = useToast();
const { updateAvailable, latestVersion, checking, checkForUpdate } = useUpdate();
const { locale, t } = useI18n();

const selected = ref<ToolType>(defaultToolType.value);
const projectPathCount = ref<number | null>(null);
const toolPathCount = ref<number | null>(null);
const clearing = ref(false)
const appVersion = ref('')
const checkUpdateText = ref('')

async function loadVersion() {
  try {
    appVersion.value = await invoke<string>('get_app_version')
  } catch {
    appVersion.value = ''
  }
}
loadVersion();

const tools = computed<{ value: ToolType; label: string }[]>(() =>
  (Object.entries(TOOL_LABELS) as [ToolType, string][]).map(([value]) => ({ value, label: getToolLabel(value, t('tool.custom')) }))
);

async function loadCounts() {
  try {
    const config = await invoke<{ projectPaths: string[]; toolPaths: Record<string, string> }>('load_config');
    projectPathCount.value = config.projectPaths.length;
    toolPathCount.value = Object.keys(config.toolPaths).length;
  } catch {
    projectPathCount.value = 0;
    toolPathCount.value = 0;
  }
}
loadCounts();

const totalCount = computed(() => (projectPathCount.value ?? 0) + (toolPathCount.value ?? 0));

async function handleClearAll() {
  clearing.value = true;
  try {
    await Promise.all([
      clearProjectPaths(),
      invoke('clear_tool_paths'),
    ]);
    projectPathCount.value = 0;
    toolPathCount.value = 0;
  } catch { /* ignore */ }
  clearing.value = false;
}

async function handleCheckUpdate() {
  checkUpdateText.value = ''
  try {
    await checkForUpdate()
    if (updateAvailable.value) {
      checkUpdateText.value = t('settings.updateAvailable') + ' v' + latestVersion.value
    } else {
      checkUpdateText.value = t('settings.upToDate')
      addToast(t('settings.upToDate'), 'success')
    }
  } catch {
    checkUpdateText.value = ''
    addToast(t('settings.updateError'), 'error')
  }
}

function handleSave() {
  setDefaultToolType(selected.value);
  emit('close');
}
</script>
```

- [ ] **Step 2: 更新版本区域模板**

将 `SettingsDialog.vue` 中版本区域（约第 107-110 行）替换为：

```html
      <div class="section version-section" v-if="appVersion">
        <label>{{ t('settings.version') }}</label>
        <div class="version-right">
          <span class="version-value">v{{ appVersion }}</span>
          <button
            class="check-update-btn"
            :disabled="checking"
            @click="handleCheckUpdate"
          >
            {{ checking ? t('settings.checking') : t('settings.checkUpdate') }}
          </button>
        </div>
      </div>

      <div v-if="checkUpdateText" class="section">
        <span class="update-status" :class="{ 'has-update': updateAvailable }">{{ checkUpdateText }}</span>
      </div>
```

- [ ] **Step 3: 更新样式**

在 `SettingsDialog.vue` 的 `<style scoped>` 末尾（`.version-value` 规则之后）替换 `.version-section` 和 `.version-value` 并添加新样式。将以下样式块替换：

原 `.version-section` 和 `.version-value`：
```css
.version-section {
  align-items: center;
  flex-direction: row !important;
  justify-content: space-between;
}
.version-value {
  color: var(--text-secondary);
  font-size: 0.85rem;
  font-variant-numeric: tabular-nums;
}
```

替换为：
```css
.version-section {
  align-items: center;
  flex-direction: row !important;
  justify-content: space-between;
}
.version-right {
  display: flex;
  align-items: center;
  gap: 10px;
}
.version-value {
  color: var(--text-secondary);
  font-size: 0.85rem;
  font-variant-numeric: tabular-nums;
}
.check-update-btn {
  padding: 4px 14px;
  font-size: 0.8rem;
  border-radius: 4px;
  border: 1px solid var(--border);
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  cursor: pointer;
  transition: background 0.15s;
}
.check-update-btn:hover:not(:disabled) {
  background: var(--bg-surface);
}
.check-update-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.update-status {
  font-size: 0.85rem;
  color: var(--text-secondary);
}
.update-status.has-update {
  color: var(--primary);
  font-weight: 500;
}
```

- [ ] **Step 4: 验证类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误

- [ ] **Step 5: Commit**

```bash
git add src/components/SettingsDialog.vue
git commit -m "feat: add manual update check button in settings dialog"
```

---

### Task 9: 集成验证

**Files:**
- All modified files

- [ ] **Step 1: 验证 Rust 编译**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: 编译成功

- [ ] **Step 2: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无错误

- [ ] **Step 3: 验证开发模式启动**

Run: `pnpm tauri dev`
Expected: 应用正常启动，无控制台报错。由于无实际 GitHub Release，更新检查会静默失败（符合预期）

- [ ] **Step 4: Commit（如有格式修复）**

```bash
git add -A
git commit -m "chore: fix lint issues after version update feature integration"
```
