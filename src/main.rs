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

        egui::SidePanel::left("editor_panel")
            .exact_width(ctx.screen_rect().width() / 2.0)
            .show(ctx, |ui| {
                ui.heading("Editor");
                ui.separator();

                egui::TextEdit::multiline(&mut self.user_text)
                    .desired_width(f32::INFINITY)  // Fill available width
                    .desired_rows(25)
                    .show(ui);

                ui.label(format!("Characters: {}", self.user_text.len()));
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Preview");
            ui.separator();

            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(&self.user_text)
                            .size(14.0)
                    );
                });
        });

    }
}