import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type {
  ResolvedWorkspace,
  WorkspaceProfile,
  WorkspaceTemplate,
} from "../types/template";
import "../styles/launcher.css";

interface LauncherProps {
  onLaunch: (workspace: ResolvedWorkspace) => void;
  onEditTemplate: (template: WorkspaceTemplate) => void;
}

export function Launcher({ onLaunch, onEditTemplate }: LauncherProps) {
  const [templates, setTemplates] = useState<WorkspaceTemplate[]>([]);
  const [profiles, setProfiles] = useState<WorkspaceProfile[]>([]);
  const [selectedProfileId, setSelectedProfileId] = useState<string>("");
  const [projectRoot, setProjectRoot] = useState("");
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    async function load() {
      try {
        const [templateList, profileList, home] = await Promise.all([
          invoke<WorkspaceTemplate[]>("list_templates"),
          invoke<WorkspaceProfile[]>("list_profiles"),
          invoke<string>("get_home_dir"),
        ]);
        setTemplates(templateList);
        setProfiles(profileList);
        setProjectRoot(home);
        if (profileList.length > 0) {
          setSelectedProfileId(profileList[0].id);
          setProjectRoot(profileList[0].projectRoot);
        }
      } catch (err) {
        setError(String(err));
      } finally {
        setLoading(false);
      }
    }
    load();
  }, []);

  const handleProfileChange = (profileId: string) => {
    setSelectedProfileId(profileId);
    const profile = profiles.find((p) => p.id === profileId);
    if (profile) setProjectRoot(profile.projectRoot);
  };

  const launch = async (templateId: string) => {
    try {
      setError(null);
      const workspace = await invoke<ResolvedWorkspace>("resolve_workspace", {
        templateId,
        profileId: selectedProfileId || null,
        projectRoot: projectRoot || null,
      });
      onLaunch(workspace);
    } catch (err) {
      setError(String(err));
    }
  };

  if (loading) {
    return <div className="launcher">Loading templates...</div>;
  }

  return (
    <div className="launcher">
      <header className="launcher-header">
        <h1>Local Vibe</h1>
        <p>Launch pre-configured terminal workspaces in one click.</p>
      </header>

      <section className="launcher-section">
        <h2>Project</h2>
        <div className="profile-row">
          <label htmlFor="profile-select">Profile</label>
          <select
            id="profile-select"
            value={selectedProfileId}
            onChange={(e) => handleProfileChange(e.target.value)}
          >
            <option value="">None (use path below)</option>
            {profiles.map((profile) => (
              <option key={profile.id} value={profile.id}>
                {profile.name}
              </option>
            ))}
          </select>
          <input
            className="project-root-input"
            type="text"
            placeholder="Project root path"
            value={projectRoot}
            onChange={(e) => setProjectRoot(e.target.value)}
          />
        </div>
      </section>

      {error && (
        <section className="launcher-section">
          <p style={{ color: "var(--danger)" }}>{error}</p>
        </section>
      )}

      <section className="launcher-section">
        <h2>Templates</h2>
        <div className="template-grid">
          {templates.map((template) => (
            <article key={template.id} className="template-card">
              <h3>{template.name}</h3>
              <p>{template.description ?? "No description"}</p>
              <div className="template-card-meta">
                {template.layout.rows}×{template.layout.cols} ·{" "}
                {template.panes.length} panes
              </div>
              <div className="template-card-actions">
                <button
                  className="primary"
                  onClick={() => launch(template.id)}
                >
                  Launch
                </button>
                <button onClick={() => onEditTemplate(template)}>Edit</button>
              </div>
            </article>
          ))}
        </div>
      </section>
    </div>
  );
}