# Layout Redesign & Dark Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Redesign the Skills Manager app with a three-panel IDE-style layout and a three-mode theme system (system/light/dark).

**Architecture:** Replace single-column layout with Icon Rail (56px) + Repo Panel (220px) + Main Content (flex). Theme system uses CSS custom properties toggled via class on `<html>`, replacing all `@media (prefers-color-scheme: dark)` blocks. Toast notification system replaces inline status messages.

**Tech Stack:** Vue 3 + TypeScript, Tauri v2, Lucide icons (`lucide-vue-next`)

**Spec:** `docs/superpowers/specs/2026-04-10-layout-redesign-dark-mode-design.md`

**Verification commands:**
- Type check: `pnpm build` (runs `vue-tsc --noEmit` then Vite build)
- Visual: `pnpm tauri dev` (starts dev server + Tauri window)

---

## File Structure

| Action | File | Responsibility |
|--------|------|----------------|
| Create | `src/assets/theme.css` | Global CSS variable definitions (light + dark) |
| Create | `src/composables/useTheme.ts` | Three-mode theme state (system/light/dark) |
| Create | `src/composables/useToast.ts` | Toast notification state management |
| Create | `src/components/IconRail.vue` | 56px icon column: logo + theme toggle |
| Create | `src/components/RepoPanel.vue` | 220px sidebar: repo list + add button |
| Create | `src/components/MainContent.vue` | Main area: repo header + skill card grid |
| Create | `src/components/Toast.vue` | Toast notification renderer (fixed bottom-right) |
| Modify | `src-tauri/tauri.conf.json` | Window size 1100x750, min 800x500 |
| Modify | `src/main.ts` | Import theme.css |
| Modify | `src/composables/useSkills.ts` | Module-scoped singleton with cache + clearCache |
| Modify | `src/composables/useRepos.ts` | Per-operation loading states |
| Modify | `src/components/SkillCard.vue` | CSS variables, Lucide icons |
| Modify | `src/components/RepoManager.vue` | CSS variables, Escape key, remove media queries |
| Modify | `src/components/SkillDialog.vue` | CSS variables, Escape key, remove media queries |
| Modify | `src/App.vue` | Three-column layout, wire all new components |
| Remove | `src/components/RepoGroup.vue` | Split into RepoPanel + MainContent |

---

### Task 1: Install Dependencies

**Files:**
- Modify: `package.json`

- [ ] **Step 1: Install lucide-vue-next**

```bash
cd d:/code/dev/app/skills-manager && pnpm add lucide-vue-next
```

- [ ] **Step 2: Verify installation**

```bash
pnpm build
```

Expected: Build succeeds (no type errors from new dependency).

- [ ] **Step 3: Commit**

```bash
git add package.json pnpm-lock.yaml
git commit -m "chore: add lucide-vue-next icon dependency"
```

---

### Task 2: Theme CSS Foundation

**Files:**
- Create: `src/assets/theme.css`
- Modify: `src/main.ts`

- [ ] **Step 1: Create theme.css with all CSS variables**

```css
/* src/assets/theme.css */
:root,
.theme-light {
  --bg-app: #f8fafc;
  --bg-surface: #ffffff;
  --bg-surface-sunken: #f1f5f9;
  --bg-surface-hover: #f1f5f9;
  --bg-scrim: rgba(0, 0, 0, 0.5);
  --border: #e2e8f0;
  --text-primary: #1e293b;
  --text-secondary: #64748b;
  --text-muted: #94a3b8;
  --primary: #2563eb;
  --primary-hover: #1d4ed8;
  --primary-light: #dbeafe;
  --danger: #dc2626;
  --danger-hover: #b91c1c;
  --danger-light: #fef2f2;
  --success: #16a34a;
  --success-light: #f0fdf4;
}

.theme-dark {
  --bg-app: #0f172a;
  --bg-surface: #1e293b;
  --bg-surface-sunken: #020617;
  --bg-surface-hover: #334155;
  --bg-scrim: rgba(0, 0, 0, 0.7);
  --border: #334155;
  --text-primary: #e2e8f0;
  --text-secondary: #94a3b8;
  --text-muted: #64748b;
  --primary: #3b82f6;
  --primary-hover: #2563eb;
  --primary-light: #172554;
  --danger: #ef4444;
  --danger-hover: #dc2626;
  --danger-light: #450a0a;
  --success: #22c55e;
  --success-light: #052e16;
}

/* Global resets */
*,
*::before,
*::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  background: var(--bg-app);
  color: var(--text-primary);
}

/* Focus styles */
:focus-visible {
  outline: 2px solid var(--primary);
  outline-offset: 2px;
}

/* Custom scrollbar (WebKit - Tauri WebView) */
::-webkit-scrollbar {
  width: 6px;
}
::-webkit-scrollbar-track {
  background: var(--bg-app);
}
::-webkit-scrollbar-thumb {
  background: var(--border);
  border-radius: 3px;
}
::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}
```

- [ ] **Step 2: Update main.ts to import theme.css**

Replace the entire content of `src/main.ts` with:

```typescript
import { createApp } from "vue";
import "./assets/theme.css";
import App from "./App.vue";

createApp(App).mount("#app");
```

- [ ] **Step 3: Remove global styles from App.vue**

In `src/App.vue`, delete this global `<style>` block (lines 99-102):

```css
<style>
* { box-sizing: border-box; }
body { margin: 0; font-family: Inter, Avenir, Helvetica, Arial, sans-serif; }
</style>
```

These are now handled by `theme.css`.

- [ ] **Step 4: Verify build**

```bash
pnpm build
```

Expected: Build succeeds.

- [ ] **Step 5: Commit**

```bash
git add src/assets/theme.css src/main.ts src/App.vue
git commit -m "feat: add global theme CSS variables and base styles"
```

---

### Task 3: useTheme Composable

**Files:**
- Create: `src/composables/useTheme.ts`

- [ ] **Step 1: Create useTheme.ts**

```typescript
// src/composables/useTheme.ts
import { ref, computed, watch } from 'vue';

export type ThemeMode = 'system' | 'light' | 'dark';

const STORAGE_KEY = 'skills-manager-theme';

const mode = ref<ThemeMode>(
  (localStorage.getItem(STORAGE_KEY) as ThemeMode) || 'system'
);

function getSystemTheme(): 'light' | 'dark' {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: 'light' | 'dark') {
  const html = document.documentElement;
  html.classList.remove('theme-light', 'theme-dark');
  html.classList.add(`theme-${theme}`);
}

let mediaQuery: MediaQueryList | null = null;
let mediaHandler: ((e: MediaQueryListEvent) => void) | null = null;

function startSystemListener() {
  if (mediaQuery) return;
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaHandler = () => {
    if (mode.value === 'system') {
      applyTheme(getSystemTheme());
    }
  };
  mediaQuery.addEventListener('change', mediaHandler);
}

function stopSystemListener() {
  if (mediaQuery && mediaHandler) {
    mediaQuery.removeEventListener('change', mediaHandler);
    mediaQuery = null;
    mediaHandler = null;
  }
}

function resolveTheme(): 'light' | 'dark' {
  return mode.value === 'system' ? getSystemTheme() : mode.value;
}

watch(mode, (newMode) => {
  localStorage.setItem(STORAGE_KEY, newMode);
  applyTheme(resolveTheme());
  if (newMode === 'system') {
    startSystemListener();
  } else {
    stopSystemListener();
  }
});

// Initialize on first import
if (mode.value === 'system') {
  startSystemListener();
}

const resolvedTheme = computed(() => resolveTheme());

// Apply theme immediately on module load
applyTheme(resolveTheme());

export function useTheme() {
  function cycleTheme() {
    const order: ThemeMode[] = ['system', 'light', 'dark'];
    const idx = order.indexOf(mode.value);
    mode.value = order[(idx + 1) % order.length];
  }

  return {
    mode,
    resolvedTheme,
    cycleTheme,
  };
}
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/composables/useTheme.ts
git commit -m "feat: add useTheme composable with system/light/dark modes"
```

---

### Task 4: useToast Composable + Toast Component

**Files:**
- Create: `src/composables/useToast.ts`
- Create: `src/components/Toast.vue`

- [ ] **Step 1: Create useToast.ts**

```typescript
// src/composables/useToast.ts
import { ref } from 'vue';

export interface ToastItem {
  id: number;
  message: string;
  variant: 'success' | 'error' | 'info';
}

const toasts = ref<ToastItem[]>([]);
let nextId = 0;

function removeToast(id: number) {
  toasts.value = toasts.value.filter(t => t.id !== id);
}

export function useToast() {
  function addToast(message: string, variant: ToastItem['variant'] = 'info') {
    const id = nextId++;
    toasts.value.push({ id, message, variant });
    // Max 3: remove oldest if exceeded
    if (toasts.value.length > 3) {
      toasts.value.shift();
    }
    // Auto-dismiss after 4 seconds
    setTimeout(() => removeToast(id), 4000);
  }

  return {
    toasts,
    addToast,
    removeToast,
  };
}
```

- [ ] **Step 2: Create Toast.vue**

```vue
<!-- src/components/Toast.vue -->
<script setup lang="ts">
import { useToast } from '../composables/useToast';
import { CheckCircle, AlertCircle, Info, X } from 'lucide-vue-next';

const { toasts, removeToast } = useToast();

const iconMap = {
  success: CheckCircle,
  error: AlertCircle,
  info: Info,
};

const variantClass = {
  success: 'toast-success',
  error: 'toast-error',
  info: 'toast-info',
};
</script>

<template>
  <div class="toast-container" v-if="toasts.length">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        :class="['toast', variantClass[toast.variant]]"
      >
        <component :is="iconMap[toast.variant]" :size="16" class="toast-icon" />
        <span class="toast-message">{{ toast.message }}</span>
        <button class="toast-close" @click="removeToast(toast.id)">
          <X :size="14" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 16px;
  right: 16px;
  z-index: 200;
  display: flex;
  flex-direction: column;
  gap: 8px;
  pointer-events: none;
}
.toast {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  border-radius: 8px;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  font-size: 0.85rem;
  color: var(--text-primary);
  pointer-events: auto;
  min-width: 240px;
  max-width: 400px;
}
.toast-icon { flex-shrink: 0; }
.toast-message { flex: 1; }
.toast-close {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  display: flex;
  align-items: center;
}
.toast-close:hover { color: var(--text-primary); }
.toast-success .toast-icon { color: var(--success); }
.toast-error .toast-icon { color: var(--danger); }
.toast-info .toast-icon { color: var(--primary); }

/* Transition animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
</style>
```

- [ ] **Step 3: Verify build**

```bash
pnpm build
```

- [ ] **Step 4: Commit**

```bash
git add src/composables/useToast.ts src/components/Toast.vue
git commit -m "feat: add toast notification system with composable and component"
```

---

### Task 5: Refactor useSkills to Module-Scoped Singleton

**Files:**
- Modify: `src/composables/useSkills.ts`

- [ ] **Step 1: Rewrite useSkills.ts**

Replace entire content with:

```typescript
// src/composables/useSkills.ts
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Skill } from '../types';

// Module-scoped state (singleton)
const skillsByRepo = ref<Record<string, Skill[]>>({});
const loadingByRepo = ref<Record<string, boolean>>({});

// Helper to trigger Vue reactivity when deleting keys from Record
function deleteSkillEntry(repoUrl: string) {
  const copy = { ...skillsByRepo.value };
  delete copy[repoUrl];
  skillsByRepo.value = copy;
}

export function useSkills() {
  async function loadSkills(repoUrl: string, forceRefresh = false): Promise<Skill[]> {
    // Return cached if available and not forcing refresh
    if (!forceRefresh && skillsByRepo.value[repoUrl]?.length) {
      return skillsByRepo.value[repoUrl];
    }

    loadingByRepo.value[repoUrl] = true;
    try {
      const skills = await invoke<Skill[]>('list_skills', { repoUrl });
      skillsByRepo.value[repoUrl] = skills;
      return skills;
    } finally {
      loadingByRepo.value[repoUrl] = false;
    }
  }

  function isLoading(repoUrl: string): boolean {
    return loadingByRepo.value[repoUrl] ?? false;
  }

  function getSkills(repoUrl: string): Skill[] {
    return skillsByRepo.value[repoUrl] ?? [];
  }

  function clearCache(repoUrl?: string) {
    if (repoUrl) {
      deleteSkillEntry(repoUrl);
    } else {
      skillsByRepo.value = {};
    }
  }

  return {
    skillsByRepo,
    loadingByRepo,
    loadSkills,
    isLoading,
    getSkills,
    clearCache,
  };
}
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/composables/useSkills.ts
git commit -m "refactor: useSkills to module-scoped singleton with persistent cache"
```

---

### Task 6: Refactor useRepos with Per-Operation Loading

**Files:**
- Modify: `src/composables/useRepos.ts`

- [ ] **Step 1: Rewrite useRepos.ts**

Replace entire content with:

```typescript
// src/composables/useRepos.ts
import { ref, computed, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Repo } from '../types';

const repos = ref<Repo[]>([]);
const reposLoading = ref(false);
const addRepoLoading = ref(false);
const updateLoading = ref<Record<string, boolean>>({});
const updateAllLoading = ref(false);
const removeRepoLoading = ref(false);
const error = ref<string | null>(null);

const reposReadonly = computed(() => repos.value);

export function useRepos() {
  async function loadRepos() {
    reposLoading.value = true;
    error.value = null;
    try {
      const result = await invoke<Repo[]>('list_repos');
      repos.value = result.map(r => ({ ...r, skills: [] }));
    } catch (e) {
      error.value = String(e);
    } finally {
      reposLoading.value = false;
    }
  }

  async function addRepo(url: string): Promise<void> {
    addRepoLoading.value = true;
    error.value = null;
    try {
      const repo = await invoke<Repo>('add_repo', { url });
      repos.value.push({ ...repo, skills: [] });
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      addRepoLoading.value = false;
    }
  }

  async function removeRepo(url: string): Promise<void> {
    removeRepoLoading.value = true;
    error.value = null;
    try {
      await invoke('remove_repo', { url });
      repos.value = repos.value.filter(r => r.url !== url);
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      removeRepoLoading.value = false;
    }
  }

  async function updateRepo(url: string): Promise<string> {
    updateLoading.value[url] = true;
    error.value = null;
    try {
      const result = await invoke<string>('update_repo', { url });
      return result;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      updateLoading.value[url] = false;
    }
  }

  async function updateAllRepos(): Promise<string[]> {
    updateAllLoading.value = true;
    error.value = null;
    try {
      const results = await invoke<string[]>('update_all_repos');
      return results;
    } catch (e) {
      error.value = String(e);
      throw e;
    } finally {
      updateAllLoading.value = false;
    }
  }

  function isUpdateLoading(url: string): boolean {
    return updateLoading.value[url] ?? false;
  }

  return {
    repos: reposReadonly,
    reposLoading: readonly(reposLoading),
    addRepoLoading: readonly(addRepoLoading),
    updateLoading: readonly(updateLoading),
    updateAllLoading: readonly(updateAllLoading),
    removeRepoLoading: readonly(removeRepoLoading),
    error: readonly(error),
    loadRepos,
    addRepo,
    removeRepo,
    updateRepo,
    updateAllRepos,
    isUpdateLoading,
  };
}
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/composables/useRepos.ts
git commit -m "refactor: useRepos with per-operation loading states"
```

---

### Task 7: Window Configuration

**Files:**
- Modify: `src-tauri/tauri.conf.json`

- [ ] **Step 1: Update window config**

In `src-tauri/tauri.conf.json`, replace the `windows` array:

```json
"windows": [
  {
    "title": "Skills Manager",
    "width": 1100,
    "height": 750,
    "minWidth": 800,
    "minHeight": 500
  }
]
```

- [ ] **Step 2: Commit**

```bash
git add src-tauri/tauri.conf.json
git commit -m "chore: update window size for three-panel layout"
```

---

### Task 8: IconRail Component

**Files:**
- Create: `src/components/IconRail.vue`

- [ ] **Step 1: Create IconRail.vue**

```vue
<!-- src/components/IconRail.vue -->
<script setup lang="ts">
import { useTheme } from '../composables/useTheme';
import { Sun, Moon, Monitor, Package } from 'lucide-vue-next';

const { mode, cycleTheme } = useTheme();

const themeIcon = {
  system: Monitor,
  light: Sun,
  dark: Moon,
};

const themeTitle = {
  system: '跟随系统',
  light: '亮色模式',
  dark: '暗色模式',
};
</script>

<template>
  <div class="icon-rail">
    <div class="rail-top">
      <div class="logo" title="Skills Manager">
        <Package :size="22" />
      </div>
    </div>
    <div class="rail-bottom">
      <button
        class="theme-btn"
        @click="cycleTheme"
        :title="themeTitle[mode]"
      >
        <component :is="themeIcon[mode]" :size="18" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.icon-rail {
  width: 56px;
  min-width: 56px;
  height: 100%;
  background: var(--bg-surface-sunken);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
}
.rail-top {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}
.logo {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}
.rail-bottom {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}
.theme-btn {
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
.theme-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
</style>
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/components/IconRail.vue
git commit -m "feat: add IconRail component with theme toggle"
```

---

### Task 9: RepoPanel Component

**Files:**
- Create: `src/components/RepoPanel.vue`

- [ ] **Step 1: Create RepoPanel.vue**

```vue
<!-- src/components/RepoPanel.vue -->
<script setup lang="ts">
import { computed } from 'vue';
import { useRepos } from '../composables/useRepos';
import { RefreshCw, Trash2, Plus, Loader2 } from 'lucide-vue-next';

defineProps<{
  selectedRepoUrl: string | null;
}>();

const emit = defineEmits<{
  select: [url: string];
  add: [];
  remove: [url: string];
  update: [url: string];
  updateAll: [];
}>();

const { repos, updateAllLoading, isUpdateLoading } = useRepos();

const repoList = computed(() => repos.value);
</script>

<template>
  <div class="repo-panel">
    <div class="panel-header">
      <span class="panel-title">仓库</span>
      <button
        class="icon-btn"
        @click="emit('updateAll')"
        :disabled="updateAllLoading || repoList.length === 0"
        title="全部更新"
      >
        <Loader2 v-if="updateAllLoading" :size="15" class="spin" />
        <RefreshCw v-else :size="15" />
      </button>
    </div>

    <div class="repo-list">
      <div
        v-for="repo in repoList"
        :key="repo.url"
        :class="['repo-item', { selected: repo.url === selectedRepoUrl }]"
        @click="emit('select', repo.url)"
      >
        <div class="repo-info">
          <div class="repo-name">{{ repo.name }}</div>
          <div class="repo-skill-count">{{ repo.lastUpdate ? '已同步' : '新仓库' }}</div>
        </div>
        <div class="repo-actions" @click.stop>
          <button
            class="icon-btn-sm"
            @click="emit('update', repo.url)"
            :disabled="isUpdateLoading(repo.url)"
            title="更新"
          >
            <Loader2 v-if="isUpdateLoading(repo.url)" :size="13" class="spin" />
            <RefreshCw v-else :size="13" />
          </button>
          <button
            class="icon-btn-sm danger"
            @click="emit('remove', repo.url)"
            title="删除"
          >
            <Trash2 :size="13" />
          </button>
        </div>
      </div>

      <div v-if="repoList.length === 0" class="empty">
        暂无仓库
      </div>
    </div>

    <div class="panel-footer">
      <button class="add-btn" @click="emit('add')">
        <Plus :size="15" />
        <span>添加仓库</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.repo-panel {
  width: 220px;
  min-width: 220px;
  height: 100%;
  background: var(--bg-surface);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
}
.panel-header {
  padding: 12px 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid var(--border);
}
.panel-title {
  font-size: 0.8rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.repo-list {
  flex: 1;
  overflow-y: auto;
  padding: 6px 8px;
}
.repo-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  border-left: 3px solid transparent;
  transition: background 0.15s, border-color 0.15s;
}
.repo-item:hover {
  background: var(--bg-surface-hover);
}
.repo-item.selected {
  background: var(--bg-surface-hover);
  border-left-color: var(--primary);
}
.repo-info {
  flex: 1;
  min-width: 0;
}
.repo-name {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.repo-skill-count {
  font-size: 0.7rem;
  color: var(--text-muted);
  margin-top: 2px;
}
.repo-actions {
  display: flex;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}
.repo-item:hover .repo-actions {
  opacity: 1;
}
.empty {
  text-align: center;
  color: var(--text-muted);
  font-size: 0.85rem;
  padding: 24px 12px;
}
.panel-footer {
  padding: 10px 12px;
  border-top: 1px solid var(--border);
}
.add-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 6px;
  padding: 7px 12px;
  border: 1px dashed var(--border);
  border-radius: 6px;
  background: transparent;
  color: var(--text-secondary);
  font-size: 0.8rem;
  cursor: pointer;
  transition: background 0.15s, color 0.15s, border-color 0.15s;
}
.add-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  border-color: var(--primary);
}

/* Small icon buttons */
.icon-btn {
  width: 28px;
  height: 28px;
  border-radius: 6px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s, color 0.15s;
}
.icon-btn:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.icon-btn:disabled { opacity: 0.5; cursor: not-allowed; }

.icon-btn-sm {
  width: 24px;
  height: 24px;
  border-radius: 4px;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s, color 0.15s;
}
.icon-btn-sm:hover { background: var(--bg-surface-hover); color: var(--text-primary); }
.icon-btn-sm.danger:hover { color: var(--danger); }
.icon-btn-sm:disabled { opacity: 0.5; cursor: not-allowed; }

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/components/RepoPanel.vue
git commit -m "feat: add RepoPanel sidebar component"
```

---

### Task 10: MainContent Component

**Files:**
- Create: `src/components/MainContent.vue`

- [ ] **Step 1: Create MainContent.vue**

```vue
<!-- src/components/MainContent.vue -->
<script setup lang="ts">
import { ref, watch } from 'vue';
import type { Repo, Skill } from '../types';
import { useSkills } from '../composables/useSkills';
import { useRepos } from '../composables/useRepos';
import SkillCard from './SkillCard.vue';
import { RefreshCw, Loader2, Inbox } from 'lucide-vue-next';

const props = defineProps<{
  repoUrl: string | null;
}>();

const emit = defineEmits<{
  installSkill: [skill: Skill];
  updateRepo: [url: string];
}>();

const { repos } = useRepos();
const { loadSkills, isLoading, getSkills } = useSkills();

const skills = ref<Skill[]>([]);
const loading = ref(false);

const currentRepo = ref<Repo | null>(null);

watch(
  () => props.repoUrl,
  async (url) => {
    if (!url) {
      skills.value = [];
      currentRepo.value = null;
      return;
    }
    currentRepo.value = repos.value.find(r => r.url === url) || null;
    loading.value = true;
    try {
      skills.value = await loadSkills(url);
    } finally {
      loading.value = false;
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="main-content">
    <!-- No repo selected -->
    <div v-if="!repoUrl" class="empty-state">
      <Inbox :size="48" class="empty-icon" />
      <p>选择一个仓库查看技能</p>
    </div>

    <!-- Loading -->
    <div v-else-if="loading" class="loading-state">
      <Loader2 :size="24" class="spin" />
      <span>加载技能中...</span>
    </div>

    <!-- Repo content -->
    <template v-else-if="currentRepo">
      <div class="content-header">
        <div class="header-info">
          <h2 class="repo-title">{{ currentRepo.name }}</h2>
          <span class="repo-url">{{ currentRepo.url }}</span>
        </div>
        <button
          class="update-btn"
          @click="emit('updateRepo', currentRepo!.url)"
          title="更新仓库"
        >
          <RefreshCw :size="14" />
          <span>更新</span>
        </button>
      </div>

      <div v-if="skills.length === 0" class="empty-state">
        <Inbox :size="48" class="empty-icon" />
        <p>未找到技能</p>
      </div>

      <div v-else class="skills-grid">
        <SkillCard
          v-for="skill in skills"
          :key="skill.id"
          :skill="skill"
          @install="emit('installSkill', $event)"
        />
      </div>
    </template>
  </div>
</template>

<style scoped>
.main-content {
  flex: 1;
  height: 100%;
  overflow-y: auto;
  background: var(--bg-app);
}
.content-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 16px 20px;
  border-bottom: 1px solid var(--border);
}
.header-info {
  min-width: 0;
}
.repo-title {
  margin: 0;
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--text-primary);
}
.repo-url {
  font-size: 0.75rem;
  color: var(--text-muted);
  display: block;
  margin-top: 2px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.update-btn {
  display: flex;
  align-items: center;
  gap: 5px;
  padding: 5px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  font-size: 0.8rem;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  flex-shrink: 0;
}
.update-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: 12px;
  padding: 20px;
}
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  height: 60%;
  color: var(--text-muted);
  gap: 12px;
}
.empty-state p {
  margin: 0;
  font-size: 0.9rem;
}
.empty-icon {
  opacity: 0.4;
}
.loading-state {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 60%;
  gap: 8px;
  color: var(--text-muted);
}

.spin {
  animation: spin 1s linear infinite;
}
@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}
</style>
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/components/MainContent.vue
git commit -m "feat: add MainContent component for skill card grid"
```

---

### Task 11: Update SkillCard

**Files:**
- Modify: `src/components/SkillCard.vue`

- [ ] **Step 1: Rewrite SkillCard.vue with CSS variables and Lucide icons**

Replace entire content with:

```vue
<!-- src/components/SkillCard.vue -->
<script setup lang="ts">
import type { Skill } from '../types';
import { Download } from 'lucide-vue-next';

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
    <div class="card-footer">
      <button class="install-btn" @click.stop="emit('install', props.skill)">
        <Download :size="14" />
        <span>安装</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.skill-card {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 14px;
  cursor: pointer;
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  gap: 6px;
  transition: border-color 0.15s, box-shadow 0.15s;
}
.skill-card:hover {
  border-color: var(--primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}
.skill-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-primary);
}
.skill-description {
  font-size: 0.8rem;
  color: var(--text-secondary);
  flex: 1;
  line-height: 1.4;
}
.skill-meta {
  font-size: 0.7rem;
  color: var(--text-muted);
  display: flex;
  gap: 8px;
}
.skill-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.tag {
  background: var(--primary-light);
  color: var(--primary);
  border-radius: 4px;
  padding: 1px 6px;
  font-size: 0.7rem;
}
.card-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: 4px;
}
.install-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px 12px;
  background: var(--primary);
  color: #fff;
  border: none;
  border-radius: 5px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: background 0.15s;
}
.install-btn:hover {
  background: var(--primary-hover);
}
</style>
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

- [ ] **Step 3: Commit**

```bash
git add src/components/SkillCard.vue
git commit -m "refactor: SkillCard with CSS variables and Lucide icons"
```

---

### Task 12: Update Modal Components

**Files:**
- Modify: `src/components/RepoManager.vue`
- Modify: `src/components/SkillDialog.vue`

- [ ] **Step 1: Update RepoManager.vue — replace `<style scoped>` with CSS variables, add Escape key**

In `src/components/RepoManager.vue`:

Keep the existing `<script setup>` block unchanged.

In the `<template>`, change the `.modal-overlay` div to add `tabindex="0"` and `@keydown.escape`:

```vue
<div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')" ref="overlayRef">
```

Replace the entire `<style scoped>` block with:

```css
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
}
h2 { margin: 0 0 8px; color: var(--text-primary); }
.hint { color: var(--text-secondary); font-size: 0.85rem; margin-bottom: 20px; }
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 20px;
}
label { font-size: 0.85rem; font-weight: 500; color: var(--text-primary); }
input {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 0.95rem;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-app);
  color: var(--text-primary);
}
input:focus {
  border-color: var(--primary);
  outline: none;
}
input.error { border-color: var(--danger); }
.error-msg { color: var(--danger); font-size: 0.8rem; }
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
}
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
button:hover {
  background: var(--bg-surface);
}
button.primary {
  background: var(--primary);
  color: #fff;
  border-color: var(--primary);
}
button.primary:hover { background: var(--primary-hover); }
</style>
```

- [ ] **Step 2: Update SkillDialog.vue — same pattern**

In `src/components/SkillDialog.vue`:

Keep the existing `<script setup>` block unchanged.

In the `<template>`, change the `.modal-overlay` div to add `tabindex="0"` and `@keydown.escape`:

```vue
<div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
```

Replace the entire `<style scoped>` block with:

```css
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
  width: 520px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; color: var(--text-primary); }
.desc { color: var(--text-secondary); font-size: 0.85rem; margin: 0; }
.section { display: flex; flex-direction: column; gap: 8px; }
label { font-size: 0.85rem; font-weight: 500; color: var(--text-primary); }
.radio-group { display: flex; flex-direction: column; gap: 6px; }
.radio { font-weight: normal; display: flex; align-items: center; gap: 8px; cursor: pointer; color: var(--text-secondary); }
.path-label { margin-top: 8px; }
select, input[type="text"] {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-app);
  color: var(--text-primary);
}
select:focus, input[type="text"]:focus {
  border-color: var(--primary);
  outline: none;
}
.checkbox { font-weight: normal; display: flex; align-items: center; gap: 6px; cursor: pointer; color: var(--text-secondary); }
.preview {
  background: var(--primary-light);
  border-radius: 6px;
  padding: 10px 14px;
  font-size: 0.85rem;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.preview span { color: var(--text-secondary); }
.preview code { word-break: break-all; color: var(--text-primary); }
.error-box {
  background: var(--danger-light);
  border-radius: 6px;
  padding: 10px 14px;
  color: var(--danger);
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 12px;
}
.overwrite-btn {
  padding: 4px 12px;
  background: var(--danger);
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  white-space: nowrap;
}
.overwrite-btn:hover {
  background: var(--danger-hover);
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

- [ ] **Step 3: Verify build**

```bash
pnpm build
```

- [ ] **Step 4: Commit**

```bash
git add src/components/RepoManager.vue src/components/SkillDialog.vue
git commit -m "refactor: modal components with CSS variables and Escape key support"
```

---

### Task 13: Restructure App.vue (Wire Everything)

**Files:**
- Modify: `src/App.vue`

- [ ] **Step 1: Rewrite App.vue**

Replace entire content with:

```vue
<!-- src/App.vue -->
<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import type { Skill } from './types';
import { useRepos } from './composables/useRepos';
import { useSkills } from './composables/useSkills';
import { useToast } from './composables/useToast';
import IconRail from './components/IconRail.vue';
import RepoPanel from './components/RepoPanel.vue';
import MainContent from './components/MainContent.vue';
import Toast from './components/Toast.vue';
import RepoManager from './components/RepoManager.vue';
import SkillDialog from './components/SkillDialog.vue';

const {
  repos,
  reposLoading,
  error: reposError,
  loadRepos,
  addRepo,
  removeRepo,
  updateRepo,
  updateAllRepos,
} = useRepos();

const { clearCache } = useSkills();
const { addToast } = useToast();

const selectedRepoUrl = ref<string | null>(null);
const showAddRepo = ref(false);
const selectedSkill = ref<Skill | null>(null);

// Watch for errors and show toast
watch(reposError, (err) => {
  if (err) addToast(err, 'error');
});

onMounted(async () => {
  await loadRepos();
  if (repos.value.length > 0) {
    selectedRepoUrl.value = repos.value[0].url;
  }
});

async function handleAddRepo(url: string) {
  try {
    await addRepo(url);
    showAddRepo.value = false;
    addToast('仓库添加成功', 'success');
  } catch (e) {
    // error already handled by watch
  }
}

async function handleUpdateAll() {
  try {
    const results = await updateAllRepos();
    addToast(results.join('\n') || '所有仓库已更新', 'success');
  } catch (e) {
    // error already handled by watch
  }
}

async function handleUpdateRepo(url: string) {
  try {
    const result = await updateRepo(url);
    addToast(result || '更新完成', 'success');
  } catch (e) {
    // error already handled by watch
  }
}

async function handleRemoveRepo(url: string) {
  if (!confirm('确定删除该仓库？本地克隆的文件也会被删除。')) return;
  try {
    await removeRepo(url);
    clearCache(url);
    addToast('仓库已删除', 'success');
  } catch (e) {
    // error already handled by watch
  } finally {
    // Always reset selection even on error
    selectedRepoUrl.value = repos.value.length > 0 ? repos.value[0].url : null;
  }
}
</script>

<template>
  <div class="app-layout">
    <IconRail />
    <RepoPanel
      :selected-repo-url="selectedRepoUrl"
      @select="selectedRepoUrl = $event"
      @add="showAddRepo = true"
      @remove="handleRemoveRepo"
      @update="handleUpdateRepo"
      @update-all="handleUpdateAll"
    />
    <MainContent
      :repo-url="selectedRepoUrl"
      @install-skill="selectedSkill = $event"
      @update-repo="handleUpdateRepo"
    />
  </div>

  <!-- Modals -->
  <RepoManager
    v-if="showAddRepo"
    @add="handleAddRepo"
    @close="showAddRepo = false"
  />
  <SkillDialog
    v-if="selectedSkill"
    :skill="selectedSkill"
    @close="selectedSkill = null"
    @installed="addToast('技能安装成功', 'success')"
  />

  <!-- Toast notifications -->
  <Toast />
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}
</style>
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

Expected: Build succeeds. All type checks pass.

- [ ] **Step 3: Commit**

```bash
git add src/App.vue
git commit -m "feat: restructure App.vue with three-panel layout and toast integration"
```

---

### Task 14: Remove Old RepoGroup

**Files:**
- Remove: `src/components/RepoGroup.vue`

- [ ] **Step 1: Delete RepoGroup.vue**

```bash
rm src/components/RepoGroup.vue
```

- [ ] **Step 2: Verify build**

```bash
pnpm build
```

Expected: Build succeeds. RepoGroup is no longer imported anywhere.

- [ ] **Step 3: Commit**

```bash
git add -A
git commit -m "chore: remove RepoGroup.vue (replaced by RepoPanel + MainContent)"
```

---

### Task 15: Final Verification

- [ ] **Step 1: Run full build**

```bash
pnpm build
```

Expected: Clean build with no type errors.

- [ ] **Step 2: Start dev server for visual check**

```bash
pnpm tauri dev
```

Manual verification checklist:
- [ ] Three-panel layout renders correctly (Icon Rail + Repo Panel + Main Content)
- [ ] Theme toggle cycles: system -> light -> dark (check icon changes: Monitor -> Sun -> Moon)
- [ ] Light mode: white/light gray backgrounds, dark text
- [ ] Dark mode: dark navy backgrounds, light text
- [ ] Click repo in sidebar -> selected (blue left border) -> skills load in main area
- [ ] Hover repo item -> update/delete buttons appear
- [ ] Click skill card -> install dialog opens
- [ ] Click "+ 添加仓库" -> modal opens
- [ ] Press Escape in modal -> closes
- [ ] Toast notifications appear bottom-right and auto-dismiss
- [ ] Window cannot be resized below 800x500
