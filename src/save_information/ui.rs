use std::fs::File;
use slint::{ComponentHandle, SharedString, VecModel};
use crate::AppWindow;
use std::rc::Rc;
use crate::save_information::save_json::save_file_json;
use crate::save_information::load_json::load_all_resumes;
use crate::save_information::delete_json::delete_resume_from_json;
use crate::save_information::json_setup::{ResumeContainer, RESUME_SAVE_FILE};

pub fn setup_personal_data_save(ui: &AppWindow){
    let ui_handle = ui.as_weak();
    ui.on_save_personal_info(move |cv_name, name, job_title,
                                   email, phone, location,
                                   linkedin, github, website, description |{


        save_file_json(cv_name.to_string(),
                       name.to_string(), job_title.to_string(), email.to_string(), phone.to_string(),
                       location.to_string(), linkedin.to_string(), github.to_string(), website.to_string(), description.to_string());

        if let Some(ui) = ui_handle.upgrade() {
            println!("Resume saved successfully. Repopulating resume list...");
            setup_resume_list(&ui);
        }

    });


}

pub fn setup_resume_list(ui: &AppWindow) {
    ui.set_resumes_list(Rc::new(VecModel::<SharedString>::default()).into());

    let mut names = Vec::new();

    if let Some(container) = load_all_resumes() {
        for map in container.resumes.iter() {
            for key in map.keys() {
                names.push(SharedString::from(key));
            }
        }
    }
    if names.len() == 1 {
        names.push(SharedString::from("Add"));
    } else if names.is_empty() {
        names.push(SharedString::from("Add"));
    }

    names.sort();

    let model = Rc::new(VecModel::from(names));
    ui.set_resumes_list(model.into());
}

pub fn load_resume_into_ui(ui: &AppWindow, resume_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&*RESUME_SAVE_FILE)?;
    let container: ResumeContainer = serde_json::from_reader(file)?;

    if let Some(entry) = container.resumes.iter().find(|map| map.contains_key(resume_name)) {
        if let Some(info) = entry.get(resume_name) {

            ui.set_cv_name(resume_name.into());
            ui.set_field_name(info.name.clone().into());
            ui.set_field_job_title(info.job_title.clone().into());
            ui.set_field_email(info.email.clone().into());
            ui.set_field_phone(info.phone.clone().into());
            ui.set_field_location(info.location.clone().into());
            ui.set_field_linkedin(info.linkedin.clone().into());
            ui.set_field_github(info.github.clone().into());
            ui.set_field_website(info.website.clone().into());
            ui.set_field_description(info.description.clone().into());

            println!("Populated UI fields successfully for profile: {}", resume_name);
            return Ok(());
        }
    }

    Err(format!("Profile structure '{}' not found.", resume_name).into())
}

pub fn resume_actions(ui: &AppWindow) {
        let ui_handle = ui.as_weak();

    ui.on_menu_item_clicked(move |resume_name, action_type| {
        let resume = resume_name.as_str();
        let action = action_type.as_str();

        if let Some(ui) = ui_handle.upgrade() {
            match action {
                "edit" => {
                    if let Err(e) = load_resume_into_ui(&ui, resume) {
                        eprintln!("Failed to load resume: {}", e);
                    }
                }
                "delete" => {
                    if let Err(e) = delete_resume_from_json(resume) {
                        eprintln!("Failed to delete resume: {}", e);
                    }
                }
                _ => {}
            }
        }
    });
}
