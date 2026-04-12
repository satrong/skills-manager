<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import type { ToolType } from '../types'
import { TOOL_LABELS, getToolLabel } from '../utils/toolPaths'
import { useSettings } from '../composables/useSettings'
import { useI18n } from '../i18n'
import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft, Wrench, History, Globe } from 'lucide-vue-next'

const router = useRouter()

const { defaultToolType, setDefaultToolType, clearProjectPaths } = useSettings()
const { locale, t } = useI18n()

const projectPathCount = ref<number | null>(null)
const toolPathCount = ref<number | null>(null)
const clearing = ref(false)

const tools = computed<{ value: ToolType; label: string }[]>(() =>
  (Object.entries(TOOL_LABELS) as [ToolType, string][]).map(([value]) => ({ value, label: getToolLabel(value, t('tool.custom')) }))
)

watch(defaultToolType, (val) => {
  setDefaultToolType(val)
})

async function loadCounts() {
  try {
    const config = await invoke<{ projectPaths: string[]; toolPaths: Record<string, string> }>('load_config')
    projectPathCount.value = config.projectPaths.length
    toolPathCount.value = Object.keys(config.toolPaths).length
  } catch {
    projectPathCount.value = 0
    toolPathCount.value = 0
  }
}
loadCounts()

const totalCount = computed(() => (projectPathCount.value ?? 0) + (toolPathCount.value ?? 0))

async function handleClearAll() {
  clearing.value = true
  try {
    await Promise.all([
      clearProjectPaths(),
      invoke('clear_tool_paths'),
    ])
    projectPathCount.value = 0
    toolPathCount.value = 0
  } catch { /* ignore */ }
  clearing.value = false
}
</script>

<template>
  <div class="settings-page">
    <div class="settings-header">
        <button class="back-btn" @click="router.back()">
        <ArrowLeft :size="18" />
      </button>
      <h2>{{ t('settings.title') }}</h2>
    </div>

    <div class="settings-body">
      <div class="settings-card">
        <div class="card-icon-wrap">
          <Globe :size="18" class="card-icon" />
        </div>
        <div class="card-content">
          <label>{{ t('settings.language') }}</label>
          <p class="desc">{{ t('settings.languageDesc') }}</p>
          <select v-model="locale">
            <option value="auto">{{ t('settings.langAuto') }}</option>
            <option value="zh-CN">{{ t('settings.langZhCN') }}</option>
            <option value="en">{{ t('settings.langEn') }}</option>
          </select>
        </div>
      </div>

      <div class="settings-card">
        <div class="card-icon-wrap">
          <Wrench :size="18" class="card-icon" />
        </div>
        <div class="card-content">
          <label>{{ t('settings.defaultTool') }}</label>
          <p class="desc">{{ t('settings.defaultToolDesc') }}</p>
          <select v-model="defaultToolType">
            <option v-for="tool in tools" :key="tool.value" :value="tool.value">
              {{ tool.label }}
            </option>
          </select>
        </div>
      </div>

      <div class="settings-card">
        <div class="card-icon-wrap danger">
          <History :size="18" class="card-icon" />
        </div>
        <div class="card-content">
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
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page {
  flex: 1;
  height: 100%;
  overflow-y: auto;
  background: var(--content-bg);
}
.settings-header {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 20px;
  border-bottom: 1px solid var(--border);
  position: sticky;
  top: 0;
  background: var(--content-bg);
  z-index: 10;
}
.settings-header::after {
  content: '';
  position: absolute;
  bottom: -6px;
  left: 0;
  right: 0;
  height: 6px;
  background: linear-gradient(to bottom, color-mix(in srgb, var(--content-bg) 80%, transparent), transparent);
  pointer-events: none;
}
.settings-header h2 {
  margin: 0;
  color: var(--text-primary);
  font-size: 1rem;
}
.back-btn {
  width: 32px;
  height: 32px;
  border-radius: 8px;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.15s, color 0.15s;
}
.back-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.settings-body {
  padding: 20px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}
.settings-card {
  display: flex;
  align-items: flex-start;
  gap: 14px;
  padding: 16px;
  border: 1px solid var(--border);
  border-radius: 10px;
  background: var(--card-bg);
  backdrop-filter: blur(8px);
  box-shadow: var(--card-shadow);
  transition: border-color 0.2s, box-shadow 0.2s, background 0.2s;
}
.settings-card:hover {
  border-color: var(--border-strong);
  box-shadow: var(--card-hover-shadow);
  background: var(--card-hover-bg);
}
.card-icon-wrap {
  flex-shrink: 0;
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--primary-light);
  display: flex;
  align-items: center;
  justify-content: center;
}
.card-icon-wrap.danger {
  background: var(--danger-light);
}
.card-icon {
  color: var(--primary);
}
.card-icon-wrap.danger .card-icon {
  color: var(--danger);
}
.card-content {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 6px;
  min-width: 0;
}
label {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
}
.desc {
  color: var(--text-secondary);
  font-size: 0.85rem;
  margin: 0;
  line-height: 1.45;
}
select {
  margin-top: 2px;
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
.danger-btn {
  align-self: flex-start;
  padding: 6px 16px;
  font-size: 0.8rem;
  border-radius: 6px;
  cursor: pointer;
  background: var(--danger-light);
  color: var(--danger);
  border-color: transparent;
  transition: background 0.15s, color 0.15s;
}
.danger-btn:hover:not(:disabled) { background: var(--danger); color: #fff; }
.danger-btn:disabled { opacity: 0.5; cursor: not-allowed; }
</style>
