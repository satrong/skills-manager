import { ref, computed, watch } from 'vue'
import zhCN from './zh-CN'
import en from './en'
import type { TranslationKeys } from './zh-CN'

export type Locale = 'auto' | 'zh-CN' | 'en'

const STORAGE_KEY = 'skills-manager-locale'

type Messages = Record<TranslationKeys, string>

const messages: Record<string, Messages> = {
  'zh-CN': zhCN as unknown as Messages,
  'en': en as unknown as Messages,
}

function detectSystemLocale(): string {
  const lang = navigator.language
  if (lang.startsWith('zh')) return 'zh-CN'
  return 'en'
}

const savedLocale = (localStorage.getItem(STORAGE_KEY) as Locale) || 'auto'
const locale = ref<Locale>(savedLocale)

const resolvedLocale = computed(() =>
  locale.value === 'auto' ? detectSystemLocale() : locale.value
)

function t(key: TranslationKeys, params?: Record<string, string | number>): string {
  const dict = messages[resolvedLocale.value] ?? messages['en']
  let text = dict[key] ?? key
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replace(`{${k}}`, String(v))
    }
  }
  return text
}

watch(locale, (val) => {
  localStorage.setItem(STORAGE_KEY, val)
})

export function useI18n() {
  return {
    locale,
    t,
    resolvedLocale,
  }
}
