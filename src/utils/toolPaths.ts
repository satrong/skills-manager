import type { ToolType } from '../types';

export const TOOL_LABELS: Record<ToolType, string> = {
  'claude-code': 'Claude Code',
  'cursor': 'Cursor',
  'codex': 'Codex',
  'opencode': 'Opencode',
  'qoder': 'Qoder',
  'kilo': 'Kilo Code',
  'custom': 'Custom',
};

// 默认全局路径模板 (使用 %USERPROFILE% 占位符，由 Rust 后端解析)
export const DEFAULT_TOOL_PATHS: Record<Exclude<ToolType, 'custom'>, string> = {
  'claude-code': '%USERPROFILE%\\.claude\\skills',
  'cursor': '%USERPROFILE%\\.cursor\\skills',
  'codex': '%USERPROFILE%\\.codex\\skills',
  'opencode': '%USERPROFILE%\\.config\\opencode\\skills',
  'qoder': '%USERPROFILE%\\.qoder\\skills',
  'kilo': '%USERPROFILE%\\.kilocode\\skills',
};

// 各工具在项目中的配置目录（相对于项目根目录）
export const PROJECT_TOOL_DIRS: Record<Exclude<ToolType, 'custom'>, string> = {
  'claude-code': '.claude\\skills',
  'cursor': '.cursor\\rules',
  'codex': '.codex\\skills',
  'opencode': '.opencode\\skills',
  'qoder': '.qoder\\skills',
  'kilo': '.kilocode\\rules',
};
