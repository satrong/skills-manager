<script setup lang="ts">
import { useToast } from '../composables/useToast';
import { CheckCircle, AlertCircle, Info, X } from 'lucide-vue-next';

const { toasts, removeToast } = useToast();

const iconMap = {
  success: CheckCircle,
  error: AlertCircle,
  info: Info,
};

const variantClass = {
  success: 'toast-success',
  error: 'toast-error',
  info: 'toast-info',
};
</script>

<template>
  <div class="toast-container" v-if="toasts.length">
    <TransitionGroup name="toast">
      <div
        v-for="toast in toasts"
        :key="toast.id"
        :class="['toast', variantClass[toast.variant]]"
      >
        <component :is="iconMap[toast.variant]" :size="16" class="toast-icon" />
        <span class="toast-message">{{ toast.message }}</span>
        <button class="toast-close" @click="removeToast(toast.id)">
          <X :size="14" />
        </button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  bottom: 16px;
  right: 16px;
  z-index: 200;
  display: flex;
  flex-direction: column;
  gap: 8px;
  pointer-events: none;
}
.toast {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 14px;
  border-radius: 8px;
  background: var(--bg-surface);
  border: 1px solid var(--border);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  font-size: 0.85rem;
  color: var(--text-primary);
  pointer-events: auto;
  min-width: 240px;
  max-width: 400px;
}
.toast-icon { flex-shrink: 0; }
.toast-message { flex: 1; }
.toast-close {
  flex-shrink: 0;
  background: none;
  border: none;
  color: var(--text-muted);
  cursor: pointer;
  padding: 2px;
  display: flex;
  align-items: center;
}
.toast-close:hover { color: var(--text-primary); }
.toast-success .toast-icon { color: var(--success); }
.toast-error .toast-icon { color: var(--danger); }
.toast-info .toast-icon { color: var(--primary); }

/* Transition animations */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.2s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px);
}
</style>
