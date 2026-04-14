<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import { useToast } from './composables/useToast'
import { useSkillDialog } from './composables/useSkillDialog'
import { useSettings } from './composables/useSettings'
import { useFavorites } from './composables/useFavorites'
import { useRepos } from './composables/useRepos'
import { useUpdate } from './composables/useUpdate'
import { useI18n } from './i18n'
import IconRail from './components/IconRail.vue'
import SkillDialog from './components/SkillDialog.vue'
import UpdateDialog from './components/UpdateDialog.vue'
import Toast from './components/Toast.vue'

const { addToast } = useToast()
const { selectedSkill } = useSkillDialog()
const { loadSettings } = useSettings()
const { loadFavorites } = useFavorites()
const { error: reposError } = useRepos()
const { updateAvailable, checkForUpdate } = useUpdate()
const { t } = useI18n()

const showUpdateDialog = ref(false)

watch(reposError, (err) => {
  if (err) addToast(err, 'error')
})

watch(updateAvailable, (available) => {
  if (available) {
    showUpdateDialog.value = true
  }
})

onMounted(async () => {
  await Promise.all([loadSettings(), loadFavorites()])
  try {
    await checkForUpdate()
  } catch {
    // 自动检查失败时静默跳过
  }
})
</script>

<template>
  <div class="app-layout">
    <IconRail />
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" />
      </keep-alive>
    </router-view>
  </div>

  <SkillDialog
    v-if="selectedSkill"
    :skill="selectedSkill"
    @close="selectedSkill = null"
    @installed="addToast(t('app.installSuccess'), 'success')"
  />

  <UpdateDialog
    v-if="showUpdateDialog"
    @close="showUpdateDialog = false"
  />

  <Toast />
</template>

<style scoped>
.app-layout {
  display: flex;
  height: 100vh;
  overflow: hidden;
}
</style>
