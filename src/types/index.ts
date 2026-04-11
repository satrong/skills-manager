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
  source: 'git' | 'local';
  skills?: Skill[];
  skillCount?: number;
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
}

export interface FavoriteEntry {
  skillId: string;
  repoUrl: string;
}

export interface AppConfig {
  repos: Omit<Repo, 'skills'>[];
  toolPaths: Partial<Record<ToolType, string>>;
  defaultToolType?: ToolType | null;
  favorites: FavoriteEntry[];
}

export interface SearchResult {
  skill: Skill;
  repoName: string;
  repoUrl: string;
}
