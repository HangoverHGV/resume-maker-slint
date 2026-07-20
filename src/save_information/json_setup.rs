use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::path::{PathBuf};
use std::fs;
use slint::SharedString;
use struct_iterable::Iterable;

pub static RESUME_SAVE_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("saves"));
pub static RESUME_SAVE_FILE: LazyLock<PathBuf> = LazyLock::new(|| RESUME_SAVE_DIR.join("resume_data.json"));
pub fn create_resume_folder() {
    if let Err(e) = fs::create_dir_all(RESUME_SAVE_DIR.clone()) {
        eprintln!("Failed to create directory '{:?}': {}", RESUME_SAVE_DIR.clone(), e);
        return;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Iterable)]
pub struct PersonalInfo{
    pub name: String,
    pub job_title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin: String,
    pub github: String,
    pub website: String,
    pub description: String,
}

impl PersonalInfo{
    pub fn get_info_model(&self) -> Vec<SharedString>{
        let iter = self.iter();

        let mut model = Vec::with_capacity(iter.size_hint().0);

        for (_name, value) in iter {
            if let Some(s) = value.downcast_ref::<String>() {
                model.push(SharedString::from(s.as_str()));
            }
        }

        model
    }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResumeContainer {
    pub resumes: Vec<HashMap<String, PersonalInfo>>,
}