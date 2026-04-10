import type { ToolType } from '../types';

export const TOOL_LABELS: Record<ToolType, string> = {
  'claude-code': 'Claude Code',
  'cursor': 'Cursor',
  'codex': 'Codex',
  'opencode': 'Opencode',
  'qoder': 'Qoder',
  'kilo': 'Kilo Code',
  'custom': '自定义',
};

// 默认路径模板 (使用 %USERPROFILE% 占位符，由 Rust 后端解析)
export const DEFAULT_TOOL_PATHS: Record<Exclude<ToolType, 'custom'>, string> = {
  'claude-code': '%USERPROFILE%\\.claude\\skills',
  'cursor': '%USERPROFILE%\\.cursor\\skills',
  'codex': '%USERPROFILE%\\.codex\\skills',
  'opencode': '%USERPROFILE%\\.config\\opencode\\skills',
  'qoder': '%USERPROFILE%\\.qoder\\skills',
  'kilo': '%USERPROFILE%\\.kilocode\\skills',
};
