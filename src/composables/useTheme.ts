import { ref, computed, watch } from 'vue';

export type ThemeMode = 'auto' | 'light' | 'dark';

const STORAGE_KEY = 'skills-manager-theme';

const mode = ref<ThemeMode>(
  (localStorage.getItem(STORAGE_KEY) as ThemeMode) || 'auto'
);

function getSystemTheme(): 'light' | 'dark' {
  return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}

function applyTheme(theme: 'light' | 'dark') {
  const html = document.documentElement;
  html.classList.remove('theme-light', 'theme-dark');
  html.classList.add(`theme-${theme}`);
}

let mediaQuery: MediaQueryList | null = null;
let mediaHandler: ((e: MediaQueryListEvent) => void) | null = null;

function startSystemListener() {
  if (mediaQuery) return;
  mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  mediaHandler = () => {
    if (mode.value === 'auto') {
      applyTheme(getSystemTheme());
    }
  };
  mediaQuery.addEventListener('change', mediaHandler);
}

function stopSystemListener() {
  if (mediaQuery && mediaHandler) {
    mediaQuery.removeEventListener('change', mediaHandler);
    mediaQuery = null;
    mediaHandler = null;
  }
}

function resolveTheme(): 'light' | 'dark' {
  return mode.value === 'auto' ? getSystemTheme() : mode.value;
}

watch(mode, (newMode) => {
  localStorage.setItem(STORAGE_KEY, newMode);
  applyTheme(resolveTheme());
  if (newMode === 'auto') {
    startSystemListener();
  } else {
    stopSystemListener();
  }
});

// Initialize on first import
if (mode.value === 'auto') {
  startSystemListener();
}

const resolvedTheme = computed(() => resolveTheme());

// Apply theme immediately on module load
applyTheme(resolveTheme());

export function useTheme() {
  function cycleTheme() {
    const current = resolveTheme();
    const next = current === 'light' ? 'dark' : 'light';
    const system = getSystemTheme();
    mode.value = next === system ? 'auto' : next;
  }

  return {
    mode,
    resolvedTheme,
    cycleTheme,
  };
}
