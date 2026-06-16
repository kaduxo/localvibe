mod schema;

pub use schema::*;

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("template not found: {0}")]
    NotFound(String),
    #[error("invalid template: {0}")]
    Invalid(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("yaml error: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

pub fn resolve_variables(value: &str, vars: &HashMap<String, String>) -> String {
    let mut result = value.to_string();
    for (key, val) in vars {
        result = result.replace(&format!("{{{{{key}}}}}"), val);
    }
    result
}

pub fn build_variables(home: &str, project_root: &str) -> HashMap<String, String> {
    HashMap::from([
        ("home".to_string(), home.to_string()),
        ("projectRoot".to_string(), project_root.to_string()),
    ])
}

pub fn detect_shell(shell: &str) -> String {
    match shell {
        "pwsh" => find_executable(&["pwsh.exe", "pwsh"]),
        "powershell" => find_executable(&["powershell.exe"]),
        "cmd" => "cmd.exe".to_string(),
        "wsl" => find_executable(&["wsl.exe"]),
        _ => find_default_shell(),
    }
}

fn find_default_shell() -> String {
    if let Some(pwsh) = find_optional_executable(&["pwsh.exe", "pwsh"]) {
        return pwsh;
    }
    if Path::new("C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe").exists() {
        return "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe".to_string();
    }
    "cmd.exe".to_string()
}

fn find_executable(candidates: &[&str]) -> String {
    find_optional_executable(candidates).unwrap_or_else(|| candidates[0].to_string())
}

fn find_optional_executable(candidates: &[&str]) -> Option<String> {
    for candidate in candidates {
        if Path::new(candidate).is_absolute() && Path::new(candidate).exists() {
            return Some(candidate.to_string());
        }
        if let Ok(path) = which::which(candidate) {
            return Some(path.to_string_lossy().to_string());
        }
    }
    None
}

pub fn load_template_from_file(path: &Path) -> Result<WorkspaceTemplate, TemplateError> {
    let content = std::fs::read_to_string(path)?;
    let template: WorkspaceTemplate = serde_yaml::from_str(&content)?;
    validate_template(&template)?;
    Ok(template)
}

pub fn load_profile_from_file(path: &Path) -> Result<WorkspaceProfile, TemplateError> {
    let content = std::fs::read_to_string(path)?;
    let profile: WorkspaceProfile = serde_yaml::from_str(&content)?;
    Ok(profile)
}

pub fn template_to_yaml(template: &WorkspaceTemplate) -> Result<String, TemplateError> {
    Ok(serde_yaml::to_string(template)?)
}

pub fn template_from_yaml(yaml: &str) -> Result<WorkspaceTemplate, TemplateError> {
    let template: WorkspaceTemplate = serde_yaml::from_str(yaml)?;
    validate_template(&template)?;
    Ok(template)
}

fn validate_template(template: &WorkspaceTemplate) -> Result<(), TemplateError> {
    if template.id.trim().is_empty() {
        return Err(TemplateError::Invalid("template id is required".into()));
    }
    if template.panes.is_empty() {
        return Err(TemplateError::Invalid("template must have at least one pane".into()));
    }
    let expected = (template.layout.rows * template.layout.cols) as usize;
    if template.panes.len() > expected {
        return Err(TemplateError::Invalid(format!(
            "template has {} panes but layout only supports {}",
            template.panes.len(),
            expected
        )));
    }
    Ok(())
}

pub fn resolve_workspace(
    template: WorkspaceTemplate,
    profile: Option<WorkspaceProfile>,
    project_root: &str,
    home: &str,
) -> Result<ResolvedWorkspace, TemplateError> {
    let vars = build_variables(home, project_root);
    let overrides = profile
        .as_ref()
        .map(|p| p.overrides.clone())
        .unwrap_or_default();

    let panes = template
        .panes
        .iter()
        .map(|pane| {
            let override_cfg = overrides.get(&pane.id);
            let title = override_cfg
                .and_then(|o| o.title.clone())
                .unwrap_or_else(|| pane.title.clone());
            let shell = override_cfg
                .and_then(|o| o.shell.clone())
                .unwrap_or_else(|| pane.shell.clone());
            let cwd = override_cfg
                .and_then(|o| o.cwd.clone())
                .or_else(|| pane.cwd.clone());
            let command = override_cfg
                .and_then(|o| o.command.clone())
                .or_else(|| pane.command.clone());

            let mut env = pane.env.clone();
            if let Some(override_env) = override_cfg.and_then(|o| o.env.clone()) {
                env.extend(override_env);
            }

            let resolved_cwd = cwd
                .map(|value| resolve_variables(&value, &vars))
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| project_root.to_string());

            let resolved_command = command
                .map(|value| resolve_variables(&value, &vars))
                .unwrap_or_default();

            let resolved_env: HashMap<String, String> = env
                .into_iter()
                .map(|(k, v)| (k, resolve_variables(&v, &vars)))
                .collect();

            let shell_path = detect_shell(&shell);

            ResolvedPane {
                id: pane.id.clone(),
                title,
                shell,
                cwd: pane.cwd.clone(),
                command: pane.command.clone(),
                env: pane.env.clone(),
                resolved_cwd,
                resolved_command,
                resolved_env,
                shell_path,
            }
        })
        .collect();

    Ok(ResolvedWorkspace {
        template,
        profile,
        panes,
    })
}

pub fn list_yaml_files(dir: &Path) -> Result<Vec<PathBuf>, TemplateError> {
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut files = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("yaml") {
            files.push(path);
        }
    }
    files.sort();
    Ok(files)
}