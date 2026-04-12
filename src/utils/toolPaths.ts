import toolsData from '../../shared/tools.json'

export type ToolType = keyof typeof toolsData.tools

interface ToolDef {
  label: string
  globalPath?: string
  projectDir?: string
}

const toolsMap = toolsData.tools as Record<string, ToolDef>

export const TOOL_LABELS: Record<ToolType, string> = Object.fromEntries(
  Object.entries(toolsMap).map(([key, def]) => [key, def.label])
) as Record<ToolType, string>

export function getToolLabel(tool: ToolType, customLabel: string): string {
  return tool === 'custom' ? customLabel : (TOOL_LABELS[tool] || tool)
}

export function getProjectDir(tool: string): string | undefined {
  return toolsMap[tool]?.projectDir
}
