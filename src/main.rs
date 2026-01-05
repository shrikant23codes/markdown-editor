use eframe::egui;
use pulldown_cmark::{Event, HeadingLevel, Parser, Tag, TagEnd};


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

                egui::ScrollArea::vertical()
                    .show(ui, |ui|{
                        egui::TextEdit::multiline(&mut self.user_text)
                            .desired_width(f32::INFINITY)  // Fill available width
                            .desired_rows(100)
                            .show(ui);

                        ui.label(format!("Characters: {}", self.user_text.len()));
                    });
            });
        
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Preview");
            ui.separator();

            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    self.render_markdown(ui);
                });
        });

    }
}

impl  MyApp {

    fn render_markdown(&self, ui: &mut egui::Ui) {
        let parser = Parser::new(&self.user_text);

        let mut in_heading = false;
        let mut heading_level = 1;
        let mut in_emphasis = false;
        let mut in_strong = false;
        let mut in_paragraph = false;
        let mut text_parts: Vec<egui::RichText> = Vec::new();
        let mut in_list = false;
        let mut in_list_item = false;
        
        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    in_heading = true;
                    heading_level = match level {
                        HeadingLevel::H1 => 1,
                        HeadingLevel::H2 => 2,
                        HeadingLevel::H3 => 3,
                        HeadingLevel::H4 => 4,
                        HeadingLevel::H5 => 5,
                        HeadingLevel::H6 => 6,
                    };
                }

                Event::End(TagEnd::Heading{..}) => {
                    in_heading = false;
                    ui.add_space(10.0); // space after heading
                }

                Event::Start(Tag::Paragraph) => {
                    in_paragraph = true;
                    text_parts.clear();
                }
                
                Event::Start(Tag::Emphasis) => {
                    in_emphasis = true;
                }

                Event::End(TagEnd::Emphasis) => {
                    in_emphasis = false;
                }

                Event::Start(Tag::Strong) => {
                    in_strong = true;
                }

                Event::End(TagEnd::Strong) => {
                    in_strong = false;
                }

                Event::Start(Tag::List(_)) => {
                    in_list = true;
                }

                Event::End(TagEnd::List(_)) => {
                    in_list = false;
                    ui.add_space(8.0);
                }

                Event::Start(Tag::Item) => {
                    in_list_item = true;
                    text_parts.clear();
                }

                Event::End(TagEnd::Item) => {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(egui::RichText::new("• ").size(14.0));

                        for part in &text_parts {
                            ui.label(part.clone());
                        }
                    });

                    in_list_item = false;
                    text_parts.clear();
                }

                Event::End(TagEnd::Paragraph) => {
                    // Render accumulated text parts in one line
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;
                        for part in &text_parts {
                            ui.label(part.clone());
                        }
                    });
                    text_parts.clear();
                    in_paragraph = false;
                    ui.add_space(8.0);
                }

                Event::Text(text) => {
                    let mut rich_text = egui::RichText::new(text.as_ref());

                    if in_heading {
                        let size = match heading_level {
                            1 => 32.0,
                            2 => 28.0,
                            3 => 24.0,
                            4 => 20.0,
                            5 => 16.0,
                            _ => 14.0,
                        };
                        rich_text = rich_text.size(size).strong();
                    } else {
                        rich_text = rich_text.size(14.0); // default size
                    }

                    if in_emphasis {
                        rich_text = rich_text.italics();
                    }

                    if in_strong {
                        rich_text = rich_text.strong();
                    }

                    if in_paragraph || in_list_item{
                        text_parts.push(rich_text);
                    } else {
                        ui.label(rich_text);
                    }
                }

                Event::Code(code) => {
                    let code_text = egui::RichText::new(code.as_ref())
                        .monospace()
                        .background_color(egui::Color32::from_rgb(40, 40, 40));
                    
                    if in_paragraph  || in_list_item{
                        text_parts.push(code_text);
                    } else {
                        ui.label(code_text);
                    }
                }

                _ => {}
                
            }


        }
    }
    
}