use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LayoutConfig {
    #[serde(rename = "type")]
    pub layout_type: String,
    pub rows: u32,
    pub cols: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaneConfig {
    pub id: String,
    pub title: String,
    #[serde(default = "default_shell")]
    pub shell: String,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

fn default_shell() -> String {
    "auto".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceTemplate {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub layout: LayoutConfig,
    pub panes: Vec<PaneConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceProfile {
    pub id: String,
    pub name: String,
    pub project_root: String,
    pub template_id: String,
    #[serde(default)]
    pub overrides: HashMap<String, PaneOverride>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PaneOverride {
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub shell: Option<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub env: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedPane {
    pub id: String,
    pub title: String,
    pub shell: String,
    pub cwd: Option<String>,
    pub command: Option<String>,
    pub env: HashMap<String, String>,
    pub resolved_cwd: String,
    pub resolved_command: String,
    pub resolved_env: HashMap<String, String>,
    pub shell_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedWorkspace {
    pub template: WorkspaceTemplate,
    pub profile: Option<WorkspaceProfile>,
    pub panes: Vec<ResolvedPane>,
}