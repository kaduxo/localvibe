import { useState } from "react";
import { Launcher } from "./components/Launcher";
import { TemplateEditor } from "./components/TemplateEditor";
import { WorkspaceLayout } from "./components/WorkspaceLayout";
import type {
  AppView,
  ResolvedWorkspace,
  WorkspaceTemplate,
} from "./types/template";
import "./styles/workspace.css";

function App() {
  const [view, setView] = useState<AppView>("launcher");
  const [workspace, setWorkspace] = useState<ResolvedWorkspace | null>(null);
  const [editingTemplate, setEditingTemplate] =
    useState<WorkspaceTemplate | null>(null);

  if (view === "workspace" && workspace) {
    return (
      <div className="workspace">
        <div className="workspace-toolbar">
          <button onClick={() => setView("launcher")}>← Back</button>
          <span className="workspace-title">
            {workspace.template.name}
            {workspace.profile ? ` · ${workspace.profile.name}` : ""}
          </span>
        </div>
        <WorkspaceLayout
          layout={workspace.template.layout}
          panes={workspace.panes}
        />
      </div>
    );
  }

  if (view === "editor" && editingTemplate) {
    return (
      <TemplateEditor
        template={editingTemplate}
        onBack={() => setView("launcher")}
        onSaved={() => setView("launcher")}
      />
    );
  }

  return (
    <Launcher
      onLaunch={(resolved) => {
        setWorkspace(resolved);
        setView("workspace");
      }}
      onEditTemplate={(template) => {
        setEditingTemplate(template);
        setView("editor");
      }}
    />
  );
}

export default App;