use std::fs::File;
use std::io::Write;
use crate::save_information::json_setup::{ResumeContainer, RESUME_SAVE_FILE};

pub fn delete_resume_from_json(resume_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&*RESUME_SAVE_FILE)?;
    let mut container: ResumeContainer = serde_json::from_reader(file)?;

    container.resumes.retain(|map| !map.contains_key(resume_name));
    let updated_json = serde_json::to_string_pretty(&container)?;
    
    let mut file = File::create(&*RESUME_SAVE_FILE)?;
    file.write_all(updated_json.as_bytes())?;

    Ok(())
}