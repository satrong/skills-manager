<script setup lang="ts">
import type { Skill } from '../types';

const props = defineProps<{
  skill: Skill;
}>();

const emit = defineEmits<{
  install: [skill: Skill];
}>();
</script>

<template>
  <div class="skill-card" @click="emit('install', props.skill)">
    <div class="skill-name">{{ skill.name }}</div>
    <div class="skill-description">{{ skill.description }}</div>
    <div class="skill-meta">
      <span v-if="skill.version" class="skill-version">v{{ skill.version }}</span>
      <span v-if="skill.author" class="skill-author">{{ skill.author }}</span>
    </div>
    <div class="skill-tags" v-if="skill.tags?.length">
      <span v-for="tag in skill.tags" :key="tag" class="tag">{{ tag }}</span>
    </div>
    <div class="card-footer">
      <button class="install-btn" @click.stop="emit('install', props.skill)">
        <span>安装</span>
      </button>
    </div>
  </div>
</template>

<style scoped>
.skill-card {
  border: 1px solid var(--border);
  border-radius: 8px;
  padding: 14px;
  cursor: pointer;
  background: var(--bg-surface);
  display: flex;
  flex-direction: column;
  gap: 6px;
  transition: border-color 0.15s, box-shadow 0.15s;
}
.skill-card:hover {
  border-color: var(--primary);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.08);
}
.skill-name {
  font-weight: 600;
  font-size: 0.9rem;
  color: var(--text-primary);
}
.skill-description {
  font-size: 0.8rem;
  color: var(--text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.skill-meta {
  font-size: 0.7rem;
  color: var(--text-muted);
  display: flex;
  gap: 8px;
}
.skill-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}
.tag {
  background: var(--primary-light);
  color: var(--primary);
  border-radius: 4px;
  padding: 1px 6px;
  font-size: 0.7rem;
}
.card-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: auto;
}
.install-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 5px 12px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 5px;
  cursor: pointer;
  font-size: 0.8rem;
  transition: all 0.15s;
}
.install-btn:hover {
  background: var(--primary);
  color: #fff;
  border-color: var(--primary);
}
</style>
