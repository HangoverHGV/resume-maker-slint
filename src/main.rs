#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod save_information;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {

    save_information::create_resume_folder();

    let ui = AppWindow::new()?;

    save_information::setup_resume_list(&ui);
    save_information::setup_personal_data_save(&ui);


    // 4. Run the application
    ui.run()?;
    Ok(())
}