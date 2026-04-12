<script setup lang="ts">
import { computed } from 'vue';
import { useRepos } from '../composables/useRepos';
import { parseRepoUrl } from '../utils/repo';
import { RefreshCw, Trash2, Plus, Loader2, Folder } from 'lucide-vue-next';

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

const repoList = computed(() => repos.value.map(r => ({
  ...r,
  meta: parseRepoUrl(r.url),
  isLocal: r.source === 'local',
})));
</script>

<template>
  <div class="repo-panel">
    <div class="panel-header">
      <span class="panel-title">技能集合</span>
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
            <div class="repo-name">{{ repo.isLocal ? repo.name : (repo.meta.owner || repo.name) }}</div>
            <div class="repo-subtitle">
              <span class="repo-subtitle-text">
                <Folder v-if="repo.isLocal" :size="12" class="local-folder-icon" />
                {{ repo.isLocal ? '本地目录' : repo.meta.name }}
              </span>
              <span v-if="repo.skillCount != null" class="skill-count">{{ repo.skillCount }}</span>
            </div>
          </div>
          <div class="repo-meta" v-if="repo.isLocal">
            <div class="repo-actions" @click.stop>
              <button
                class="icon-btn-sm danger"
                @click="emit('remove', repo.url)"
                title="删除"
              >
                <Trash2 :size="13" />
              </button>
            </div>
          </div>
          <div class="repo-meta" v-else>
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
        <span>添加来源</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.repo-panel {
  width: 260px;
  min-width: 260px;
  height: 100%;
  background: var(--card-bg);
  box-shadow: var(--panel-shadow);
  display: flex;
  flex-direction: column;
  position: relative;
  z-index: 1;
}
.panel-header {
  height: 53px;
  padding: 0 14px;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.panel-title {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.5px;
}
.repo-list {
  flex: 1;
  overflow-y: auto;
  padding: 6px 10px 10px;
  display: flex;
  flex-direction: column;
  gap: 2px;
}

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
  gap: 10px;
  padding: 10px 12px;
  border-radius: 8px;
  cursor: pointer;
  position: relative;
  transition: background 0.18s ease;
}
.repo-item:not(:last-child)::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 28px;
  right: 12px;
  height: 1px;
  background: var(--border);
  opacity: 0.5;
}
.repo-item:hover {
  background: var(--bg-surface-hover);
}
.repo-item.selected {
  background: var(--bg-surface-hover);
  box-shadow: inset 3px 0 0 var(--primary);
  border-radius: 8px;
}

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
  50% { box-shadow: 0 0 0 5px color-mix(in srgb, var(--success) 0%, transparent); }
}

.repo-info {
  flex: 1;
  min-width: 0;
}
.repo-name {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  letter-spacing: -0.01em;
}
.repo-subtitle {
  display: flex;
  align-items: center;
  gap: 5px;
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 1px;
  font-weight: 450;
}
.repo-subtitle-text {
  min-width: 0;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.repo-item:hover .repo-subtitle {
  color: var(--text-secondary);
}
.repo-item.selected .repo-subtitle {
  color: var(--text-secondary);
}

.skill-count {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  border-radius: 8px;
  background: color-mix(in srgb, var(--text-muted) 15%, transparent);
  color: var(--text-muted);
  font-size: 0.65rem;
  font-weight: 600;
  font-variant-numeric: tabular-nums;
  flex-shrink: 0;
  line-height: 1;
  transition: background 0.15s, color 0.15s;
}
.repo-item:hover .skill-count {
  background: color-mix(in srgb, var(--text-secondary) 15%, transparent);
  color: var(--text-secondary);
}
.repo-item.selected .skill-count {
  background: color-mix(in srgb, var(--primary) 15%, transparent);
  color: var(--primary);
}

.repo-meta {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  flex-shrink: 0;
}
.repo-time {
  font-size: 0.7rem;
  color: var(--text-muted);
  font-variant-numeric: tabular-nums;
}
.local-folder-icon {
  color: var(--text-muted);
  flex-shrink: 0;
  vertical-align: -1px;
}
.repo-updating {
  display: flex;
  align-items: center;
  gap: 3px;
  font-size: 0.8rem;
  color: var(--success);
}

.repo-actions {
  display: flex;
  align-items: center;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.18s ease;
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

.empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  color: var(--text-muted);
  font-size: 0.75rem;
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
  font-size: 0.75rem;
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
  font-size: 0.75rem;
  cursor: pointer;
  transition: background 0.15s, color 0.15s, border-color 0.15s;
}
.add-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  border-color: var(--primary);
}

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
