#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod save_information;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    save_information::setup_personal_data_save(&ui);

    ui.run()?;
    Ok(())
}