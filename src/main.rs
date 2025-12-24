use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("My First Window")
            .with_inner_size([800.0, 600.0]),
        // Fill other options with defaults
        ..Default::default()
    };

    eframe::run_native(
        "My First Window",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    user_text: String,

}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            user_text: String::from("Type something here..."),
        }
    }
}

impl eframe::App for MyApp {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Namaskar, world!!");

            ui.separator();

            egui::TextEdit::multiline(&mut self.user_text)
                .desired_width(f32::INFINITY)
                .desired_rows(20)
                .show(ui);

            ui.separator();

            ui.label(format!("Characters: {}", self.user_text.len()));
        });
    }
}