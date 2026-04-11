export type ToolType =
  | 'claude-code'
  | 'cursor'
  | 'codex'
  | 'opencode'
  | 'qoder'
  | 'kilo'
  | 'custom';

export type InstallType = 'global' | 'project';

export interface Repo {
  url: string;
  localPath: string;
  name: string;
  lastUpdate: string;
  skills?: Skill[];
}

export interface Skill {
  id: string;
  name: string;
  description: string;
  repoUrl: string;
  sourcePath: string;
  version?: string;
  author?: string;
  tags?: string[];
}

export interface InstallRequest {
  skillId: string;
  repoUrl: string;
  installType: InstallType;
  toolType: ToolType;
  targetPath: string;
  rememberPath?: boolean;
}

export interface AppConfig {
  repos: Omit<Repo, 'skills'>[];
  toolPaths: Partial<Record<ToolType, string>>;
  defaultToolType?: ToolType | null;
}
