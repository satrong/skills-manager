/**
 * 从仓库 URL 中解析 owner 和 repo name
 * 例如: https://github.com/anthropics/skills → { owner: 'anthropics', name: 'skills' }
 */
export function parseRepoUrl(url: string): { owner: string; name: string } {
  const parts = url.replace(/\/+$/, '').replace(/\.git$/, '').split('/');
  const name = parts[parts.length - 1] || url;
  const owner = parts[parts.length - 2] || '';
  return { owner, name };
}
