use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use crate::save_informations::json_definition::{PersonalInfo, ResumeContainer, ResumeEntry};

pub fn save_file_json(cv_name: String, name: String, job_title: String, email: String, phone: String,
                 location: String, linkedin: String, github: String, website: String) {
    let dir_path = Path::new("saves");
    if let Err(e) = fs::create_dir_all(dir_path) {
        eprintln!("Failed to create directory 'saves': {}", e);
        return;
    }

    let info = PersonalInfo{
        name: name,
        job_title: job_title,
        email: email,
        phone: phone,
        location: location,
        linkedin: linkedin,
        github: github,
        website: website,
    };

    let mut resume_map = HashMap::new();
    resume_map.insert(cv_name.clone(), info);

    let container = ResumeContainer{
        resumes: vec![ResumeEntry {data: resume_map}],
    };

    let json_data = match serde_json::to_string_pretty(&container){
        Ok(json) => json,
        Err(e) => {
            eprintln!("Failed to serialize resume data to JSON: {}", e);
            return;
        }
    };

    let file_path = dir_path.join("resumes_data.json");

    match File::create(&file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(json_data.as_bytes()){
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