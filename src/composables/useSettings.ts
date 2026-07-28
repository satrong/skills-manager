import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { ProxyConfig, ToolType } from '../types';

const defaultToolType = ref<ToolType>('claude-code');
const projectPaths = ref<string[]>([]);
const proxyEnabled = ref(false);
const proxyUrl = ref('');
let loaded = false;

// 实际生效的代理地址，未启用或为空时为 undefined
const effectiveProxy = computed(() => {
  const url = proxyUrl.value.trim();
  return proxyEnabled.value && url ? url : undefined;
});

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
  await loadProxyConfig();
}

async function loadProxyConfig() {
  try {
    const config = await invoke<ProxyConfig>('get_proxy_config');
    proxyEnabled.value = config.enabled;
    proxyUrl.value = config.url;
  } catch {
    proxyEnabled.value = false;
    proxyUrl.value = '';
  }
}

async function setProxyConfig(enabled: boolean, url: string) {
  const trimmed = url.trim();
  await invoke('set_proxy_config', { enabled, url: trimmed });
  proxyEnabled.value = enabled;
  proxyUrl.value = trimmed;
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
    proxyEnabled,
    proxyUrl,
    effectiveProxy,
    setProxyConfig,
    loadSettings,
    setDefaultToolType,
    loadProjectPaths,
    addProjectPath,
    removeProjectPath,
    clearProjectPaths,
  };
}
