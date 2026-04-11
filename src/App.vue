<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { ask } from '@tauri-apps/plugin-dialog';
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
import SettingsDialog from './components/SettingsDialog.vue';
import { useSettings } from './composables/useSettings';

const {
  repos,
  error: reposError,
  loadRepos,
  addRepo,
  removeRepo,
  updateRepo,
  updateAllRepos,
} = useRepos();

const { clearCache } = useSkills();
const { addToast } = useToast();
const { loadSettings } = useSettings();

const selectedRepoUrl = ref<string | null>(null);
const showAddRepo = ref(false);
const showSettings = ref(false);
const selectedSkill = ref<Skill | null>(null);

// Watch for errors and show toast
watch(reposError, (err) => {
  if (err) addToast(err, 'error');
});

onMounted(async () => {
  await Promise.all([loadRepos(), loadSettings()]);
  if (repos.value.length > 0) {
    selectedRepoUrl.value = repos.value[0].url;
  }
});

function handleAddRepo(url: string) {
  showAddRepo.value = false;
  addRepo(url)
    .then(() => addToast('仓库添加成功', 'success'))
    .catch(() => { /* error already handled by watch */ });
}

async function handleUpdateAll() {
  try {
    const results = await updateAllRepos();
    await loadRepos();
    addToast(results.join('\n') || '所有仓库已更新', 'success');
  } catch (e) {
    // error already handled by watch
  }
}

async function handleUpdateRepo(url: string) {
  try {
    const result = await updateRepo(url);
    await loadRepos();
    addToast(result || '更新完成', 'success');
  } catch (e) {
    // error already handled by watch
  }
}

async function handleRemoveRepo(url: string) {
  const confirmed = await ask('确定删除该仓库？本地克隆的文件也会被删除。', {
    title: '删除仓库',
    kind: 'warning',
    okLabel: '删除',
    cancelLabel: '取消',
  });
  if (!confirmed) return;
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
    <IconRail @settings="showSettings = true" />
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
  <SettingsDialog
    v-if="showSettings"
    @close="showSettings = false"
  />
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
