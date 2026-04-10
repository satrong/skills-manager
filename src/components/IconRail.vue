<script setup lang="ts">
import { useTheme } from '../composables/useTheme';
import { Sun, Moon, Monitor, Package } from 'lucide-vue-next';

const { mode, cycleTheme } = useTheme();

const themeIcon = {
  system: Monitor,
  light: Sun,
  dark: Moon,
};

const themeTitle = {
  system: '跟随系统',
  light: '亮色模式',
  dark: '暗色模式',
};
</script>

<template>
  <div class="icon-rail">
    <div class="rail-top">
      <div class="logo" title="Skills Manager">
        <Package :size="22" />
      </div>
    </div>
    <div class="rail-bottom">
      <button
        class="theme-btn"
        @click="cycleTheme"
        :title="themeTitle[mode]"
      >
        <component :is="themeIcon[mode]" :size="18" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.icon-rail {
  width: 56px;
  min-width: 56px;
  height: 100%;
  background: var(--bg-surface-sunken);
  border-right: 1px solid var(--border);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
}
.rail-top {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
}
.logo {
  width: 36px;
  height: 36px;
  border-radius: 8px;
  background: var(--primary);
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: center;
}
.rail-bottom {
  margin-top: auto;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
}
.theme-btn {
  width: 36px;
  height: 36px;
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
.theme-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
</style>
