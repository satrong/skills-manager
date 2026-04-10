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
const { loadSkills } = useSkills();

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
