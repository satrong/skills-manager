<script setup lang="ts">
import { computed } from 'vue';
import type { Skill, ToolType } from '../types';

export interface QuickInstallEntry {
  label: string;
  installType: 'global' | 'project';
  toolType: ToolType;
  targetPath: string;
  header?: boolean;
}

const props = defineProps<{
  skill: Skill;
  quickInstallEntries?: QuickInstallEntry[];
  openDropdown?: boolean;
}>();

const emit = defineEmits<{
  install: [skill: Skill];
  quickInstall: [skill: Skill, entry: QuickInstallEntry];
  toggleDropdown: [];
}>();

const hasEntries = computed(() =>
  props.quickInstallEntries && props.quickInstallEntries.length > 0
);

function handleToggleDropdown(e: Event) {
  e.stopPropagation();
  emit('toggleDropdown');
}

function handleQuickInstall(entry: QuickInstallEntry) {
  emit('toggleDropdown');
  emit('quickInstall', props.skill, entry);
}
</script>

<template>
  <div class="skill-card" @click="emit('toggleDropdown')">
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
      <div class="install-btn-group" :class="{ active: openDropdown }">
        <button class="install-btn" @click.stop="emit('install', props.skill)">
          <span>安装</span>
        </button>
        <button
          v-if="hasEntries"
          class="install-btn-arrow"
          @click.stop="handleToggleDropdown"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="6 9 12 15 18 9"/></svg>
        </button>
        <div v-if="openDropdown" class="install-dropdown">
          <template v-for="(entry, idx) in quickInstallEntries" :key="idx">
            <div v-if="entry.header" class="dropdown-header">
              <span class="item-label">{{ entry.label }}</span>
            </div>
            <button
              v-else
              class="dropdown-item"
              @click.stop="handleQuickInstall(entry)"
            >
              <span class="item-label">{{ entry.label }}</span>
              <span class="item-path">{{ entry.targetPath }}</span>
            </button>
          </template>
        </div>
      </div>
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
  font-size: 1rem;
  color: var(--text-primary);
}
.skill-description {
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.4;
  display: -webkit-box;
  -webkit-line-clamp: 3;
  -webkit-box-orient: vertical;
  overflow: hidden;
}
.skill-meta {
  font-size: 0.75rem;
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
  font-size: 0.75rem;
}
.card-footer {
  display: flex;
  justify-content: flex-end;
  margin-top: auto;
}
.install-btn-group {
  position: relative;
  display: flex;
}
.install-btn-group.active {
  z-index: 10;
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
  font-size: 0.75rem;
  transition: all 0.15s;
}
.install-btn:hover {
  background: var(--primary);
  color: #fff;
  border-color: var(--primary);
}
.install-btn-group:has(.install-btn-arrow) .install-btn {
  border-radius: 5px 0 0 5px;
  border-right: none;
}
.install-btn-group:has(.install-btn-arrow) .install-btn:hover + .install-btn-arrow {
  border-left-color: var(--primary);
}
.install-btn-arrow {
  display: flex;
  align-items: center;
  padding: 5px 6px;
  background: transparent;
  color: var(--text-secondary);
  border: 1px solid var(--border);
  border-radius: 0 5px 5px 0;
  cursor: pointer;
  font-size: 0.75rem;
  transition: all 0.15s;
}
.install-btn-arrow:hover {
  background: var(--primary);
  color: #fff;
  border-color: var(--primary);
}
.install-dropdown {
  position: absolute;
  top: calc(100% + 4px);
  right: 0;
  width: max-content;
  max-width: 540px;
  background: var(--bg-elevated, #fff);
  border: 1px solid var(--border-strong, #c0c8d4);
  border-radius: 8px;
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.18);
  z-index: 50;
  overflow: hidden;
}
.dropdown-header {
  padding: 6px 12px;
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-muted);
  cursor: default;
  user-select: none;
}
.dropdown-item + .dropdown-header,
.dropdown-header + .dropdown-item {
  border-top: 1px solid var(--border);
}
.dropdown-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
  width: 100%;
  padding: 8px 12px;
  border: none;
  border-radius: 0;
  background: transparent;
  color: var(--text-primary);
  cursor: pointer;
  font-size: 0.8rem;
  text-align: left;
  transition: background 0.1s;
}
.dropdown-item:hover {
  background: var(--bg-surface-hover);
}
.item-label {
  font-weight: 500;
  font-size: 0.8rem;
}
.item-path {
  font-size: 0.7rem;
  color: var(--text-secondary);
  word-break: break-all;
  line-height: 1.3;
}
</style>
