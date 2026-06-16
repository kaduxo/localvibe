use crate::templates::{
    load_profile_from_file, load_template_from_file, list_yaml_files, template_from_yaml,
    template_to_yaml, TemplateError, WorkspaceProfile, WorkspaceTemplate,
};
use std::fs;
use std::path::{Path, PathBuf};

pub struct Store {
    root: PathBuf,
    bundled_templates: PathBuf,
    bundled_profiles: PathBuf,
}

impl Store {
    pub fn new() -> Result<Self, TemplateError> {
        let root = dirs::home_dir()
            .ok_or_else(|| TemplateError::Invalid("could not resolve home directory".into()))?
            .join(".local-vibe");

        fs::create_dir_all(root.join("templates"))?;
        fs::create_dir_all(root.join("profiles"))?;

        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let project_root = manifest_dir
            .parent()
            .expect("valid manifest dir");

        let bundled_templates = project_root.join("templates");
        let bundled_profiles = project_root.join("profiles");

        let store = Self {
            root,
            bundled_templates,
            bundled_profiles,
        };
        store.seed_dir(&store.bundled_templates, &store.templates_dir())?;
        store.seed_dir(&store.bundled_profiles, &store.profiles_dir())?;
        Ok(store)
    }

    fn seed_dir(&self, source: &Path, target: &Path) -> Result<(), TemplateError> {
        if !source.exists() {
            return Ok(());
        }

        for bundled in list_yaml_files(source)? {
            let file_name = bundled.file_name().expect("yaml file name");
            let destination = target.join(file_name);
            if !destination.exists() {
                fs::copy(&bundled, &destination)?;
            }
        }
        Ok(())
    }

    pub fn home_dir(&self) -> String {
        dirs::home_dir()
            .map(|path| path.to_string_lossy().to_string())
            .unwrap_or_else(|| "C:\\".to_string())
    }

    pub fn templates_dir(&self) -> PathBuf {
        self.root.join("templates")
    }

    pub fn profiles_dir(&self) -> PathBuf {
        self.root.join("profiles")
    }

    pub fn list_templates(&self) -> Result<Vec<WorkspaceTemplate>, TemplateError> {
        let mut templates = vec![];
        for path in list_yaml_files(&self.templates_dir())? {
            templates.push(load_template_from_file(&path)?);
        }
        Ok(templates)
    }

    pub fn list_profiles(&self) -> Result<Vec<WorkspaceProfile>, TemplateError> {
        let mut profiles = vec![];
        for path in list_yaml_files(&self.profiles_dir())? {
            profiles.push(load_profile_from_file(&path)?);
        }
        Ok(profiles)
    }

    pub fn load_template(&self, template_id: &str) -> Result<WorkspaceTemplate, TemplateError> {
        let path = self.template_path(template_id)?;
        load_template_from_file(&path)
    }

    pub fn load_profile(&self, profile_id: &str) -> Result<WorkspaceProfile, TemplateError> {
        let path = self.profile_path(profile_id)?;
        load_profile_from_file(&path)
    }

    pub fn save_template(&self, template: &WorkspaceTemplate) -> Result<(), TemplateError> {
        let path = self.templates_dir().join(format!("{}.yaml", template.id));
        let yaml = template_to_yaml(template)?;
        fs::write(path, yaml)?;
        Ok(())
    }

    pub fn save_template_yaml(&self, yaml: &str) -> Result<WorkspaceTemplate, TemplateError> {
        let template = template_from_yaml(yaml)?;
        self.save_template(&template)?;
        Ok(template)
    }

    fn template_path(&self, template_id: &str) -> Result<PathBuf, TemplateError> {
        let path = self.templates_dir().join(format!("{template_id}.yaml"));
        if path.exists() {
            Ok(path)
        } else {
            Err(TemplateError::NotFound(template_id.to_string()))
        }
    }

    fn profile_path(&self, profile_id: &str) -> Result<PathBuf, TemplateError> {
        let path = self.profiles_dir().join(format!("{profile_id}.yaml"));
        if path.exists() {
            Ok(path)
        } else {
            Err(TemplateError::NotFound(profile_id.to_string()))
        }
    }
}