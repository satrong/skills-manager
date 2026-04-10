<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import type { Skill, InstallType, ToolType } from '../types';
import { TOOL_LABELS } from '../utils/toolPaths';
import { useInstall } from '../composables/useInstall';

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  close: [];
  installed: [skillId: string];
}>();

const { getToolPath, installSkill } = useInstall();

const installType = ref<InstallType>('global');
const toolType = ref<ToolType>('claude-code');
const targetPath = ref('');
const projectPath = ref('');
const rememberPath = ref(false);
const loading = ref(false);
const error = ref('');
const overwriteConfirm = ref(false);

const tools: { value: ToolType; label: string }[] = (
  Object.entries(TOOL_LABELS) as [ToolType, string][]
).map(([value, label]) => ({ value, label }));

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

const previewPath = computed(() => {
  if (installType.value === 'project') {
    const base = projectPath.value || '<项目路径>';
    return `${base}\\.skills\\${props.skill.id}`;
  }
  const base = targetPath.value || '<工具技能路径>';
  return `${base}\\${props.skill.id}`;
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
            项目安装（.skills 目录）
          </label>
        </div>
      </div>

      <div v-if="installType === 'global'" class="section">
        <label>目标工具</label>
        <select v-model="toolType">
          <option v-for="tool in tools" :key="tool.value" :value="tool.value">
            {{ tool.label }}
          </option>
        </select>
        <label class="path-label">技能目录路径</label>
        <input v-model="targetPath" type="text" placeholder="工具技能目录路径" />
        <label class="checkbox">
          <input type="checkbox" v-model="rememberPath" />
          记住此路径
        </label>
      </div>

      <div v-else class="section">
        <label>项目路径</label>
        <input
          v-model="projectPath"
          type="text"
          placeholder="例: D:\MyProject"
        />
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
.checkbox { font-weight: normal; display: flex; align-items: center; gap: 6px; cursor: pointer; color: var(--text-secondary); }
.preview {
  background: var(--primary-light);
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
