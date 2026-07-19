use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::path::{PathBuf};
use std::fs;

pub static RESUME_SAVE_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("saves"));
pub static RESUME_SAVE_FILE: LazyLock<PathBuf> = LazyLock::new(|| RESUME_SAVE_FILE.join("resume_data.json"));
pub fn create_resume_folder() {
    if let Err(e) = fs::create_dir_all(RESUME_SAVE_DIR.clone()) {
        eprintln!("Failed to create directory '{:?}': {}", RESUME_SAVE_DIR.clone(), e);
        return;
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersonalInfo{
    pub name: String,
    pub job_title: String,
    pub email: String,
    pub phone: String,
    pub location: String,
    pub linkedin: String,
    pub github: String,
    pub website: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResumeEntry{
    #[serde(flatten)]
    pub data: HashMap<String, PersonalInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResumeContainer {
    pub resumes: Vec<ResumeEntry>,
}