/**
 * 从仓库 URL 中解析 owner 和 repo name
 * 例如: https://github.com/anthropics/skills → { owner: 'anthropics', name: 'skills' }
 */
export function parseRepoUrl(url: string): { owner: string; name: string } {
  if (url.startsWith('local://')) {
    const path = url.replace('local://', '');
    const parts = path.replace(/\/+$/, '').split('/');
    const name = parts[parts.length - 1] || path;
    const owner = '本地目录';
    return { owner, name };
  }
  const parts = url.replace(/\/+$/, '').replace(/\.git$/, '').split('/');
  const name = parts[parts.length - 1] || url;
  const owner = parts[parts.length - 2] || '';
  return { owner, name };
}
