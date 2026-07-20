use crate::AppWindow;
use crate::save_information::delete_json::delete_resume_from_json;
use crate::save_information::json_setup::{RESUME_SAVE_FILE, ResumeContainer};
use crate::save_information::load_json::load_all_resumes;
use crate::save_information::save_json::save_file_json;
use slint::{ComponentHandle, SharedString, VecModel};
use std::fs::File;
use std::rc::Rc;

pub fn setup_personal_data_save(ui: &AppWindow) {
    let ui_handle = ui.as_weak();
    ui.on_save_personal_info(move |data| {
        save_file_json(
            data.cv_name.to_string(),
            data.name.to_string(),
            data.job_title.to_string(),
            data.email.to_string(),
            data.phone.to_string(),
            data.location.to_string(),
            data.linkedin.to_string(),
            data.github.to_string(),
            data.website.to_string(),
            data.description.to_string(),
        );

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

    names.sort();

    let model = Rc::new(VecModel::from(names));
    ui.set_resumes_list(model.into());
}

pub fn load_resume_into_ui(
    ui: &AppWindow,
    resume_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&*RESUME_SAVE_FILE)?;
    let container: ResumeContainer = serde_json::from_reader(file)?;

    if let Some(entry) = container
        .resumes
        .iter()
        .find(|map| map.contains_key(resume_name))
    {
        if let Some(info) = entry.get(resume_name) {
            ui.set_personal_data(Default::default());
            ui.set_personal_data(info.to_slint(resume_name));

            println!(
                "Populated UI fields successfully for profile: {}",
                resume_name
            );
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
                    setup_resume_list(&ui);
                }
                _ => {}
            }
        }
    });
}

pub fn clear_fields(ui: &AppWindow) {
    let ui_handle = ui.as_weak();

    ui.on_clear_all_fields(move || {
        if let Some(ui) = ui_handle.upgrade() {
            ui.set_personal_data(Default::default());
        }
    });
}