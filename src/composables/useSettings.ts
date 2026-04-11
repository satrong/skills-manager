import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ToolType } from '../types';

const defaultToolType = ref<ToolType>('claude-code');
const projectPaths = ref<string[]>([]);
let loaded = false;

async function loadSettings() {
  if (loaded) return;
  loaded = true;
  try {
    const result = await invoke<string | null>('get_default_tool_type');
    defaultToolType.value = (result as ToolType | null) ?? 'claude-code';
  } catch {
    defaultToolType.value = 'claude-code';
  }
  await loadProjectPaths();
}

async function loadProjectPaths() {
  try {
    projectPaths.value = await invoke<string[]>('get_project_paths');
  } catch {
    projectPaths.value = [];
  }
}

async function setDefaultToolType(toolType: ToolType) {
  await invoke('set_default_tool_type', { toolType });
  defaultToolType.value = toolType;
}

async function addProjectPath(path: string) {
  await invoke('add_project_path', { path });
  await loadProjectPaths();
}

async function removeProjectPath(path: string) {
  await invoke('remove_project_path', { path });
  projectPaths.value = projectPaths.value.filter(p => p !== path);
}

async function clearProjectPaths() {
  await invoke('clear_project_paths');
  projectPaths.value = [];
}

export function useSettings() {
  return {
    defaultToolType,
    projectPaths,
    loadSettings,
    setDefaultToolType,
    loadProjectPaths,
    addProjectPath,
    removeProjectPath,
    clearProjectPaths,
  };
}
