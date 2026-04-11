import { ref, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { SearchResult } from '../types';

const results = ref<SearchResult[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);

export function useSearch() {
  async function search(query: string): Promise<SearchResult[]> {
    if (!query.trim()) {
      results.value = [];
      return [];
    }

    loading.value = true;
    error.value = null;
    try {
      const data = await invoke<SearchResult[]>('search_skills', { query });
      results.value = data;
      return data;
    } catch (e) {
      error.value = String(e);
      results.value = [];
      return [];
    } finally {
      loading.value = false;
    }
  }

  function clear() {
    results.value = [];
    error.value = null;
  }

  return {
    results: readonly(results),
    loading: readonly(loading),
    error: readonly(error),
    search,
    clear,
  };
}
