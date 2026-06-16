mod store;
mod templates;

use store::Store;
use templates::{
    resolve_workspace as build_resolved_workspace, template_to_yaml, ResolvedWorkspace,
    WorkspaceProfile, WorkspaceTemplate,
};
#[tauri::command]
fn get_home_dir(state: tauri::State<'_, Store>) -> Result<String, String> {
    Ok(state.home_dir())
}

#[tauri::command]
fn list_templates(state: tauri::State<'_, Store>) -> Result<Vec<WorkspaceTemplate>, String> {
    state.list_templates().map_err(|e| e.to_string())
}

#[tauri::command]
fn list_profiles(state: tauri::State<'_, Store>) -> Result<Vec<WorkspaceProfile>, String> {
    state.list_profiles().map_err(|e| e.to_string())
}

#[tauri::command]
fn resolve_workspace(
    template_id: String,
    profile_id: Option<String>,
    project_root: Option<String>,
    state: tauri::State<'_, Store>,
) -> Result<ResolvedWorkspace, String> {
    let template = state
        .load_template(&template_id)
        .map_err(|e| e.to_string())?;

    let profile = match profile_id {
        Some(id) if !id.is_empty() => Some(
            state
                .load_profile(&id)
                .map_err(|e| e.to_string())?,
        ),
        _ => None,
    };

    let home = state.home_dir();
    let vars = templates::build_variables(&home, &home);
    let root = project_root
        .filter(|value| !value.trim().is_empty())
        .or_else(|| {
            profile.as_ref().map(|p| {
                templates::resolve_variables(&p.project_root, &vars)
            })
        })
        .unwrap_or(home.clone());

    build_resolved_workspace(template, profile, &root, &home).map_err(|e| e.to_string())
}

#[tauri::command]
fn template_to_yaml_cmd(template: WorkspaceTemplate) -> Result<String, String> {
    template_to_yaml(&template).map_err(|e| e.to_string())
}

#[tauri::command]
fn save_template_yaml(yaml: String, state: tauri::State<'_, Store>) -> Result<(), String> {
    state.save_template_yaml(&yaml).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let store = Store::new().expect("failed to initialize local-vibe store");

    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_pty::init())
        .manage(store)
        .invoke_handler(tauri::generate_handler![
            get_home_dir,
            list_templates,
            list_profiles,
            resolve_workspace,
            template_to_yaml_cmd,
            save_template_yaml,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}