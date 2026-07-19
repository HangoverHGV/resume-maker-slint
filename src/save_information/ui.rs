use slint::{ComponentHandle, SharedString, VecModel};
use crate::AppWindow;
use std::rc::Rc;
use crate::save_information::save_json::save_file_json;
use crate::save_information::load_json::load_all_resumes;

pub fn setup_personal_data_save(ui: &AppWindow){
    let ui_handle = ui.as_weak();
    ui.on_save_personal_info(move |cv_name, name, job_title,
                                   email, phone, location,
                                   linkedin, github, website |{
        let ui = ui_handle.unwrap();


        save_file_json(cv_name.to_string(),
                       name.to_string(), job_title.to_string(), email.to_string(), phone.to_string(),
                       location.to_string(), linkedin.to_string(), github.to_string(), website.to_string());

    });
}

pub fn setup_resume_list(ui: &AppWindow) {
    ui.set_resumes_list(Rc::new(VecModel::<SharedString>::default()).into());

    let mut names = Vec::new();

    if let Some(container) = load_all_resumes() {
        for map in container.resumes.iter() {
            for key in map.keys() {
                names.push(slint::SharedString::from(key));
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
