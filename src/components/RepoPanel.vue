<script setup lang="ts">
import { computed } from 'vue';
import { useRepos } from '../composables/useRepos';
import { parseRepoUrl } from '../utils/repo';
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

const { repos, updateAllLoading, isUpdateLoading, addRepoUrl } = useRepos();

function formatTime(timestamp: string): string {
  if (!timestamp) return '';
  const date = new Date(Number(timestamp) * 1000);
  if (isNaN(date.getTime())) return '';
  const now = new Date();
  const diff = (now.getTime() - date.getTime()) / 1000;
  if (diff < 60) return '刚刚';
  if (diff < 3600) return `${Math.floor(diff / 60)} 分钟前`;
  if (diff < 86400) return `${Math.floor(diff / 3600)} 小时前`;
  if (diff < 2592000) return `${Math.floor(diff / 86400)} 天前`;
  const y = date.getFullYear();
  const m = String(date.getMonth() + 1).padStart(2, '0');
  const d = String(date.getDate()).padStart(2, '0');
  return now.getFullYear() === y ? `${m}-${d}` : `${y}-${m}-${d}`;
}

const repoList = computed(() => repos.value.map(r => ({ ...r, meta: parseRepoUrl(r.url) })));
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
      <TransitionGroup name="list">
        <div
          v-for="repo in repoList"
          :key="repo.url"
          :class="['repo-item', { selected: repo.url === selectedRepoUrl, 'is-cloning': addRepoUrl === repo.url }]"
          @click="emit('select', repo.url)"
        >
          <div class="repo-status-dot" :class="{ active: repo.url === selectedRepoUrl, cloning: addRepoUrl === repo.url }"></div>
          <div class="repo-info">
            <div class="repo-name">{{ repo.meta.owner || repo.name }}</div>
            <div class="repo-subtitle">{{ repo.meta.name }}</div>
          </div>
          <div class="repo-meta">
            <span v-if="addRepoUrl === repo.url" class="repo-updating">
              <Loader2 :size="11" class="spin" />
              <span>克隆中</span>
            </span>
            <span v-else class="repo-time">{{ formatTime(repo.lastUpdate) }}</span>
            <div class="repo-actions" @click.stop>
              <template v-if="addRepoUrl === repo.url">
                <span class="action-placeholder"></span>
              </template>
              <template v-else>
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
              </template>
            </div>
          </div>
        </div>
      </TransitionGroup>

      <div v-if="repoList.length === 0" class="empty">
        <div class="empty-icon">
          <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
            <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4"/>
            <path d="M9 18c-4.51 2-5-2-7-2"/>
          </svg>
        </div>
        <span>暂无仓库</span>
        <span class="empty-hint">点击下方按钮添加</span>
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
  padding: 8px 8px;
}

/* TransitionGroup animations */
.list-enter-active {
  transition: all 0.25s ease-out;
}
.list-leave-active {
  transition: all 0.2s ease-in;
}
.list-enter-from {
  opacity: 0;
  transform: translateX(-8px);
}
.list-leave-to {
  opacity: 0;
  transform: translateX(8px);
}
.list-move {
  transition: transform 0.25s ease;
}

.repo-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 10px;
  border-radius: 8px;
  cursor: pointer;
  position: relative;
  transition: background 0.18s ease, box-shadow 0.18s ease;
}
.repo-item:not(:last-child) {
  margin-bottom: 2px;
}
.repo-item:hover {
  background: var(--bg-surface-hover);
}
.repo-item.selected {
  background: var(--bg-surface-hover);
  box-shadow: inset 3px 0 0 var(--primary);
}

/* Status indicator dot */
.repo-status-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--border);
  flex-shrink: 0;
  transition: background 0.2s, box-shadow 0.2s;
}
.repo-item:hover .repo-status-dot {
  background: var(--text-muted);
}
.repo-status-dot.active {
  background: var(--primary);
  box-shadow: 0 0 6px color-mix(in srgb, var(--primary) 40%, transparent);
}
.repo-status-dot.cloning {
  background: var(--success);
  animation: pulse-dot 1.2s ease-in-out infinite;
}
@keyframes pulse-dot {
  0%, 100% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--success) 30%, transparent); }
  50% { box-shadow: 0 0 0 4px color-mix(in srgb, var(--success) 0%, transparent); }
}

.repo-info {
  flex: 1;
  min-width: 0;
}
.repo-name {
  font-size: 0.82rem;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  letter-spacing: -0.01em;
}
.repo-subtitle {
  font-size: 0.72rem;
  color: var(--text-muted);
  margin-top: 1px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  font-weight: 450;
}
.repo-item.selected .repo-subtitle {
  color: var(--text-secondary);
}

.repo-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  flex-shrink: 0;
}
.repo-time {
  font-size: 0.65rem;
  color: var(--text-muted);
  font-variant-numeric: tabular-nums;
}
.repo-updating {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: 0.65rem;
  color: var(--success);
}

.repo-actions {
  display: flex;
  align-items: center;
  gap: 1px;
  opacity: 0;
  transition: opacity 0.15s;
}
.repo-item:hover .repo-actions {
  opacity: 1;
}
.repo-item.is-cloning .repo-actions {
  opacity: 1;
}
.action-placeholder {
  display: inline-block;
  width: 26px;
}

/* Empty state */
.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 0.82rem;
  padding: 40px 12px 24px;
}
.empty-icon {
  color: var(--border);
  margin-bottom: 4px;
  transition: color 0.2s;
}
.empty:hover .empty-icon {
  color: var(--text-muted);
}
.empty-hint {
  font-size: 0.72rem;
  color: var(--text-muted);
  opacity: 0.6;
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

/* Icon buttons */
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
