export type ShellType = "auto" | "pwsh" | "powershell" | "cmd" | "wsl";

export interface PaneConfig {
  id: string;
  title: string;
  shell?: ShellType;
  cwd?: string;
  command?: string;
  env?: Record<string, string>;
}

export interface LayoutConfig {
  type: "grid";
  rows: number;
  cols: number;
}

export interface WorkspaceTemplate {
  id: string;
  name: string;
  description?: string;
  layout: LayoutConfig;
  panes: PaneConfig[];
}

export interface ProfileOverrides {
  [paneId: string]: Partial<PaneConfig>;
}

export interface WorkspaceProfile {
  id: string;
  name: string;
  projectRoot: string;
  templateId: string;
  overrides?: ProfileOverrides;
}

export interface ResolvedPane extends PaneConfig {
  resolvedCwd: string;
  resolvedCommand: string;
  resolvedEnv: Record<string, string>;
  shellPath: string;
}

export interface ResolvedWorkspace {
  template: WorkspaceTemplate;
  profile?: WorkspaceProfile;
  panes: ResolvedPane[];
}

export type AppView = "launcher" | "workspace" | "editor";