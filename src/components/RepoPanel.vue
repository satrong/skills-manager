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

const { repos, updateAllLoading, isUpdateLoading, addRepoUrl } = useRepos();

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
        :class="['repo-item', { selected: repo.url === selectedRepoUrl, 'is-cloning': addRepoUrl === repo.url }]"
        @click="emit('select', repo.url)"
      >
        <div class="repo-info">
          <div class="repo-name">{{ repo.name }}</div>
          <div class="repo-skill-count">{{ addRepoUrl === repo.url ? '克隆中...' : (repo.lastUpdate ? '已同步' : '新仓库') }}</div>
        </div>
        <div class="repo-actions" @click.stop>
          <template v-if="addRepoUrl === repo.url">
            <Loader2 :size="13" class="spin repo-loading" />
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
  align-items: center;
  gap: 2px;
  opacity: 0;
  transition: opacity 0.15s;
}
.repo-item:hover .repo-actions {
  opacity: 1;
}
.repo-item.is-cloning .repo-actions {
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
