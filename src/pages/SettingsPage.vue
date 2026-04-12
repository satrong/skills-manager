<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { useRouter } from 'vue-router'
import type { ToolType } from '../types'
import { TOOL_LABELS } from '../utils/toolPaths'
import { useSettings } from '../composables/useSettings'
import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft } from 'lucide-vue-next'

const router = useRouter()

const { defaultToolType, setDefaultToolType, clearProjectPaths } = useSettings()

const projectPathCount = ref<number | null>(null)
const toolPathCount = ref<number | null>(null)
const clearing = ref(false)

const tools: { value: ToolType; label: string }[] = (
  Object.entries(TOOL_LABELS) as [ToolType, string][]
).map(([value, label]) => ({ value, label }))

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
      <button class="back-btn" @click="router.push({ name: 'skills' })">
        <ArrowLeft :size="18" />
      </button>
      <h2>设置</h2>
    </div>

    <div class="settings-body">
      <div class="section">
        <label>默认目标工具</label>
        <p class="desc">安装技能时默认选中的目标工具。</p>
        <select v-model="defaultToolType">
          <option v-for="tool in tools" :key="tool.value" :value="tool.value">
            {{ tool.label }}
          </option>
        </select>
      </div>

      <div class="section">
        <label>路径历史</label>
        <p class="desc">
          安装技能时的路径记录（项目路径 {{ projectPathCount ?? '...' }} 条，工具路径 {{ toolPathCount ?? '...' }} 条）。
        </p>
        <button
          class="danger-btn"
          :disabled="clearing || totalCount === 0"
          @click="handleClearAll"
        >
          {{ clearing ? '清空中...' : '清空所有路径历史' }}
        </button>
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
  padding: 24px 20px;
  max-width: 480px;
  display: flex;
  flex-direction: column;
  gap: 20px;
}
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
</style>
