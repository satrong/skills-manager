<script setup lang="ts">
import { watch, onMounted } from 'vue'
import { useToast } from './composables/useToast'
import { useSkillDialog } from './composables/useSkillDialog'
import { useSettings } from './composables/useSettings'
import { useFavorites } from './composables/useFavorites'
import { useRepos } from './composables/useRepos'
import IconRail from './components/IconRail.vue'
import SkillDialog from './components/SkillDialog.vue'
import Toast from './components/Toast.vue'

const { addToast } = useToast()
const { selectedSkill } = useSkillDialog()
const { loadSettings } = useSettings()
const { loadFavorites } = useFavorites()
const { error: reposError } = useRepos()

watch(reposError, (err) => {
  if (err) addToast(err, 'error')
})

onMounted(async () => {
  await Promise.all([loadSettings(), loadFavorites()])
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
    @installed="addToast('技能安装成功', 'success')"
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
