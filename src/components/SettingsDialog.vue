<script setup lang="ts">
import { ref, computed } from 'vue';
import type { ToolType } from '../types';
import { TOOL_LABELS, getToolLabel } from '../utils/toolPaths';
import { useSettings } from '../composables/useSettings';
import { useUpdate } from '../composables/useUpdate';
import { useToast } from '../composables/useToast';
import { useI18n } from '../i18n';
import { invoke } from '@tauri-apps/api/core';

const emit = defineEmits<{
  close: [];
}>();

const { defaultToolType, setDefaultToolType, clearProjectPaths } = useSettings();
const { addToast } = useToast();
const { updateAvailable, latestVersion, checking, checkForUpdate } = useUpdate();
const { locale, t } = useI18n();

const selected = ref<ToolType>(defaultToolType.value);
const projectPathCount = ref<number | null>(null);
const toolPathCount = ref<number | null>(null);
const clearing = ref(false)
const appVersion = ref('')
const checkUpdateText = ref('')

async function loadVersion() {
  try {
    appVersion.value = await invoke<string>('get_app_version')
  } catch {
    appVersion.value = ''
  }
}
loadVersion();

const tools = computed<{ value: ToolType; label: string }[]>(() =>
  (Object.entries(TOOL_LABELS) as [ToolType, string][]).map(([value]) => ({ value, label: getToolLabel(value, t('tool.custom')) }))
);

async function loadCounts() {
  try {
    const config = await invoke<{ projectPaths: string[]; toolPaths: Record<string, string> }>('load_config');
    projectPathCount.value = config.projectPaths.length;
    toolPathCount.value = Object.keys(config.toolPaths).length;
  } catch {
    projectPathCount.value = 0;
    toolPathCount.value = 0;
  }
}
loadCounts();

const totalCount = computed(() => (projectPathCount.value ?? 0) + (toolPathCount.value ?? 0));

async function handleClearAll() {
  clearing.value = true;
  try {
    await Promise.all([
      clearProjectPaths(),
      invoke('clear_tool_paths'),
    ]);
    projectPathCount.value = 0;
    toolPathCount.value = 0;
  } catch { /* ignore */ }
  clearing.value = false;
}

async function handleCheckUpdate() {
  checkUpdateText.value = ''
  try {
    await checkForUpdate()
    if (updateAvailable.value) {
      checkUpdateText.value = t('settings.updateAvailable') + ' v' + latestVersion.value
    } else {
      checkUpdateText.value = t('settings.upToDate')
      addToast(t('settings.upToDate'), 'success')
    }
  } catch {
    checkUpdateText.value = ''
    addToast(t('settings.updateError'), 'error')
  }
}

function handleSave() {
  setDefaultToolType(selected.value);
  emit('close');
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
    <div class="modal">
      <h2>{{ t('settings.title') }}</h2>

      <div class="section">
        <label>{{ t('settings.language') }}</label>
        <p class="desc">{{ t('settings.languageDesc') }}</p>
        <select v-model="locale">
          <option value="auto">{{ t('settings.langAuto') }}</option>
          <option value="zh-CN">{{ t('settings.langZhCN') }}</option>
          <option value="en">{{ t('settings.langEn') }}</option>
        </select>
      </div>

      <div class="section">
        <label>{{ t('settings.defaultTool') }}</label>
        <p class="desc">{{ t('settings.defaultToolDesc') }}</p>
        <select v-model="selected">
          <option v-for="tool in tools" :key="tool.value" :value="tool.value">
            {{ tool.label }}
          </option>
        </select>
      </div>

      <div class="section">
        <label>{{ t('settings.pathHistory') }}</label>
        <p class="desc">
          {{ t('settings.pathHistoryDesc', { projectCount: projectPathCount ?? '...', toolPathCount: toolPathCount ?? '...' }) }}
        </p>
        <button
          class="danger-btn"
          :disabled="clearing || totalCount === 0"
          @click="handleClearAll"
        >
          {{ clearing ? t('settings.clearing') : t('settings.clearAllPaths') }}
        </button>
      </div>

      <div class="section version-section" v-if="appVersion">
        <label>{{ t('settings.version') }}</label>
        <div class="version-right">
          <span class="version-value">v{{ appVersion }}</span>
          <button
            class="check-update-btn"
            :disabled="checking"
            @click="handleCheckUpdate"
          >
            {{ checking ? t('settings.checking') : t('settings.checkUpdate') }}
          </button>
        </div>
      </div>

      <div v-if="checkUpdateText" class="section">
        <span class="update-status" :class="{ 'has-update': updateAvailable }">{{ checkUpdateText }}</span>
      </div>

      <div class="actions">
        <button @click="emit('close')">{{ t('install.cancel') }}</button>
        <button class="primary" @click="handleSave">{{ t('settings.save') }}</button>
      </div>
    </div>
  </div>
</template>

<style scoped>
.modal-overlay {
  position: fixed;
  inset: 0;
  background: var(--bg-scrim);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}
.modal {
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 24px;
  width: 420px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; color: var(--text-primary); }
.desc { color: var(--text-secondary); font-size: 0.85rem; margin: 0; }
.section { display: flex; flex-direction: column; gap: 6px; }
label { font-size: 0.85rem; font-weight: 500; color: var(--text-primary); }
select {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-app);
  color: var(--text-primary);
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='M2 4l4 4 4-4' fill='none' stroke='%2394a3b8' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 30px;
}
select:focus {
  border-color: var(--primary);
  outline: none;
}
.actions { display: flex; justify-content: flex-end; gap: 8px; }
button {
  padding: 8px 20px;
  border-radius: 6px;
  cursor: pointer;
  border: 1px solid var(--border);
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  font-size: 0.85rem;
  transition: background 0.15s;
}
button:hover { background: var(--bg-surface); }
button.primary { background: var(--primary); color: #fff; border-color: var(--primary); }
button.primary:hover:not(:disabled) { background: var(--primary-hover); }
.danger-btn {
  align-self: flex-start;
  padding: 6px 16px;
  font-size: 0.8rem;
  background: var(--danger-light);
  color: var(--danger);
  border-color: transparent;
}
.danger-btn:hover:not(:disabled) { background: var(--danger); color: #fff; }
.danger-btn:disabled { opacity: 0.5; cursor: not-allowed; }
.version-section {
  align-items: center;
  flex-direction: row !important;
  justify-content: space-between;
}
.version-right {
  display: flex;
  align-items: center;
  gap: 10px;
}
.version-value {
  color: var(--text-secondary);
  font-size: 0.85rem;
  font-variant-numeric: tabular-nums;
}
.check-update-btn {
  padding: 4px 14px;
  font-size: 0.8rem;
  border-radius: 4px;
  border: 1px solid var(--border);
  background: var(--bg-surface-hover);
  color: var(--text-primary);
  cursor: pointer;
  transition: background 0.15s;
}
.check-update-btn:hover:not(:disabled) {
  background: var(--bg-surface);
}
.check-update-btn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}
.update-status {
  font-size: 0.85rem;
  color: var(--text-secondary);
}
.update-status.has-update {
  color: var(--primary);
  font-weight: 500;
}
</style>
