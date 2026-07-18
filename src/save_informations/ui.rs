use slint::ComponentHandle;
use crate::AppWindow;
use crate::save_informations::save_json::save_file_json;

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
