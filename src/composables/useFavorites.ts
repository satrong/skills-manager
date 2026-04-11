import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import type { FavoriteEntry } from '../types'

const favorites = ref<FavoriteEntry[]>([])
let loaded = false

async function loadFavorites() {
  if (loaded) return
  loaded = true
  try {
    favorites.value = await invoke<FavoriteEntry[]>('list_favorites')
  } catch {
    favorites.value = []
  }
}

function isFavorite(skillId: string, repoUrl: string): boolean {
  return favorites.value.some(f => f.skillId === skillId && f.repoUrl === repoUrl)
}

async function addFavorite(skillId: string, repoUrl: string) {
  await invoke('add_favorite', { skillId, repoUrl })
  if (!isFavorite(skillId, repoUrl)) {
    favorites.value.push({ skillId, repoUrl })
  }
}

async function removeFavorite(skillId: string, repoUrl: string) {
  await invoke('remove_favorite', { skillId, repoUrl })
  favorites.value = favorites.value.filter(
    f => !(f.skillId === skillId && f.repoUrl === repoUrl)
  )
}

async function toggleFavorite(skillId: string, repoUrl: string) {
  if (isFavorite(skillId, repoUrl)) {
    await removeFavorite(skillId, repoUrl)
  } else {
    await addFavorite(skillId, repoUrl)
  }
}

export function useFavorites() {
  return {
    favorites,
    loadFavorites,
    isFavorite,
    addFavorite,
    removeFavorite,
    toggleFavorite,
  }
}
