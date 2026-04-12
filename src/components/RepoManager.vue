<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { useI18n } from '../i18n';

const emit = defineEmits<{
  add: [url: string];
  addLocal: [path: string];
  close: [];
}>();

const url = ref('');
const error = ref('');
const { t } = useI18n();

function validate(value: string): string {
  if (!value.trim()) return t('repoManager.urlRequired');
  if (!value.startsWith('https://github.com/')) {
    return t('repoManager.urlInvalid');
  }
  return '';
}

function handleSubmit() {
  const err = validate(url.value);
  if (err) {
    error.value = err;
    return;
  }
  emit('add', url.value.trim());
}

async function handleSelectDir() {
  const selected = await open({
    directory: true,
    multiple: false,
    title: t('repoManager.selectDirTitle'),
  });
  if (selected) {
    emit('addLocal', selected);
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
    <div class="modal">
      <h2>{{ t('repoManager.title') }}</h2>
      <p class="hint">{{ t('repoManager.hint') }}</p>

      <form @submit.prevent="handleSubmit">
        <div class="field">
          <label>{{ t('repoManager.urlLabel') }}</label>
          <input
            v-model="url"
            type="text"
            :placeholder="t('repoManager.urlPlaceholder')"
            :class="{ error: error }"
            @input="error = ''"
            autofocus
          />
          <span v-if="error" class="error-msg">{{ error }}</span>
        </div>
        <div class="actions">
          <button type="button" class="secondary" @click="handleSelectDir">
            <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>
            </svg>
            <span>{{ t('repoManager.selectLocalDir') }}</span>
          </button>
          <button type="submit" class="primary">{{ t('repoManager.cloneAndAdd') }}</button>
        </div>
      </form>
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
}
h2 { margin: 0 0 8px; color: var(--text-primary); }
.hint { color: var(--text-secondary); font-size: 0.85rem; margin-bottom: 20px; }
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 20px;
}
label { font-size: 0.85rem; font-weight: 500; color: var(--text-primary); }
input {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 0.95rem;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-app);
  color: var(--text-primary);
}
input:focus {
  border-color: var(--primary);
  outline: none;
}
input.error { border-color: var(--danger); }
.error-msg { color: var(--danger); font-size: 0.8rem; }
.actions {
  display: flex;
  justify-content: flex-end;
  gap: 8px;
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
button:hover {
  background: var(--bg-surface);
}
button.primary {
  background: var(--primary);
  color: #fff;
  border-color: var(--primary);
}
button.primary:hover { background: var(--primary-hover); }
button.secondary {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-right: auto;
}
</style>
