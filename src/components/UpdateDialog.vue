<script setup lang="ts">
import { ref, computed } from 'vue'
import { useUpdate } from '../composables/useUpdate'
import { useI18n } from '../i18n'
import { invoke } from '@tauri-apps/api/core'

const emit = defineEmits<{
  close: []
}>()

const { latestVersion, updateBody, downloading, downloadProgress, downloadAndInstall } = useUpdate()
const { t } = useI18n()

const currentVersion = ref('')
const downloadError = ref('')

async function loadVersion() {
  try {
    currentVersion.value = await invoke<string>('get_app_version')
  } catch {
    currentVersion.value = ''
  }
}
loadVersion()

const progressPercent = computed(() => Math.min(downloadProgress.value, 100))

async function handleUpdate() {
  downloadError.value = ''
  try {
    await downloadAndInstall()
  } catch {
    downloadError.value = t('update.downloadError')
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
    <div class="modal">
      <h2>{{ t('update.title') }}</h2>

      <div class="version-info">
        <div class="version-row">
          <span class="version-label">{{ t('update.currentVersion') }}</span>
          <span class="version-value">v{{ currentVersion }}</span>
        </div>
        <div class="version-row">
          <span class="version-label">{{ t('update.newVersion') }}</span>
          <span class="version-value highlight">v{{ latestVersion }}</span>
        </div>
      </div>

      <div v-if="updateBody" class="changelog">
        <label>{{ t('update.changelog') }}</label>
        <pre class="changelog-body">{{ updateBody }}</pre>
      </div>

      <div v-if="downloading" class="progress-section">
        <div class="progress-bar">
          <div class="progress-fill" :style="{ width: progressPercent + '%' }"></div>
        </div>
        <span class="progress-text">{{ t('update.downloadProgress', { progress: progressPercent }) }}</span>
      </div>

      <div v-if="downloadError" class="error-box">
        {{ downloadError }}
      </div>

      <div class="actions">
        <button v-if="!downloading" @click="emit('close')">{{ t('update.later') }}</button>
        <button
          v-if="progressPercent >= 100"
          class="primary"
          @click="() => { /* updater handles restart */ }"
        >
          {{ t('update.restartInstall') }}
        </button>
        <button
          v-else
          class="primary"
          @click="handleUpdate"
          :disabled="downloading"
        >
          {{ downloading ? t('update.downloading') : t('update.updateNow') }}
        </button>
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
  width: 460px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; color: var(--text-primary); }
.version-info {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.version-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
}
.version-label {
  font-size: 0.85rem;
  color: var(--text-secondary);
}
.version-value {
  font-size: 0.85rem;
  color: var(--text-primary);
  font-variant-numeric: tabular-nums;
}
.version-value.highlight {
  color: var(--primary);
  font-weight: 600;
}
.changelog {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.changelog label {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
}
.changelog-body {
  background: var(--bg-surface-sunken);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 12px;
  font-size: 0.8rem;
  color: var(--text-secondary);
  max-height: 200px;
  overflow-y: auto;
  white-space: pre-wrap;
  word-break: break-word;
  margin: 0;
}
.progress-section {
  display: flex;
  flex-direction: column;
  gap: 6px;
}
.progress-bar {
  height: 6px;
  background: var(--bg-surface-sunken);
  border-radius: 3px;
  overflow: hidden;
}
.progress-fill {
  height: 100%;
  background: var(--primary);
  border-radius: 3px;
  transition: width 0.3s ease;
}
.progress-text {
  font-size: 0.8rem;
  color: var(--text-secondary);
  text-align: center;
}
.error-box {
  background: var(--danger-light);
  border-radius: 6px;
  padding: 10px 14px;
  color: var(--danger);
  font-size: 0.85rem;
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
button:disabled { opacity: 0.6; cursor: not-allowed; }
</style>
