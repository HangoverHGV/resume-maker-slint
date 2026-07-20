use serde::{Deserialize, Serialize};
use slint::SharedString;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::LazyLock;
use struct_iterable::Iterable;
use crate::PersonalData;

pub static RESUME_SAVE_DIR: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("saves"));
pub static RESUME_SAVE_FILE: LazyLock<PathBuf> =
    LazyLock::new(|| RESUME_SAVE_DIR.join("resume_data.json"));

pub fn create_resume_folder() {
    if let Err(e) = fs::create_dir_all(&*RESUME_SAVE_DIR) {
        eprintln!(
            "Failed to create directory '{:?}': {}",
            &*RESUME_SAVE_DIR, e
        );
        return;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Iterable, Default)]
pub struct PersonalInfo {
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

impl PersonalInfo {
    #[allow(dead_code)]
    pub fn get_info_model(&self) -> Vec<SharedString> {
        let iter = self.iter();
        let mut model = Vec::with_capacity(iter.size_hint().0);

        for (_name, value) in iter {
            if let Some(s) = value.downcast_ref::<String>() {
                model.push(SharedString::from(s.as_str()));
            }
        }

        model
    }

    pub fn to_slint(&self, cv_name: &str) -> PersonalData {
        PersonalData {
            cv_name: cv_name.into(),
            name: self.name.as_str().into(),
            job_title: self.job_title.as_str().into(),
            email: self.email.as_str().into(),
            phone: self.phone.as_str().into(),
            location: self.location.as_str().into(),
            linkedin: self.linkedin.as_str().into(),
            github: self.github.as_str().into(),
            website: self.website.as_str().into(),
            description: self.description.as_str().into(),
        }
    }
    #[allow(dead_code)]
    pub fn from_slint(slint_data: PersonalData) -> (String, Self) {
        let cv_name = slint_data.cv_name.to_string();
        let info = Self {
            name: slint_data.name.to_string(),
            job_title: slint_data.job_title.to_string(),
            email: slint_data.email.to_string(),
            phone: slint_data.phone.to_string(),
            location: slint_data.location.to_string(),
            linkedin: slint_data.linkedin.to_string(),
            github: slint_data.github.to_string(),
            website: slint_data.website.to_string(),
            description: slint_data.description.to_string(),
        };

        (cv_name, info)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResumeContainer {
    pub resumes: Vec<HashMap<String, PersonalInfo>>,
}