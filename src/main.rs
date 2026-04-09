mod fft;
mod load;
mod rotate;
mod step;

use crate::fft::FftStep;
use crate::load::LoadStep;
use crate::rotate::RotateStep;
use crate::step::Step;
use eframe::egui;
use egui::menu;

struct RasternApp {
    load: LoadStep,
    texture: Option<egui::TextureHandle>,
    steps: Vec<Box<dyn Step>>,
    selected: usize,
}

impl RasternApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            load: LoadStep::new(),
            texture: None,
            steps: vec![Box::new(RotateStep::new()), Box::new(FftStep::new())],
            selected: 0,
        }
    }
}

impl eframe::App for RasternApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // drag and drop for files
        ctx.input(|i| {
            if let Some(file) = i.raw.dropped_files.iter().last() {
                if let Some(path) = &file.path {
                    self.load.load_image(path);
                    self.texture = None;
                    for step in &mut self.steps {
                        step.invalidate();
                    }
                } else {
                    eprintln!("Could not get a filepath for dropped file");
                }
            }
        });

        // top panel with file picker menu
        egui::TopBottomPanel::top("file_menu").show(ctx, |ui| {
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open image...").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter(
                                "pictures",
                                &["png", "jpg", "jpeg", "tiff", "bmp"],
                            )
                            .pick_file()
                        {
                            self.load.load_image(&path);
                            self.texture = None;
                            for step in &mut self.steps {
                                step.invalidate();
                            }
                        }
                        ui.close_menu();
                    }
                })
            })
        });

        // side panel with step cards
        egui::SidePanel::right("processing_steps")
            .min_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Steps");
                ui.add_space(4.0);

                if self.load.card(ui, self.selected == 0) {
                    self.selected = 0;
                    self.texture = None;
                }
                for i in 0..self.steps.len() {
                    if self.steps[i].card(ui, self.selected == i + 1) {
                        self.selected = i + 1;
                        self.texture = None;
                    }
                }
            });

        // check if any step's parameters changed
        let any_dirty = self.steps.iter_mut().any(|s| s.dirty());
        if any_dirty {
            self.texture = None;
        }

        // run pipeline up to the selected step and get the display image
        let display_image = if self.load.image().is_some() {
            let mut current = self.load.image().unwrap().clone();
            let up_to = self.selected; // 0 = load, 1+ = steps
            for i in 0..up_to.min(self.steps.len()) {
                current = self.steps[i].process(&current).clone();
            }
            Some(current)
        } else {
            None
        };

        // central panel with image display
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(ref img) = display_image {
                let texture = self.texture.get_or_insert_with(|| {
                    ctx.load_texture(
                        "image",
                        LoadStep::to_color_image(img),
                        egui::TextureOptions::LINEAR,
                    )
                });
                let available = ui.available_size();
                let img_size = texture.size_vec2();
                let scale = (available[0] / img_size[0])
                    .min(available[1] / img_size[1])
                    .min(1.0);
                let display_size = img_size * scale;
                ui.image(egui::load::SizedTexture::new(
                    texture.id(),
                    display_size,
                ));
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Open or drop an image to get started");
                });
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "rastern",
        options,
        Box::new(|cc| Ok(Box::new(RasternApp::new(cc)))),
    )
}
