import { ref } from 'vue'
import type { Skill } from '../types'

const selectedSkill = ref<Skill | null>(null)

export function useSkillDialog() {
  return { selectedSkill }
}
