import { ref } from 'vue';

export interface ToastItem {
  id: number;
  message: string;
  variant: 'success' | 'error' | 'info';
}

const toasts = ref<ToastItem[]>([]);
let nextId = 0;

function removeToast(id: number) {
  toasts.value = toasts.value.filter(t => t.id !== id);
}

export function useToast() {
  function addToast(message: string, variant: ToastItem['variant'] = 'info') {
    const id = nextId++;
    toasts.value.push({ id, message, variant });
    // Max 3: remove oldest if exceeded
    if (toasts.value.length > 3) {
      toasts.value.shift();
    }
    // Auto-dismiss after 4 seconds
    setTimeout(() => removeToast(id), 4000);
  }

  return {
    toasts,
    addToast,
    removeToast,
  };
}
