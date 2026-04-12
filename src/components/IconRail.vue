<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router'
import { useTheme } from '../composables/useTheme'
import { Sun, Moon, Settings, Star } from 'lucide-vue-next'

const route = useRoute()
const router = useRouter()
const { resolvedTheme, cycleTheme } = useTheme()

const themeIcon = {
  light: Sun,
  dark: Moon,
}

const themeTitle = {
  light: '亮色模式',
  dark: '暗色模式',
}
</script>

<template>
  <div class="icon-rail">
    <div class="rail-top">
      <div class="logo" title="Skills Manager">
        <svg width="180" height="180" viewBox="0 0 180 180" fill="none" xmlns="http://www.w3.org/2000/svg">
        <path d="M87.7313 46.5628C89.0796 43.1457 93.9204 43.1457 95.2688 46.5628L99.0794 56.2359C100.301 59.333 102.146 62.146 104.5 64.5002C106.854 66.8543 109.667 68.6993 112.764 69.9206L122.437 73.7313C125.854 75.0796 125.854 79.9204 122.437 81.2688L112.764 85.0794C109.667 86.3007 106.854 88.1457 104.5 90.4998C102.146 92.854 100.301 95.667 99.0794 98.7641L95.2688 108.437C93.9204 111.854 89.0796 111.854 87.7313 108.437L83.9206 98.7641C82.6993 95.667 80.8543 92.854 78.5002 90.4998C76.146 88.1457 73.333 86.3007 70.2359 85.0794L60.5628 81.2688C57.1457 79.9204 57.1457 75.0796 60.5628 73.7313L70.2359 69.9206C73.333 68.6993 76.146 66.8543 78.5002 64.5002C80.8543 62.146 82.6993 59.333 83.9206 56.2359L87.7313 46.5628Z" fill="currentColor" />
        <path d="M123.094 32.9563C123.597 31.6812 125.403 31.6812 125.906 32.9563L127.328 36.5656C127.784 37.7213 128.472 38.7709 129.351 39.6493C130.229 40.5277 131.279 41.2162 132.434 41.6719L136.044 43.0938C137.319 43.5969 137.319 45.4031 136.044 45.9063L132.434 47.3281C131.279 47.7838 130.229 48.4723 129.351 49.3507C128.472 50.2291 127.784 51.2787 127.328 52.4344L125.906 56.0438C125.403 57.3188 123.597 57.3188 123.094 56.0438L121.672 52.4344C121.216 51.2787 120.528 50.2291 119.649 49.3507C118.771 48.4723 117.721 47.7838 116.566 47.3281L112.956 45.9063C111.681 45.4031 111.681 43.5969 112.956 43.0938L116.566 41.6719C117.721 41.2162 118.771 40.5277 119.649 39.6493C120.528 38.7709 121.216 37.7213 121.672 36.5656L123.094 32.9563Z" fill="currentColor" />
        <rect x="41" y="130" width="102" height="14" rx="6" fill="currentColor" />
        </svg>
      </div>
      <div class="nav-divider"></div>
      <button
        class="nav-btn"
        :class="{ active: route.name === 'skills' }"
        title="技能列表"
        @click="router.push({ name: 'skills' })"
      >
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/></svg>
      </button>
      <button
        class="nav-btn"
        :class="{ active: route.name === 'favorites' }"
        title="收藏列表"
        @click="router.push({ name: 'favorites' })"
      >
        <Star :size="18" />
      </button>
    </div>
    <div class="rail-bottom">
      <button
        class="theme-btn"
        @click="cycleTheme"
        :title="themeTitle[resolvedTheme]"
      >
        <component :is="themeIcon[resolvedTheme]" :size="18" />
      </button>
      <button
        class="theme-btn"
        :class="{ active: route.name === 'settings' }"
        title="设置"
        @click="router.push({ name: 'settings' })"
      >
        <Settings :size="18" />
      </button>
    </div>
  </div>
</template>

<style scoped>
.icon-rail {
  width: 56px;
  min-width: 56px;
  height: 100%;
  background: var(--rail-bg);

  box-shadow: var(--panel-shadow);
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 12px 0;
  position: relative;
  z-index: 2;
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
  display: flex;
  color: #FF7A27;
  align-items: center;
  justify-content: center;

  overflow: hidden;
}
.logo svg {
  width: 100%;
  height: 100%;
}
.nav-divider {
  width: 20px;
  height: 1px;
  background: var(--border);
  margin: 4px 0;
}
.nav-btn {
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
.nav-btn:hover {
  background: var(--bg-surface-hover);
  color: var(--text-primary);
}
.nav-btn.active {
  background: var(--primary-light);
  color: var(--primary);
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
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.08);
}
.theme-btn.active {
  background: var(--primary-light);
  color: var(--primary);
}
</style>
