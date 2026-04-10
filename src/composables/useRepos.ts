import { ref, computed, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Repo } from '../types';

const repos = ref<Repo[]>([]);
const reposLoading = ref(false);
const addRepoUrl = ref<string | null>(null);
const updateLoading = ref<Record<string, boolean>>({});
const updateAllLoading = ref(false);
const removeRepoLoading = ref(false);
const error = ref<string | null>(null);

function extractRepoName(url: string): string {
  const parts = url.replace(/\/+$/, '').split('/');
  return parts[parts.length - 1] || url;
}

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
    addRepoUrl.value = url;
    error.value = null;

    // Optimistically add placeholder to list
    repos.value.push({
      url,
      localPath: '',
      name: extractRepoName(url),
      lastUpdate: '',
      skills: [],
    });

    try {
      const repo = await invoke<Repo>('add_repo', { url });
      // Replace placeholder with real data
      const index = repos.value.findIndex(r => r.url === url);
      if (index !== -1) {
        repos.value[index] = { ...repo, skills: [] };
      }
    } catch (e) {
      // Remove placeholder on error
      repos.value = repos.value.filter(r => r.url !== url);
      error.value = String(e);
      throw e;
    } finally {
      addRepoUrl.value = null;
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
    addRepoUrl: readonly(addRepoUrl),
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
