import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { Skill } from '../types';

// Module-scoped state (singleton)
const skillsByRepo = ref<Record<string, Skill[]>>({});
const loadingByRepo = ref<Record<string, boolean>>({});

// Helper to trigger Vue reactivity when deleting keys from Record
function deleteSkillEntry(repoUrl: string) {
  const copy = { ...skillsByRepo.value };
  delete copy[repoUrl];
  skillsByRepo.value = copy;
}

export function useSkills() {
  async function loadSkills(repoUrl: string, forceRefresh = false): Promise<Skill[]> {
    // Return cached if available and not forcing refresh
    if (!forceRefresh && skillsByRepo.value[repoUrl]?.length) {
      return skillsByRepo.value[repoUrl];
    }

    loadingByRepo.value[repoUrl] = true;
    try {
      const skills = await invoke<Skill[]>('list_skills', { repoUrl });
      skillsByRepo.value[repoUrl] = skills;
      return skills;
    } finally {
      loadingByRepo.value[repoUrl] = false;
    }
  }

  function isLoading(repoUrl: string): boolean {
    return loadingByRepo.value[repoUrl] ?? false;
  }

  function getSkills(repoUrl: string): Skill[] {
    return skillsByRepo.value[repoUrl] ?? [];
  }

  function clearCache(repoUrl?: string) {
    if (repoUrl) {
      deleteSkillEntry(repoUrl);
    } else {
      skillsByRepo.value = {};
    }
  }

  return {
    skillsByRepo,
    loadingByRepo,
    loadSkills,
    isLoading,
    getSkills,
    clearCache,
  };
}
