<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { Repo, Skill } from '../types';
import { useSkills } from '../composables/useSkills';
import { useRepos } from '../composables/useRepos';
import { parseRepoUrl } from '../utils/repo';
import SkillCard from './SkillCard.vue';
import { RefreshCw, Loader2, Inbox, Copy, Check } from 'lucide-vue-next';

const props = defineProps<{
  repoUrl: string | null;
}>();

const emit = defineEmits<{
  installSkill: [skill: Skill];
  updateRepo: [url: string];
}>();

const { repos } = useRepos();
const { loadSkills } = useSkills();

const skills = ref<Skill[]>([]);
const loading = ref(false);

const currentRepo = ref<Repo | null>(null);
const copied = ref(false);

function copyRepoUrl() {
  if (!currentRepo.value) return;
  navigator.clipboard.writeText(currentRepo.value.url).then(() => {
    copied.value = true;
    setTimeout(() => { copied.value = false; }, 1500);
  });
}

const repoMeta = computed(() => {
  if (!currentRepo.value) return null;
  return parseRepoUrl(currentRepo.value.url);
});

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
          <div class="title-row">
            <h2 class="repo-title">{{ repoMeta?.owner || currentRepo.name }}</h2>
            <template v-if="repoMeta?.name">
              <span class="title-sep">/</span>
              <span class="repo-name">{{ repoMeta.name }}</span>
            </template>
          </div>
          <button class="url-chip" @click="copyRepoUrl" :title="copied ? '已复制' : '点击复制地址'">
            <span class="url-text">{{ currentRepo!.url }}</span>
            <component :is="copied ? Check : Copy" :size="11" class="copy-icon" :class="{ copied }" />
          </button>
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
  padding: 14px 20px;
  border-bottom: 1px solid var(--border);
}
.header-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}
.title-row {
  display: flex;
  align-items: baseline;
  gap: 6px;
  min-width: 0;
}
.repo-title {
  margin: 0;
  font-size: 1.05rem;
  font-weight: 600;
  color: var(--text-primary);
  white-space: nowrap;
}
.title-sep {
  color: var(--text-muted);
  font-weight: 300;
}
.repo-name {
  font-size: 1.05rem;
  font-weight: 450;
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}
.url-chip {
  display: inline-flex;
  align-items: center;
  gap: 5px;
  padding: 2px 10px;
  border-radius: 4px;
  background: transparent;
  border: 1px solid var(--border);
  cursor: pointer;
  transition: background 0.15s, border-color 0.15s;
  max-width: 100%;
  align-self: flex-start;
  font-family: inherit;
}
.url-chip:hover {
  background: var(--bg-surface-hover);
  border-color: var(--text-muted);
}
.url-text {
  font-size: 0.72rem;
  color: var(--text-muted);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 360px;
}
.copy-icon {
  color: var(--text-muted);
  flex-shrink: 0;
  transition: color 0.15s;
}
.copy-icon.copied {
  color: var(--success, #22c55e);
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
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
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
