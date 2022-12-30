use eframe::egui::{self, ScrollArea, TextStyle};
fn main() {
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some(egui::vec2(320.0, 500.1)),
        ..Default::default()
    };
    eframe::run_native(
        "Application",
        options,
        Box::new(|_cc| Box::new(MainApp::default())),
    );
}

struct MainApp {
    name: String,
    age: u32,
    upload_paths: Vec<String>
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            upload_paths: Vec::new()
        }
    }
}

impl eframe::App for MainApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe:: Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            if ui.button("Choose File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    println!("{}", path.display().to_string());
                }
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            let text_style = TextStyle::Body;
            let row_height = ui.text_style_height(&text_style);
            let num_rows = self.upload_paths.len();

            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show_rows(ui, row_height, num_rows, |ui, row_range| {
                    for row in row_range {
                        if let Some(path) = self.upload_paths.get(row) {
                            let text = format!("{}. {path}", row + 1);
                            ui.label(text);
                        }
                    }
                })
        });

        if !ctx.input().raw.dropped_files.is_empty() {
            let dropped_files = ctx.input().raw.dropped_files.clone();
            let dropped_files: Vec<String> = dropped_files
                .into_iter()
                .filter_map(|file| file.path)
                .map(|path| path.display().to_string())
                .collect();
            self.upload_paths.extend(dropped_files);
        }

        if !ctx.input().raw.hovered_files.is_empty() {
            let files: Vec<String> = ctx.input().raw.hovered_files.clone()
                .into_iter()
                .filter_map(|file| file.path)
                .map(|path| path.display().to_string())
                .collect();
            println!("{:#?}", files.len());
        }
    }
}
