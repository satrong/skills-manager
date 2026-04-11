import { invoke } from '@tauri-apps/api/core';
import type { InstallType, ToolType } from '../types';

export function useInstall() {
  async function getToolPath(toolType: ToolType): Promise<string> {
    return await invoke<string>('get_tool_path', { toolType });
  }

  async function setToolPath(toolType: ToolType, path: string): Promise<void> {
    await invoke('set_tool_path', { toolType, path });
  }

  async function checkJunctionExists(linkPath: string): Promise<boolean> {
    return await invoke<boolean>('check_junction_exists', { linkPath });
  }

  async function installSkill(params: {
    skillId: string;
    repoUrl: string;
    installType: InstallType;
    toolType: ToolType;
    targetPath: string;
    overwrite?: boolean;
  }): Promise<void> {
    await invoke('install_skill', {
      skillId: params.skillId,
      repoUrl: params.repoUrl,
      installType: params.installType,
      toolType: params.toolType,
      targetPath: params.targetPath,
      overwrite: params.overwrite ?? false,
    });
  }

  async function uninstallSkill(linkPath: string): Promise<void> {
    await invoke('uninstall_skill', { linkPath });
  }

  return {
    getToolPath,
    setToolPath,
    checkJunctionExists,
    installSkill,
    uninstallSkill,
  };
}
