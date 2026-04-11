<script setup lang="ts">
import { ref, watch, onMounted } from 'vue';
import { ask } from '@tauri-apps/plugin-dialog';
import type { Skill } from './types';
import { useRepos } from './composables/useRepos';
import { useSkills } from './composables/useSkills';
import { useToast } from './composables/useToast';
import { useInstall } from './composables/useInstall';
import type { QuickInstallEntry } from './components/SkillCard.vue';
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
  addLocalDir,
  removeRepo,
  updateRepo,
  updateAllRepos,
} = useRepos();

const { clearCache } = useSkills();
const { addToast } = useToast();
const { loadSettings } = useSettings();
const { installSkill } = useInstall();

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

function handleAddLocalDir(path: string) {
  showAddRepo.value = false;
  addLocalDir(path)
    .then(() => addToast('本地目录添加成功', 'success'))
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
  const repo = repos.value.find(r => r.url === url);
  const isLocal = repo?.source === 'local';
  const message = isLocal
    ? '确定移除该本地目录？文件不会被删除。'
    : '确定删除该仓库？本地克隆的文件也会被删除。';
  const confirmed = await ask(message, {
    title: isLocal ? '移除本地目录' : '删除仓库',
    kind: 'warning',
    okLabel: isLocal ? '移除' : '删除',
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

async function handleQuickInstall(skill: Skill, entry: QuickInstallEntry) {
  try {
    await installSkill({
      skillId: skill.id,
      repoUrl: skill.repoUrl,
      installType: entry.installType,
      toolType: entry.toolType,
      targetPath: entry.targetPath,
      overwrite: true,
    });
    addToast('技能安装成功', 'success');
  } catch (e) {
    addToast(String(e), 'error');
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
      @quick-install-skill="handleQuickInstall"
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
    @add-local="handleAddLocalDir"
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
