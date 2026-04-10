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
