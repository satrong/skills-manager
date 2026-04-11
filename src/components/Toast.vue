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
        <component :is="iconMap[toast.variant]" :size="18" class="toast-icon" />
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
  gap: 10px;
  padding: 12px 16px 12px 14px;
  border-radius: 8px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-strong);
  box-shadow: 0 6px 20px rgba(0, 0, 0, 0.2);
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--text-primary);
  pointer-events: auto;
  min-width: 260px;
  max-width: 420px;
  position: relative;
  overflow: hidden;
}
.toast::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  border-radius: 8px 0 0 8px;
}
.toast-success::before { background: var(--success); }
.toast-error::before { background: var(--danger); }
.toast-info::before { background: var(--primary); }
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
  border-radius: 4px;
  transition: background 0.15s, color 0.15s;
}
.toast-close:hover {
  color: var(--text-primary);
  background: var(--bg-surface-hover);
}
.toast-success .toast-icon { color: var(--success); }
.toast-error .toast-icon { color: var(--danger); }
.toast-info .toast-icon { color: var(--primary); }

/* Transition animations */
.toast-enter-active {
  transition: all 0.35s cubic-bezier(0.21, 1.02, 0.73, 1);
}
.toast-leave-active {
  transition: all 0.2s ease;
}
.toast-enter-from {
  opacity: 0;
  transform: translateX(40px) scale(0.95);
}
.toast-leave-to {
  opacity: 0;
  transform: translateX(40px) scale(0.95);
}
</style>
