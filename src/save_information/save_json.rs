use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

use crate::save_information::json_setup::{PersonalInfo, ResumeContainer};
use crate::save_information::json_setup::RESUME_SAVE_FILE;

pub fn save_file_json(
    cv_name: String,
    name: String,
    job_title: String,
    email: String,
    phone: String,
    location: String,
    linkedin: String,
    github: String,
    website: String,
    description: String,
) {
    let info = PersonalInfo {
        name,
        job_title,
        email,
        phone,
        location,
        linkedin,
        github,
        website,
        description
    };

    let file_path = &*RESUME_SAVE_FILE;

    let mut container = if file_path.exists() {
        match File::open(&file_path).and_then(|mut f| {
            let mut contents = String::new();
            f.read_to_string(&mut contents)?;
            Ok(contents)
        }) {
            Ok(contents) => serde_json::from_str::<ResumeContainer>(&contents).unwrap_or_else(|_| ResumeContainer { resumes: vec![] }),
            Err(_) => ResumeContainer { resumes: vec![] },
        }
    } else {
        ResumeContainer { resumes: vec![] }
    };

    if let Some(entry) = container.resumes.first_mut() {
        entry.insert(cv_name, info);
    } else {
        let mut resume_map = HashMap::new();
        resume_map.insert(cv_name, info);
        container.resumes.push(resume_map);
    }

    let json_data = match serde_json::to_string_pretty(&container) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to serialize resume data to JSON: {}", e);
            return;
        }
    };

    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(json_data.as_bytes()) {
                eprintln!("Failed to write contents to file {:?}: {}", file_path, e);
            } else {
                println!("Successfully saved data to {:?}", file_path);
            }
        }
        Err(e) => {
            eprintln!("Failed to create file {:?}: {}", file_path, e);
        }
    }
}