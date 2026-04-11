<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import type { Repo, Skill, ToolType } from '../types';
import { useSkills } from '../composables/useSkills';
import { useRepos } from '../composables/useRepos';
import SkillCard from './SkillCard.vue';
import type { QuickInstallEntry } from './SkillCard.vue';
import { Loader2, Inbox, Search, Copy, Check } from 'lucide-vue-next';
import { invoke } from '@tauri-apps/api/core';
import { useSettings } from '../composables/useSettings';
import { TOOL_LABELS } from '../utils/toolPaths';

const props = defineProps<{
  repoUrl: string | null;
}>();

const emit = defineEmits<{
  installSkill: [skill: Skill];
  quickInstallSkill: [skill: Skill, entry: QuickInstallEntry];
}>();

const { repos } = useRepos();
const { loadSkills } = useSkills();
const { defaultToolType, projectPaths, loadProjectPaths, removeProjectPath } = useSettings();

const skills = ref<Skill[]>([]);
const loading = ref(false);
const openDropdownId = ref<string | null>(null);
const searchQuery = ref('');

const toolPathsConfig = ref<Record<string, string>>({});

const quickInstallEntries = computed<QuickInstallEntry[]>(() => {
  const entries: QuickInstallEntry[] = [];
  if (projectPaths.value.length) {
    const tool = defaultToolType.value;
    entries.push({
      label: `项目安装 (${TOOL_LABELS[tool] || tool})`,
      installType: 'project',
      toolType: tool,
      targetPath: '',
      header: true,
    });
    for (const p of projectPaths.value) {
      entries.push({
        label: '',
        installType: 'project',
        toolType: tool,
        targetPath: p,
      });
    }
  }
  if (Object.keys(toolPathsConfig.value).length) {
    for (const [tool, path] of Object.entries(toolPathsConfig.value)) {
      entries.push({
        label: `全局安装 (${TOOL_LABELS[tool as ToolType] || tool})`,
        installType: 'global',
        toolType: tool as ToolType,
        targetPath: path,
      });
    }
  }
  return entries;
});

onMounted(async () => {
  try {
    const config = await invoke<{
      toolPaths: Record<string, string>;
      projectPaths: string[];
    }>('load_config');
    toolPathsConfig.value = config.toolPaths;
  } catch { /* ignore */ }
  await loadProjectPaths();
});

const currentRepo = ref<Repo | null>(null);
const copied = ref(false);
const isLocalRepo = computed(() => currentRepo.value?.source === 'local');

function copyRepoUrl() {
  if (!currentRepo.value) return;
  const textToCopy = isLocalRepo.value ? currentRepo.value.localPath : currentRepo.value.url;
  navigator.clipboard.writeText(textToCopy).then(() => {
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 1500);
  });
}

const filteredSkills = computed(() => {
  const q = searchQuery.value.trim().toLowerCase();
  if (!q) return skills.value;
  return skills.value.filter(
    s => s.name.toLowerCase().includes(q) || s.description.toLowerCase().includes(q),
  );
});

watch(
  () => props.repoUrl,
  async (url) => {
    if (!url) {
      skills.value = [];
      currentRepo.value = null;
      searchQuery.value = '';
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

function onDocumentClick() {
  if (openDropdownId.value) openDropdownId.value = null;
}

async function removeQuickInstallEntry(entry: QuickInstallEntry) {
  if (entry.installType === 'project') {
    try {
      await removeProjectPath(entry.targetPath);
    } catch { /* ignore */ }
  }
}
onMounted(() => document.addEventListener('click', onDocumentClick));
onUnmounted(() => document.removeEventListener('click', onDocumentClick));
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
            :quick-install-entries="quickInstallEntries"
            :open-dropdown="openDropdownId === skill.id"
            @install="emit('installSkill', $event)"
            @quick-install="(skill, entry) => emit('quickInstallSkill', skill, entry)"
            @remove-quick-install-entry="removeQuickInstallEntry"
            @toggle-dropdown="openDropdownId = openDropdownId === skill.id ? null : skill.id"
          />
        </div>
      </template>
    </template>
  </div>
</template>

<style scoped>
.main-content {
  flex: 1;
  height: 100%;
  overflow-y: auto;
  background: var(--content-bg);
  position: relative;
}
.sticky-header {
  position: sticky;
  top: 0;
  z-index: 10;
  height: 53px;
  padding: 0 20px;
  display: flex;
  align-items: center;
  background: var(--content-bg);
  border-bottom: 1px solid var(--border);
}
.sticky-header::after {
  content: '';
  position: absolute;
  bottom: -6px;
  left: 0;
  right: 0;
  height: 6px;
  background: linear-gradient(to bottom, color-mix(in srgb, var(--content-bg) 80%, transparent), transparent);
  pointer-events: none;
}
.skills-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 12px;
  padding: 12px 20px 20px;
}
.search-bar {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 7px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-elevated);
  transition: border-color 0.15s, box-shadow 0.15s;
}
.search-bar:focus-within {
  border-color: var(--primary);
  box-shadow: 0 0 0 2px color-mix(in srgb, var(--primary) 12%, transparent);
}
.search-icon {
  color: var(--text-muted);
  flex-shrink: 0;
}
.search-input {
  flex: 1;
  border: none;
  outline: none;
  background: transparent;
  font-size: 0.8rem;
  color: var(--text-primary);
  font-family: inherit;
  min-width: 0;
}
.search-input::placeholder,
.placeholder-text {
  color: var(--text-muted);
}
.placeholder-text {
  flex: 1;
  font-size: 0.8rem;
  white-space: nowrap;
  user-select: none;
}
.header-divider {
  width: 1px;
  height: 16px;
  background: var(--border);
  flex-shrink: 0;
}
.url-chip {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 2px 8px;
  border-radius: 4px;
  background: transparent;
  border: none;
  cursor: pointer;
  transition: background 0.15s;
  flex-shrink: 0;
  max-width: 320px;
  font-family: inherit;
}
.url-chip:hover {
  background: var(--bg-surface-hover);
}
.url-text {
  font-size: 0.75rem;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.copy-icon {
  color: var(--text-muted);
  flex-shrink: 0;
  transition: color 0.15s;
}
.copy-icon.copied {
  color: var(--success, #22c55e);
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
  font-size: 0.75rem;
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
