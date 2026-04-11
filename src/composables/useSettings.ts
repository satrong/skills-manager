import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ToolType } from '../types';

const defaultToolType = ref<ToolType>('claude-code');
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
}

async function setDefaultToolType(toolType: ToolType) {
  await invoke('set_default_tool_type', { toolType });
  defaultToolType.value = toolType;
}

export function useSettings() {
  return {
    defaultToolType,
    loadSettings,
    setDefaultToolType,
  };
}
