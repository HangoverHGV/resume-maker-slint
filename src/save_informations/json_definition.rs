use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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