#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod save_informations;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;

    save_informations::setup_personal_data_save(&ui);

    ui.run()?;
    Ok(())
}