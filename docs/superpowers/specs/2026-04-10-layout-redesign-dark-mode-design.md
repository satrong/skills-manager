# Layout Redesign & Dark Mode Support

## Goal

Redesign the Skills Manager Tauri desktop app with a three-panel IDE-style layout and a three-mode theme system (system/light/dark) with manual toggle.

## Layout Architecture

Three-column layout replacing the current single-column toolbar + content structure.

| Column | Width | Content |
|--------|-------|---------|
| Icon Rail | 56px fixed | App logo, navigation icons, theme toggle (bottom) |
| Repo Panel | 220px fixed | Repository list with selection state, "Add Repo" button at bottom |
| Main Content | Flex fill | Selected repo details + skill card grid |

### Data Flow

- **IconRail**: Static. Logo + theme toggle. No data state.
- **RepoPanel**: Reads from `useRepos()`. Click selects a repo, highlighting it (left blue border). Hover reveals update/delete actions.
- **MainContent**: Receives `selectedRepoUrl` prop, calls `useSkills().loadSkills(repoUrl)` internally to load and display skill cards.

### Shared State

- `selectedRepoUrl: Ref<string | null>` — lives in `App.vue`
- `RepoPanel` emits `select` event with repo URL; `App.vue` updates `selectedRepoUrl`
- `MainContent` receives `repoUrl` as a prop; watches it and calls `loadSkills(repoUrl)` on change
- Default: first repo selected on load (set in `onMounted` after `loadRepos()` completes)

### App.vue Responsibilities

`App.vue` provides:
- Three-column layout container
- Theme initialization (`useTheme()`)
- `selectedRepoUrl` state (passed to RepoPanel and MainContent)
- Dialog state (showAddRepo, selectedSkill)
- Toast container (`<Toast />`)
- Error watching: watches `useRepos().error` and feeds errors into `useToast()`

### Composable Refactoring

#### `useSkills` — module-scoped singleton

Current implementation creates a new cache per call site. Refactor to module-scoped state (same pattern as `useRepos`):
- `skillsByRepo: Map<string, Skill[]>` — persistent cache at module scope
- `loadingByRepo: Map<string, boolean>` — per-repo loading state
- Consumers call `loadSkills(repoUrl)` which checks cache first, fetches only if missing or explicitly refreshed

This prevents re-fetching skills when switching between repos.
- Export `clearCache(repoUrl?: string)` method: if `repoUrl` provided, delete that entry; if omitted, clear all. Called by `App.vue` after `removeRepo` to purge stale skill data.
- When `removeRepo` is called in `App.vue`: reset `selectedRepoUrl` to the first remaining repo (or null), then call `clearCache(deletedUrl)`.

#### Cache invalidation flow (repo removal)

1. User clicks delete on a repo in RepoPanel
2. `App.vue` calls `removeRepo(url)` from `useRepos()`
3. On success: `selectedRepoUrl` resets to `repos[0].url` (or null if empty)
4. `App.vue` calls `useSkills().clearCache(url)` to purge stale data

#### `useRepos` — per-operation loading

Add separate loading refs for different operations. Each async operation follows the same pattern: set loading true, clear error, try/catch/finally (set loading false).
- `reposLoading` — initial repo list fetch
- `addRepoLoading` — adding a new repo. After success, does NOT auto-select the new repo (user clicks it manually).
- `updateLoading: Map<string, boolean>` — per-repo update state (keyed by repo URL)
- `updateAllLoading` — bulk update
- Keep existing `error` ref for any operation failure
- `removeRepo` should also get a `removeRepoLoading` ref with proper try/catch/error handling (currently has none)

## Window Configuration

Update `src-tauri/tauri.conf.json`:
- Default size: `width: 1100, height: 750`
- Minimum size: `minWidth: 800, minHeight: 500`

## Theme System

### Three-Mode Toggle

| Mode | Behavior |
|------|----------|
| `system` | Follows `prefers-color-scheme` media query via `matchMedia` listener |
| `light` | Forces light theme |
| `dark` | Forces dark theme |

State persisted to `localStorage`. Cycles on click: system -> light -> dark -> system.

### Implementation

1. **`composables/useTheme.ts`**
   - Reactive state: `'system' | 'light' | 'dark'`
   - Computed `resolvedTheme`: `'light' | 'dark'` — resolves `system` via `matchMedia`
   - Applies CSS class `theme-light` or `theme-dark` to `<html>` element
   - Listens for system theme changes when in `system` mode

2. **CSS Variables** — defined in `src/assets/theme.css` (unscoped global stylesheet, imported in `src/main.ts`). Replaces all `@media (prefers-color-scheme: dark)` blocks:

```
:root, .theme-light {
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
```

`--bg-surface-sunken` is intentionally darker than `--bg-surface` in dark mode (sunken/recessed panel pattern for IconRail and RepoPanel).

3. **Theme Toggle Button**: Bottom of Icon Rail. Three-state icon (sun/moon/monitor). Click cycles through modes.

## Component Structure

```
App.vue
  IconRail.vue           (56px fixed column)
    Logo (app icon)
    ThemeToggle (bottom)

  RepoPanel.vue           (220px fixed column)
    Header section ("Repositories" label + "Update All" icon button)
    Repo list items (selectable, hover reveals actions)
    Footer section ("+ Add Repository" button)

  MainContent.vue         (flex fill)
    RepoHeader (name + url + update button)
    SkillCard grid (existing SkillCard.vue, optimized)
    Empty state / Loading state

  Toast.vue               (fixed bottom-right)

  RepoManager.vue         (modal, kept)
  SkillDialog.vue         (modal, kept)
```

`RepoPanelHeader` and `RepoPanelFooter` are template sections within `RepoPanel.vue`, not separate components.

### New Components

- **`IconRail.vue`**: 56px-wide vertical bar. Background `var(--bg-surface-sunken)`. Contains logo at top, theme toggle at bottom.
- **`RepoPanel.vue`**: 220px sidebar. Lists repos from `useRepos()`. Selected repo has left blue border + highlighted background. Hover on item reveals update/delete icon buttons. Footer has "Add Repository" button (opens `RepoManager` modal).
- **`MainContent.vue`**: Receives `repoUrl` prop. Shows repo header + skill card grid. Empty/loading states. Watches `repoUrl` prop to reload skills.
- **`Toast.vue`**: Fixed-position container at bottom-right. Renders active toasts from `useToast()`. Each toast has variant icon, message, optional dismiss button. Max 3 visible, newest at bottom. Auto-dismiss after 4 seconds.

### Modified Components

- **`SkillCard.vue`**: Remove emoji icons. Use Lucide SVG icons. All colors use CSS variables. Hover: subtle shadow + border color change (no layout shift). Tags use `var(--primary-light)` background.
- **`RepoManager.vue`**: Colors use CSS variables. Remove dark mode media query block. Add `@keydown.escape="emit('close')"` on overlay div.
- **`SkillDialog.vue`**: Same as above. Scrim uses `var(--bg-scrim)`. Add `@keydown.escape="emit('close')"` on overlay div.

### Removed Components

- **`RepoGroup.vue`**: Its responsibilities split into `RepoPanel.vue` (repo list item) and `MainContent.vue` (skill display for selected repo).

## Interaction Details

### Repo Selection
- Click repo item in RepoPanel -> highlights with left blue border + background change
- MainContent updates to show that repo's skills (with fade transition, 150ms)
- Default: first repo selected on load

### Repo Actions (hover reveal)
- Hover on repo item -> small Lucide icon buttons appear on the right (RefreshCw, Trash2)
- Delete: native `confirm()` kept for now (simple, functional). Can be replaced with themed dialog in future iteration.
- Update: calls `updateRepo(url)`, shows toast on result. Per-repo loading spinner replaces the update icon during operation.

### "Update All"
- Icon button in RepoPanel header (RefreshCw icon)
- Shows spinner during operation, toast with results on completion

### Toast System

- **`composables/useToast.ts`**: Provides `addToast(message: string, variant: 'success' | 'error' | 'info')`. Manages reactive `toasts` array with auto-removal after 4 seconds. Max 3 concurrent toasts — when a 4th is added, the oldest is immediately removed (not queued).
- **Integration**: `App.vue` watches `useRepos().error` and calls `addToast(error, 'error')` when it changes. All status-setting code uses `addToast` instead of `statusMessage`.
- **Variants**: success (green, CheckCircle icon), error (red, AlertCircle icon), info (blue, Info icon)

### Modals
- Overlay scrim: `var(--bg-scrim)`
- Modal card: `var(--bg-surface)` with `var(--border)` border
- Close on overlay click or Escape key (`@keydown.escape`)
- All form inputs styled with CSS variables

## Icons

- **Package**: `lucide-vue-next`
- **Required icons**: Sun, Moon, Monitor (theme toggle), RefreshCw (update/update-all), Trash2 (delete), Plus (add), Download (install), X (close modal), Package (logo placeholder)

## Visual Guidelines

- **Icons**: Lucide SVG icon set (stroke-width 1.5-2px, consistent)
- **No emoji**: Replace all `↻`, `✕`, `▶`, `▼` with Lucide SVG icons
- **Spacing**: 4/8px rhythm (8px base, 4px for tight gaps)
- **Border radius**: 6px buttons/inputs, 8px cards/panels
- **Typography**: Inter font, weights: 400 body, 500 labels, 600 headings
- **Transitions**: 150-200ms for hover/state changes, ease-out
- **Focus states**:
  ```css
  :focus-visible {
    outline: 2px solid var(--primary);
    outline-offset: 2px;
  }
  ```
- **Scroll**: Custom scrollbar (WebKit only, Tauri WebView):
  ```css
  ::-webkit-scrollbar { width: 6px; }
  ::-webkit-scrollbar-track { background: var(--bg-app); }
  ::-webkit-scrollbar-thumb { background: var(--border); border-radius: 3px; }
  ::-webkit-scrollbar-thumb:hover { background: var(--text-muted); }
  ```

## Files to Create

- `src/assets/theme.css` — global CSS variable definitions (unscoped)
- `src/composables/useTheme.ts`
- `src/composables/useToast.ts`
- `src/components/IconRail.vue`
- `src/components/RepoPanel.vue`
- `src/components/MainContent.vue`
- `src/components/Toast.vue`

## Files to Modify

- `src-tauri/tauri.conf.json` — update window size to 1100x750, add min size 800x500
- `src/main.ts` — import `./assets/theme.css`
- `src/App.vue` — restructure to three-column layout, add theme provider, selectedRepo state, Toast container
- `src/composables/useSkills.ts` — refactor to module-scoped singleton with persistent cache
- `src/composables/useRepos.ts` — add per-operation loading states
- `src/components/SkillCard.vue` — CSS variables, Lucide SVG icons
- `src/components/RepoManager.vue` — CSS variables, remove media queries, add Escape key handler
- `src/components/SkillDialog.vue` — CSS variables, remove media queries, add Escape key handler

## Files to Remove

- `src/components/RepoGroup.vue` — responsibilities split into RepoPanel + MainContent
