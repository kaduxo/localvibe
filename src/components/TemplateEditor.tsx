import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import type { WorkspaceTemplate } from "../types/template";

interface TemplateEditorProps {
  template: WorkspaceTemplate;
  onBack: () => void;
  onSaved: () => void;
}

export function TemplateEditor({ template, onBack, onSaved }: TemplateEditorProps) {
  const [yaml, setYaml] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    invoke<string>("template_to_yaml", { template }).then(setYaml);
  }, [template]);

  const save = async () => {
    setSaving(true);
    setError(null);
    try {
      await invoke("save_template_yaml", { yaml });
      onSaved();
    } catch (err) {
      setError(String(err));
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="launcher">
      <header className="launcher-header">
        <h1>Edit Template</h1>
        <p>{template.name}</p>
      </header>

      <section className="launcher-section">
        <textarea
          value={yaml}
          onChange={(e) => setYaml(e.target.value)}
          rows={24}
          style={{ width: "100%", fontFamily: "Consolas, monospace", fontSize: 13 }}
        />
      </section>

      {error && <p style={{ color: "var(--danger)" }}>{error}</p>}

      <div className="profile-row">
        <button onClick={onBack}>Back</button>
        <button className="primary" onClick={save} disabled={saving}>
          {saving ? "Saving..." : "Save"}
        </button>
      </div>
    </div>
  );
}