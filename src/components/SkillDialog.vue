<script setup lang="ts">
import { ref, computed, watch, onMounted, onUnmounted } from 'vue';
import type { Skill, InstallType, ToolType } from '../types';
import { TOOL_LABELS, PROJECT_TOOL_DIRS } from '../utils/toolPaths';
import { useInstall } from '../composables/useInstall';
import { useSettings } from '../composables/useSettings';
import { listen, type UnlistenFn, TauriEvent } from '@tauri-apps/api/event';
import { open } from '@tauri-apps/plugin-dialog';
import { invoke } from '@tauri-apps/api/core';

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  close: [];
  installed: [skillId: string];
}>();

const { getToolPath, installSkill } = useInstall();
const { defaultToolType } = useSettings();

const installType = ref<InstallType>('global');
const toolType = ref<ToolType>(defaultToolType.value ?? 'claude-code');
const targetPath = ref('');
const projectPath = ref('');
const rememberPath = ref(false);
const loading = ref(false);
const error = ref('');
const overwriteConfirm = ref(false);
const isDragging = ref(false);
const projectPaths = ref<string[]>([]);
const showPathDropdown = ref(false);

const unlistenFns: UnlistenFn[] = [];

onMounted(async () => {
  const unlistenDrop = await listen<{ paths: string[] }>(TauriEvent.DRAG_DROP, (event) => {
    isDragging.value = false;
    if (installType.value !== 'project') return;
    const path = event.payload.paths?.[0];
    if (path) {
      projectPath.value = path;
    }
  });
  const unlistenEnter = await listen(TauriEvent.DRAG_ENTER, () => {
    if (installType.value === 'project') {
      isDragging.value = true;
    }
  });
  const unlistenLeave = await listen(TauriEvent.DRAG_LEAVE, () => {
    isDragging.value = false;
  });
  unlistenFns.push(unlistenDrop, unlistenEnter, unlistenLeave);

  try {
    projectPaths.value = await invoke<string[]>('get_project_paths');
  } catch { /* ignore */ }
});

onUnmounted(() => {
  unlistenFns.forEach(fn => fn());
});

const tools: { value: ToolType; label: string }[] = (
  Object.entries(TOOL_LABELS) as [ToolType, string][]
).map(([value, label]) => ({ value, label }));

const projectToolDir = computed(() => {
  if (toolType.value === 'custom') return '.skills';
  return (PROJECT_TOOL_DIRS[toolType.value] ?? '.skills').replace(/\\/g, pathSep);
});

watch(toolType, async (tool) => {
  if (installType.value === 'global') {
    targetPath.value = await getToolPath(tool);
  }
}, { immediate: true });

watch(installType, async (type) => {
  if (type === 'global') {
    targetPath.value = await getToolPath(toolType.value);
  } else {
    targetPath.value = '';
  }
});

const isWindows = navigator.userAgent.includes('Windows');
const pathSep = isWindows ? '\\' : '/';

const isCustomTool = computed(() => toolType.value === 'custom');

const filteredPaths = computed(() => {
  const input = projectPath.value.toLowerCase();
  if (!input) return projectPaths.value;
  return projectPaths.value.filter(p => p.toLowerCase().includes(input));
});

function selectProjectPath(path: string) {
  projectPath.value = path;
  showPathDropdown.value = false;
}

function onPathInputFocus() {
  if (filteredPaths.value.length > 0) {
    showPathDropdown.value = true;
  }
}

function onPathInputChange() {
  showPathDropdown.value = filteredPaths.value.length > 0;
}

const previewPath = computed(() => {
  if (installType.value === 'project') {
    const base = projectPath.value || '<项目路径>';
    return [base, projectToolDir.value, props.skill.id].join(pathSep);
  }
  const base = targetPath.value || '<工具技能路径>';
  return [base, props.skill.id].join(pathSep);
});

async function handleInstall() {
  error.value = '';
  loading.value = true;

  const resolvedTargetPath = installType.value === 'project'
    ? projectPath.value
    : targetPath.value;

  if (installType.value === 'project' && !projectPath.value) {
    error.value = '请输入项目路径';
    loading.value = false;
    return;
  }

  try {
    await installSkill({
      skillId: props.skill.id,
      repoUrl: props.skill.repoUrl,
      installType: installType.value,
      toolType: toolType.value,
      targetPath: resolvedTargetPath,
      overwrite: overwriteConfirm.value,
      rememberPath: rememberPath.value,
    });

    if (installType.value === 'project' && projectPath.value) {
      try {
        await invoke('add_project_path', { path: projectPath.value });
      } catch { /* ignore */ }
    }

    emit('installed', props.skill.id);
    emit('close');
  } catch (e) {
    const msg = String(e);
    if (msg.includes('JUNCTION_EXISTS')) {
      overwriteConfirm.value = false;
      error.value = '该技能已安装。是否覆盖？';
    } else {
      error.value = msg;
    }
  } finally {
    loading.value = false;
  }
}

async function handleOverwrite() {
  overwriteConfirm.value = true;
  await handleInstall();
}

async function selectFolder() {
  const selected = await open({ directory: true, title: '选择项目目录' });
  if (typeof selected === 'string') {
    projectPath.value = selected;
  }
}
</script>

<template>
  <div class="modal-overlay" @click.self="emit('close')" tabindex="0" @keydown.escape="emit('close')">
    <div class="modal">
      <h2>安装技能: {{ skill.name }}</h2>
      <p class="desc">{{ skill.description }}</p>

      <div class="section">
        <label>安装位置</label>
        <div class="radio-group">
          <label class="radio">
            <input type="radio" v-model="installType" value="global" />
            全局安装（工具配置目录）
          </label>
          <label class="radio">
            <input type="radio" v-model="installType" value="project" />
            项目安装（工具项目目录）
          </label>
        </div>
      </div>

      <div class="section">
        <label>目标工具</label>
        <select v-model="toolType">
          <option v-for="tool in tools" :key="tool.value" :value="tool.value">
            {{ tool.label }}
          </option>
        </select>
      </div>

      <div v-if="installType === 'global'" class="section">
        <label class="path-label">技能目录路径</label>
        <input v-model="targetPath" type="text" placeholder="工具技能目录路径" :disabled="!isCustomTool" />
        <label class="checkbox">
          <input type="checkbox" v-model="rememberPath" />
          记住此路径
        </label>
      </div>

      <div v-else class="section">
        <label>项目路径</label>
        <div
          class="drop-zone"
          :class="{ 'drop-zone-active': isDragging }"
          @dragover.prevent
          @drop.prevent
        >
          <div class="path-row path-row-with-dropdown">
            <div class="path-input-wrapper">
              <input
                v-model="projectPath"
                type="text"
                :placeholder="isWindows ? '例: D:\\MyProject' : '例: /home/user/my-project'"
                @focus="onPathInputFocus"
                @input="onPathInputChange"
                @blur="showPathDropdown = false"
              />
              <div v-if="showPathDropdown && filteredPaths.length > 0" class="path-dropdown">
                <div
                  v-for="p in filteredPaths"
                  :key="p"
                  class="path-dropdown-item"
                  @mousedown.prevent="selectProjectPath(p)"
                >
                  {{ p }}
                </div>
              </div>
            </div>
            <button type="button" class="browse-btn" @click="selectFolder" title="选择文件夹">
              <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/></svg>
            </button>
          </div>
          <div v-if="isDragging" class="drop-overlay">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"/><line x1="12" y1="11" x2="12" y2="17"/><polyline points="9 14 12 11 15 14"/></svg>
            <span>松开以设置项目路径</span>
          </div>
        </div>
        <span class="hint-text">可直接将项目文件夹拖拽到此窗口</span>
      </div>

      <div class="preview">
        <span>安装到:</span>
        <code>{{ previewPath }}</code>
      </div>

      <div v-if="error" class="error-box">
        <span>{{ error }}</span>
        <button v-if="error.includes('已安装')" @click="handleOverwrite" class="overwrite-btn">
          覆盖安装
        </button>
      </div>

      <div class="actions">
        <button @click="emit('close')">取消</button>
        <button class="primary" @click="handleInstall" :disabled="loading">
          {{ loading ? '安装中...' : '安装' }}
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
  width: 520px;
  max-width: 90vw;
  display: flex;
  flex-direction: column;
  gap: 16px;
}
h2 { margin: 0; color: var(--text-primary); }
.desc { color: var(--text-secondary); font-size: 0.85rem; margin: 0; }
.section { display: flex; flex-direction: column; gap: 8px; }
label { font-size: 0.85rem; font-weight: 500; color: var(--text-primary); }
.radio-group { display: flex; flex-direction: column; gap: 6px; }
.radio { font-weight: normal; display: flex; align-items: center; gap: 8px; cursor: pointer; color: var(--text-secondary); }
.path-label { margin-top: 8px; }
select, input[type="text"] {
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: 6px;
  font-size: 0.9rem;
  width: 100%;
  box-sizing: border-box;
  background: var(--bg-app);
  color: var(--text-primary);
}
select {
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath d='M2 4l4 4 4-4' fill='none' stroke='%2394a3b8' stroke-width='1.5' stroke-linecap='round' stroke-linejoin='round'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 10px center;
  padding-right: 30px;
}
select:focus, input[type="text"]:focus {
  border-color: var(--primary);
  outline: none;
}
input[type="text"]:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: var(--bg-surface-sunken);
}
.checkbox { font-weight: normal; display: flex; align-items: center; gap: 6px; cursor: pointer; color: var(--text-secondary); }
.drop-zone {
  position: relative;
}
.path-row {
  display: flex;
  gap: 8px;
}
.path-input-wrapper {
  flex: 1;
  position: relative;
}
.path-row input[type="text"] {
  flex: 1;
}
.path-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  border-top: none;
  border-radius: 0 0 6px 6px;
  max-height: 200px;
  overflow-y: auto;
  z-index: 10;
}
.path-dropdown-item {
  padding: 8px 12px;
  font-size: 0.85rem;
  color: var(--text-secondary);
  cursor: pointer;
  word-break: break-all;
  transition: background 0.1s;
}
.path-dropdown-item:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.browse-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0 10px;
  border: 1px solid var(--border);
  border-radius: 6px;
  background: var(--bg-surface);
  color: var(--text-secondary);
  cursor: pointer;
  flex-shrink: 0;
  transition: background 0.15s, color 0.15s;
}
.browse-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.drop-zone-active input[type="text"] {
  border-color: var(--primary);
  background: color-mix(in srgb, var(--primary) 5%, transparent);
}
.drop-overlay {
  position: absolute;
  inset: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  background: color-mix(in srgb, var(--primary) 10%, var(--bg-surface));
  border: 2px dashed var(--primary);
  border-radius: 6px;
  color: var(--primary);
  font-size: 0.85rem;
  font-weight: 500;
  pointer-events: none;
  z-index: 1;
}
.hint-text {
  font-size: 0.75rem;
  color: var(--text-muted);
}
.preview {
  background: var(--bg-surface-sunken);
  border: 1px solid var(--border);
  border-radius: 6px;
  padding: 10px 14px;
  font-size: 0.85rem;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.preview span { color: var(--text-secondary); }
.preview code { word-break: break-all; color: var(--text-primary); }
.error-box {
  background: var(--danger-light);
  border-radius: 6px;
  padding: 10px 14px;
  color: var(--danger);
  font-size: 0.85rem;
  display: flex;
  align-items: center;
  gap: 12px;
}
.overwrite-btn {
  padding: 4px 12px;
  background: var(--danger);
  color: #fff;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.8rem;
  white-space: nowrap;
}
.overwrite-btn:hover {
  background: var(--danger-hover);
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
