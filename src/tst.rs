#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

use serde::Serialize;
use tera::{Context, Tera};
use typst_as_lib::TypstEngine;
use std::fs;

#[derive(Serialize)]
struct Job {
    role: String,
    company: String,
    dates: String,
    description: String,
}

fn generate_pdf(name: String, email: String, phone: String) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize Tera
    let mut tera = Tera::default();
    tera.add_template_file("templates/resume.typ", Some("resume"))?;

    let mut context = Context::new();
    context.insert("name", &name);
    context.insert("email", &email);
    context.insert("phone", &phone);

    let jobs = vec![
        Job {
            role: "Senior Systems Engineer".to_string(),
            company: "Tech Corp".to_string(),
            dates: "2023 - Present".to_string(),
            description: "Led development of a blazing fast, memory-safe backend engine written completely in Rust.".to_string(),
        },
        Job {
            role: "Software Developer".to_string(),
            company: "Web Solutions".to_string(),
            dates: "2021 - 2023".to_string(),
            description: "Maintained legacy applications while designing modern responsive user interfaces.".to_string(),
        },
    ];
    context.insert("jobs", &jobs);

    let typst_source = tera.render("resume", &context)?;

    // 2. EMBED THE FONT DATA AT COMPILE TIME
    // This bakes the font bytes directly into your binary executable.
    let font_bytes = include_bytes!("../fonts/Roboto-Regular.ttf");

    // 3. Build Typst Engine with the embedded font
    let engine = TypstEngine::builder()
        .main_file(typst_source)
        .fonts([font_bytes.as_slice()])
        .build();

    // 4. Compile to PDF
    let pdf_bytes = engine.with_world(|world| {
        let doc = typst::compile(world)
            .output
            .map_err(|e| format!("Typst compilation failed: {:?}", e))?;

        typst_pdf::pdf(&doc, &Default::default())
            .map_err(|e| format!("PDF generation failed: {:?}", e))
    })
        .map_err(|e| format!("Typst infrastructure error: {:?}", e))??;

    fs::write("output_resume.pdf", pdf_bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_generate_resume(move || {
        let ui = ui_handle.unwrap();
        let name = ui.get_user_name().to_string();
        let email = ui.get_user_email().to_string();
        let phone = ui.get_user_phone().to_string();

        match generate_pdf(name, email, phone) {
            Ok(_) => println!("Successfully generated output_resume.pdf!"),
            Err(e) => eprintln!("Error: {}", e),
        }
    });

    ui.run()?;
    Ok(())
}