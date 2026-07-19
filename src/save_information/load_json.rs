use std::fs::File;
use std::io::Read;
use crate::save_information::json_setup::{ResumeContainer, RESUME_SAVE_FILE};

pub fn load_all_resumes() -> Option<ResumeContainer> {
    let file_path = &*RESUME_SAVE_FILE;
    if !file_path.exists() {
        return None;
    }

    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to save file safely: {}", e);
            return None;
        }
    };

    let mut json_string = String::new();
    if let Err(e) = file.read_to_string(&mut json_string){
        eprintln!("Failed to read save file contents: {}", e);
        return None;
    }

    match serde_json::from_str::<ResumeContainer>(&json_string){
        Ok(container) => Some(container),
        Err(e) => {
            eprintln!("Failed to parse resume JSON data: {}", e);
            None
        }
    }
}