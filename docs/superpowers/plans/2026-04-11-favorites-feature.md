# 收藏列表功能 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 在技能卡片上增加收藏按钮，新增收藏列表页面，支持在技能列表和收藏列表中添加/移除收藏。

**Architecture:** 收藏数据持久化在 Rust 后端的 `AppConfig` 中（`favorites: Vec<FavoriteEntry>`），通过 Tauri 命令进行增删查。前端新增 `useFavorites` composable 管理收藏状态。在 `IconRail` 中增加收藏导航按钮，点击后 `MainContent` 切换为收藏列表视图（复用 `SkillCard` 组件）。

**Tech Stack:** Rust (Tauri commands, serde), Vue 3 Composition API, TypeScript, lucide-vue-next icons

---

## File Structure

| Action | File | Responsibility |
|--------|------|----------------|
| Modify | `src-tauri/src/models/repo.rs` | 添加 `FavoriteEntry` 结构体，`AppConfig` 新增 `favorites` 字段 |
| Modify | `src-tauri/src/commands/config.rs` | 添加 `add_favorite`、`remove_favorite`、`list_favorites` 命令 |
| Modify | `src-tauri/src/lib.rs` | 注册新命令 |
| Create | `src/composables/useFavorites.ts` | 收藏状态管理 composable（模块级单例） |
| Modify | `src/types/index.ts` | 添加 `FavoriteEntry` 接口 |
| Modify | `src/components/SkillCard.vue` | 添加收藏星星按钮 |
| Modify | `src/components/MainContent.vue` | 支持收藏列表模式，从 props 切换视图 |
| Modify | `src/components/IconRail.vue` | 添加收藏导航按钮，emit `favorites` 事件 |
| Modify | `src/App.vue` | 管理收藏/仓库视图切换状态 |

---

### Task 1: Rust 后端 — 数据模型与收藏命令

**Files:**
- Modify: `src-tauri/src/models/repo.rs:36-58`
- Modify: `src-tauri/src/commands/config.rs:1-103`
- Modify: `src-tauri/src/lib.rs:12-40`

- [ ] **Step 1: 在 `repo.rs` 中添加 `FavoriteEntry` 和更新 `AppConfig`**

在 `src-tauri/src/models/repo.rs` 的 `Skill` 结构体之后（第 34 行后），添加 `FavoriteEntry` 结构体。然后更新 `AppConfig` 添加 `favorites` 字段。

```rust
// 在 Skill 结构体之后添加（第 34 行后）:

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteEntry {
    pub skill_id: String,
    pub repo_url: String,
}
```

更新 `AppConfig`（第 36-47 行），在 `default_tool_type` 之后添加 `favorites` 字段：

```rust
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
}
```

更新 `Default` impl（第 49-58 行）：

```rust
impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            repos: vec![],
            tool_paths: std::collections::HashMap::new(),
            project_paths: vec![],
            default_tool_type: Some("claude-code".to_string()),
            favorites: vec![],
        }
    }
}
```

- [ ] **Step 2: 在 `config.rs` 中添加收藏命令**

在 `src-tauri/src/commands/config.rs` 文件末尾（第 103 行之后）添加三个新命令。需要在顶部 import 中加入 `FavoriteEntry`：

```rust
use crate::models::{AppConfig, FavoriteEntry};
```

在文件末尾添加：

```rust
#[tauri::command]
pub async fn list_favorites() -> Result<Vec<FavoriteEntry>, String> {
    let config = load_config_from_disk()?;
    Ok(config.favorites)
}

#[tauri::command]
pub async fn add_favorite(skill_id: String, repo_url: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;
    let entry = FavoriteEntry {
        skill_id,
        repo_url,
    };
    if config.favorites.iter().any(|f| f.skill_id == entry.skill_id && f.repo_url == entry.repo_url) {
        return Ok(());
    }
    config.favorites.push(entry);
    save_config_to_disk(&config)
}

#[tauri::command]
pub async fn remove_favorite(skill_id: String, repo_url: String) -> Result<(), String> {
    let mut config = load_config_from_disk()?;
    config.favorites.retain(|f| !(f.skill_id == skill_id && f.repo_url == repo_url));
    save_config_to_disk(&config)
}
```

- [ ] **Step 3: 在 `lib.rs` 中注册新命令**

在 `src-tauri/src/lib.rs` 的 `tauri::generate_handler![]` 中，在 `// 安装` 注释之后（第 39 行 `install::check_junction_exists,` 之后）添加：

```rust
            // 收藏
            config::list_favorites,
            config::add_favorite,
            config::remove_favorite,
```

- [ ] **Step 4: 验证 Rust 编译**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: 编译成功，无错误

- [ ] **Step 5: 提交**

```bash
git add src-tauri/src/models/repo.rs src-tauri/src/commands/config.rs src-tauri/src/lib.rs
git commit -m "feat: add favorites data model and Tauri commands"
```

---

### Task 2: 前端类型定义与 useFavorites composable

**Files:**
- Modify: `src/types/index.ts:41-45`
- Create: `src/composables/useFavorites.ts`

- [ ] **Step 1: 在 `types/index.ts` 中添加 `FavoriteEntry` 接口**

在 `AppConfig` 接口之前（第 41 行之前）添加：

```typescript
export interface FavoriteEntry {
  skillId: string;
  repoUrl: string;
}
```

同时更新 `AppConfig` 接口（第 41-45 行），添加 `favorites` 字段：

```typescript
export interface AppConfig {
  repos: Omit<Repo, 'skills'>[];
  toolPaths: Partial<Record<ToolType, string>>;
  defaultToolType?: ToolType | null;
  favorites: FavoriteEntry[];
}
```

- [ ] **Step 2: 创建 `useFavorites.ts` composable**

创建 `src/composables/useFavorites.ts`：

```typescript
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Skill, FavoriteEntry } from '../types';

const favorites = ref<FavoriteEntry[]>([]);
let loaded = false;

async function loadFavorites() {
  if (loaded) return;
  loaded = true;
  try {
    favorites.value = await invoke<FavoriteEntry[]>('list_favorites');
  } catch {
    favorites.value = [];
  }
}

function isFavorite(skillId: string, repoUrl: string): boolean {
  return favorites.value.some(f => f.skillId === skillId && f.repoUrl === repoUrl);
}

async function addFavorite(skillId: string, repoUrl: string) {
  await invoke('add_favorite', { skillId, repoUrl });
  if (!isFavorite(skillId, repoUrl)) {
    favorites.value.push({ skillId, repoUrl });
  }
}

async function removeFavorite(skillId: string, repoUrl: string) {
  await invoke('remove_favorite', { skillId, repoUrl });
  favorites.value = favorites.value.filter(
    f => !(f.skillId === skillId && f.repoUrl === repoUrl)
  );
}

async function toggleFavorite(skillId: string, repoUrl: string) {
  if (isFavorite(skillId, repoUrl)) {
    await removeFavorite(skillId, repoUrl);
  } else {
    await addFavorite(skillId, repoUrl);
  }
}

export function useFavorites() {
  return {
    favorites,
    loadFavorites,
    isFavorite,
    addFavorite,
    removeFavorite,
    toggleFavorite,
  };
}
```

- [ ] **Step 3: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 4: 提交**

```bash
git add src/types/index.ts src/composables/useFavorites.ts
git commit -m "feat: add FavoriteEntry type and useFavorites composable"
```

---

### Task 3: SkillCard 添加收藏按钮

**Files:**
- Modify: `src/components/SkillCard.vue:1-71` (script + template)
- Modify: `src/components/SkillCard.vue:126-320` (style)

- [ ] **Step 1: 更新 SkillCard 的 props、emits 和逻辑**

更新 `src/components/SkillCard.vue` 的 `<script setup>` 部分。在现有 props 中添加 `isFavorite` prop，在 emits 中添加 `toggleFavorite` 事件：

将 props（第 13-17 行）替换为：

```typescript
const props = defineProps<{
  skill: Skill;
  quickInstallEntries?: QuickInstallEntry[];
  openDropdown?: boolean;
  isFavorite?: boolean;
}>();
```

将 emits（第 19-24 行）替换为：

```typescript
const emit = defineEmits<{
  install: [skill: Skill];
  quickInstall: [skill: Skill, entry: QuickInstallEntry];
  removeQuickInstallEntry: [entry: QuickInstallEntry];
  toggleDropdown: [];
  toggleFavorite: [skill: Skill];
}>();
```

在 `handleRemoveEntry` 函数之后（第 60 行后）添加收藏切换函数：

```typescript
function handleToggleFavorite(e: Event) {
  e.stopPropagation();
  emit('toggleFavorite', props.skill);
}
```

- [ ] **Step 2: 在模板中添加收藏按钮**

在 `<div class="skill-name">` 行之前（第 75 行之前），在 `.skill-card` div 内部开头添加收藏按钮：

```html
    <button class="favorite-btn" :class="{ active: isFavorite }" @click="handleToggleFavorite" :title="isFavorite ? '取消收藏' : '收藏'">
      <svg width="16" height="16" viewBox="0 0 24 24" :fill="isFavorite ? 'currentColor' : 'none'" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"/></svg>
    </button>
```

- [ ] **Step 3: 添加收藏按钮样式**

在 `<style scoped>` 块中（第 320 行 `</style>` 之前），在 `.skill-card` 的样式块后（第 148 行之后）添加：

```css
.favorite-btn {
  position: absolute;
  top: 10px;
  right: 10px;
  background: none;
  border: none;
  cursor: pointer;
  color: var(--text-muted);
  padding: 4px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  transition: color 0.15s, background 0.15s;
  opacity: 0;
}
.skill-card:hover .favorite-btn,
.favorite-btn.active {
  opacity: 1;
}
.favorite-btn:hover {
  color: var(--primary);
  background: var(--primary-light);
}
.favorite-btn.active {
  color: var(--primary);
}
```

同时更新 `.skill-card` 的 CSS，使其 `position: relative`（第 127-139 行），将原来的样式替换为：

```css
.skill-card {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 14px;
  cursor: pointer;
  background: var(--card-bg);
  backdrop-filter: blur(8px);
  display: flex;
  flex-direction: column;
  gap: 6px;
  transition: border-color 0.2s, box-shadow 0.2s, background 0.2s;
  box-shadow: var(--card-shadow);
  position: relative;
}
```

- [ ] **Step 4: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 5: 提交**

```bash
git add src/components/SkillCard.vue
git commit -m "feat: add favorite star button to SkillCard"
```

---

### Task 4: IconRail 添加收藏导航按钮

**Files:**
- Modify: `src/components/IconRail.vue`

- [ ] **Step 1: 更新 emits 和添加 props**

更新 `src/components/IconRail.vue` 的 script 部分。添加 `activeView` prop 和 `favorites` emit：

```typescript
const props = defineProps<{
  activeView: 'repos' | 'favorites';
}>();

const emit = defineEmits<{
  settings: [];
  favorites: [];
  repos: [];
}>();
```

注意：需要同时添加 `lucide-vue-next` 的 `Star` 图标导入：

```typescript
import { Sun, Moon, Settings, Star } from 'lucide-vue-next';
```

- [ ] **Step 2: 在模板中添加收藏按钮**

在 `.rail-top` 的 `logo` div 之后（第 31 行 `</div>` 后），添加收藏按钮：

```html
      <div class="nav-divider"></div>
      <button
        class="nav-btn"
        :class="{ active: props.activeView === 'repos' }"
        title="技能列表"
        @click="emit('repos')"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
      </button>
      <button
        class="nav-btn"
        :class="{ active: props.activeView === 'favorites' }"
        title="收藏列表"
        @click="emit('favorites')"
      >
        <Star :size="18" />
      </button>
```

- [ ] **Step 3: 添加导航按钮样式**

在 `<style scoped>` 中，`.logo` 样式之后（第 77 行后）添加：

```css
.nav-divider {
  width: 20px;
  height: 1px;
  background: var(--border);
  margin: 4px 0;
}
.nav-btn {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s, color 0.15s;
}
.nav-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.nav-btn.active {
  background: var(--primary-light);
  color: var(--primary);
}
```

- [ ] **Step 4: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 5: 提交**

```bash
git add src/components/IconRail.vue
git commit -m "feat: add navigation buttons to IconRail"
```

---

### Task 5: MainContent 支持收藏列表视图

**Files:**
- Modify: `src/components/MainContent.vue`

- [ ] **Step 1: 更新 MainContent props 和 emits**

更新 `src/components/MainContent.vue` 的 script。修改 props 以支持视图模式：

将 props（第 13-15 行）替换为：

```typescript
const props = defineProps<{
  repoUrl: string | null;
  viewMode: 'repos' | 'favorites';
}>();
```

更新 emits（第 17-20 行）：

```typescript
const emit = defineEmits<{
  installSkill: [skill: Skill];
  quickInstallSkill: [skill: Skill, entry: QuickInstallEntry];
  toggleFavorite: [skill: Skill];
}>();
```

添加 imports（第 1 行附近）：

```typescript
import { useFavorites } from '../composables/useFavorites';
```

在 `useSettings()` 解构之后（第 24 行后），添加：

```typescript
const { favorites, isFavorite, toggleFavorite } = useFavorites();
```

- [ ] **Step 2: 添加收藏列表的计算属性和方法**

在 `filteredSkills` computed 之后（第 96 行后），添加收藏技能的计算属性和收藏过滤：

```typescript
const favoriteSkills = computed<Skill[]>(() => {
  const result: Skill[] = [];
  for (const fav of favorites.value) {
    const repoSkills = skillsByRepo.value[fav.repoUrl];
    if (repoSkills) {
      const skill = repoSkills.find(s => s.id === fav.skillId);
      if (skill) result.push(skill);
    }
  }
  return result;
});

const filteredFavoriteSkills = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return favoriteSkills.value;
  return favoriteSkills.value.filter(
    s => s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q),
  );
});

async function handleToggleFavorite(skill: Skill) {
  try {
    await toggleFavorite(skill.id, skill.repoUrl);
  } catch (e) {
    // ignore
  }
}
```

需要在文件顶部导入 `skillsByRepo`（从 `useSkills` 中额外解构）：

将第 23 行：
```typescript
const { loadSkills } = useSkills();
```
替换为：
```typescript
const { skillsByRepo, loadSkills } = useSkills();
```

- [ ] **Step 3: 添加收藏列表视图加载逻辑**

添加 `onMounted` 中加载收藏的逻辑。在现有 `onMounted` 回调中（第 66 行附近），在 `await loadProjectPaths();` 之后添加：

```typescript
await loadFavorites();
```

需要确保 `loadFavorites` 已从 `useFavorites()` 解构（已在 Step 1 中完成，需确认解构中包含 `loadFavorites`）。

- [ ] **Step 4: 更新模板支持双视图**

将整个 `<template>` 替换为：

```html
<template>
  <div class="main-content">
    <!-- No repo selected (repos view) -->
    <div v-if="props.viewMode === 'repos' && !repoUrl" class="empty-state">
      <Inbox :size="48" class="empty-icon" />
      <p>选择一个仓库查看技能</p>
    </div>

    <!-- Loading (repos view) -->
    <div v-else-if="props.viewMode === 'repos' && loading" class="loading-state">
      <Loader2 :size="24" class="spin" />
      <span>加载技能中...</span>
    </div>

    <!-- Favorites view -->
    <template v-else-if="props.viewMode === 'favorites'">
      <div class="sticky-header">
        <div class="search-bar">
          <Search :size="14" class="search-icon" />
          <input
            v-if="favoriteSkills.length > 1"
            v-model="searchQuery"
            type="text"
            class="search-input"
            placeholder="搜索收藏技能..."
          />
          <span v-else class="search-input placeholder-text">搜索收藏技能...</span>
          <span class="fav-count">{{ favoriteSkills.length }} 个收藏</span>
        </div>
      </div>

      <div v-if="favoriteSkills.length === 0" class="empty-state">
        <StarOff :size="48" class="empty-icon" />
        <p>暂无收藏技能</p>
      </div>

      <template v-else>
        <div v-if="filteredFavoriteSkills.length === 0" class="empty-state">
          <Inbox :size="48" class="empty-icon" />
          <p>未找到匹配的收藏技能</p>
        </div>

        <div v-else class="skills-grid">
          <SkillCard
            v-for="skill in filteredFavoriteSkills"
            :key="skill.id"
            :skill="skill"
            :is-favorite="true"
            :quick-install-entries="quickInstallEntries"
            :open-dropdown="openDropdownId === skill.id"
            @install="emit('installSkill', $event)"
            @quick-install="(skill, entry) => emit('quickInstallSkill', skill, entry)"
            @remove-quick-install-entry="removeQuickInstallEntry"
            @toggle-favorite="handleToggleFavorite"
            @toggle-dropdown="openDropdownId = openDropdownId === skill.id ? null : skill.id"
          />
        </div>
      </template>
    </template>

    <!-- Repo content -->
    <template v-else-if="currentRepo">
      <div class="sticky-header">
        <div class="search-bar">
          <Search :size="14" class="search-icon" />
          <input
            v-if="skills.length > 1"
            v-model="searchQuery"
            type="text"
            class="search-input"
            placeholder="搜索技能..."
          />
          <span v-else class="search-input placeholder-text">搜索技能...</span>
          <div class="header-divider"></div>
          <button class="url-chip" @click="copyRepoUrl" :title="copied ? '已复制' : '点击复制地址'">
            <span class="url-text">{{ isLocalRepo ? currentRepo!.localPath : currentRepo!.url }}</span>
            <component :is="copied ? Check : Copy" :size="11" class="copy-icon" :class="{ copied }" />
          </button>
        </div>
      </div>

      <div v-if="skills.length === 0" class="empty-state">
        <Inbox :size="48" class="empty-icon" />
        <p>未找到技能</p>
      </div>

      <template v-else>
        <div v-if="filteredSkills.length === 0" class="empty-state">
          <Inbox :size="48" class="empty-icon" />
          <p>未找到匹配的技能</p>
        </div>

        <div v-else class="skills-grid">
          <SkillCard
            v-for="skill in filteredSkills"
            :key="skill.id"
            :skill="skill"
            :is-favorite="isFavorite(skill.id, skill.repoUrl)"
            :quick-install-entries="quickInstallEntries"
            :open-dropdown="openDropdownId === skill.id"
            @install="emit('installSkill', $event)"
            @quick-install="(skill, entry) => emit('quickInstallSkill', skill, entry)"
            @remove-quick-install-entry="removeQuickInstallEntry"
            @toggle-favorite="handleToggleFavorite"
            @toggle-dropdown="openDropdownId = openDropdownId === skill.id ? null : skill.id"
          />
        </div>
      </template>
    </template>
  </div>
</template>
```

添加 `StarOff` 图标导入（在 lucide 导入行中追加）：

```typescript
import { Loader2, Inbox, Search, Copy, Check, StarOff } from 'lucide-vue-next';
```

添加收藏计数样式（在 `.loading-state` 之后）：

```css
.fav-count {
  font-size: 0.75rem;
  color: var(--text-muted);
  white-space: nowrap;
  flex-shrink: 0;
}
```

- [ ] **Step 5: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 6: 提交**

```bash
git add src/components/MainContent.vue
git commit -m "feat: add favorites list view to MainContent"
```

---

### Task 6: App.vue 集成视图切换与收藏状态

**Files:**
- Modify: `src/App.vue`

- [ ] **Step 1: 更新 App.vue 的状态和 imports**

在 `src/App.vue` 的 script 中，添加 `useFavorites` 导入（第 7 行附近）：

```typescript
import { useFavorites } from './composables/useFavorites';
```

在 `useInstall` 解构之后（第 34 行后）添加：

```typescript
const { loadFavorites } = useFavorites();

const activeView = ref<'repos' | 'favorites'>('repos');
```

- [ ] **Step 2: 在 `onMounted` 中加载收藏**

在 `onMounted` 回调中（第 47 行附近），在 `await Promise.all([loadRepos(), loadSettings()]);` 中添加 `loadFavorites()`：

```typescript
await Promise.all([loadRepos(), loadSettings(), loadFavorites()]);
```

- [ ] **Step 3: 更新模板**

将模板中的 `IconRail` 标签（第 137-138 行）替换为：

```html
    <IconRail :active-view="activeView" @settings="showSettings = true" @favorites="activeView = 'favorites'" @repos="activeView = 'repos'; selectedRepoUrl = repos.length > 0 ? repos[0].url : null" />
```

将 `MainContent` 标签（第 146-150 行）替换为：

```html
    <MainContent
      :repo-url="selectedRepoUrl"
      :view-mode="activeView"
      @install-skill="selectedSkill = $event"
      @quick-install-skill="handleQuickInstall"
    />
```

- [ ] **Step 4: 验证前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 5: 提交**

```bash
git add src/App.vue
git commit -m "feat: integrate favorites view switching in App"
```

---

### Task 7: 端到端验证

**Files:** 无新增/修改

- [ ] **Step 1: 运行完整 Rust 检查**

Run: `cargo check --manifest-path src-tauri/Cargo.toml`
Expected: 编译成功

- [ ] **Step 2: 运行完整前端类型检查**

Run: `vue-tsc --noEmit`
Expected: 无类型错误

- [ ] **Step 3: 运行 Rust 测试**

Run: `cargo test --manifest-path src-tauri/Cargo.toml`
Expected: 所有测试通过

- [ ] **Step 4: 启动开发模式手动验证**

Run: `pnpm tauri dev`

验证以下功能：
1. 技能列表中卡片右上角出现收藏星星按钮（hover 时显示）
2. 点击星星按钮收藏技能，按钮变为实心高亮
3. 再次点击取消收藏
4. 点击 IconRail 中的收藏按钮切换到收藏列表视图
5. 收藏列表中显示已收藏的技能卡片
6. 收藏列表中可以取消收藏
7. 重启应用后收藏数据持久化
