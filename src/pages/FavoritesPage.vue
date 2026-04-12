<script setup lang="ts">
import { useSkillDialog } from '../composables/useSkillDialog'
import { useToast } from '../composables/useToast'
import { useInstall } from '../composables/useInstall'
import type { QuickInstallEntry } from '../components/SkillCard.vue'
import type { Skill } from '../types'
import MainContent from '../components/MainContent.vue'

const { selectedSkill } = useSkillDialog()
const { addToast } = useToast()
const { installSkill } = useInstall()

async function handleQuickInstall(skill: Skill, entry: QuickInstallEntry) {
  try {
    await installSkill({
      skillId: skill.id,
      repoUrl: skill.repoUrl,
      installType: entry.installType,
      toolType: entry.toolType,
      targetPath: entry.targetPath,
      overwrite: true,
    })
    addToast('技能安装成功', 'success')
  } catch (e) {
    addToast(String(e), 'error')
  }
}
</script>

<template>
  <MainContent
    :repo-url="null"
    view-mode="favorites"
    @install-skill="selectedSkill = $event"
    @quick-install-skill="handleQuickInstall"
  />
</template>
