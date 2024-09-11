mod steganography;
mod utils;

use eframe::egui;
use eframe::{App, Frame};
use steganography::{embed_message_in_image, extract_message_from_image};

struct SteganographyApp {
    input_path: String,
    output_path: String,
    secret_message: String,
    extracted_message: String,
}

impl Default for SteganographyApp {
    fn default() -> Self {
        Self {
            input_path: String::new(),
            output_path: String::new(),
            secret_message: String::new(),
            extracted_message: String::new(),
        }
    }
}

impl App for SteganographyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("LSB Steganography Tool");

            // Input path
            ui.horizontal(|ui| {
                ui.label("Input image path:");
                ui.text_edit_singleline(&mut self.input_path);
            });

            // Output path
            ui.horizontal(|ui| {
                ui.label("Output image path:");
                ui.text_edit_singleline(&mut self.output_path);
            });

            // Secret message
            ui.horizontal(|ui| {
                ui.label("Secret message:");
                ui.text_edit_singleline(&mut self.secret_message);
            });

            // Embed button
            if ui.button("Embed Message").clicked() {
                if let Err(e) = embed_message_in_image(&self.input_path, &self.output_path, &self.secret_message) {
                    ui.label(format!("Error: {}", e));
                } else {
                    ui.label("Message embedded successfully!");
                }
            }

            // Extract button
            if ui.button("Extract Message").clicked() {
                let message_length = self.secret_message.len();
                if let Ok(message) = extract_message_from_image(&self.input_path, message_length) {
                    self.extracted_message = message;
                }
            }

            // Display extracted message
            ui.label(format!("Extracted message: {}", self.extracted_message));
        });
    }
}
fn main() -> Result<(), eframe::Error> {
    let app = SteganographyApp::default();
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Steganography GUI",
        native_options,
        Box::new(|_| Box::new(app)),
    )?;
    Ok(())
}