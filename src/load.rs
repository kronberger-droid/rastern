use eframe::egui;
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

use crate::step::Step;

pub struct LoadStep {
    path: Option<PathBuf>,
    image: Option<DynamicImage>,
}

impl LoadStep {
    pub fn new() -> Self {
        Self { path: None, image: None }
    }

    pub fn load_image(&mut self, path: &std::path::Path) {
        match image::open(path) {
            Ok(img) => {
                self.path = Some(path.to_path_buf());
                self.image = Some(img);
            },
            Err(e) => eprintln!("Failed to load image: {e}"),
        }
    }

    pub fn image(&self) -> Option<&DynamicImage> {
        self.image.as_ref()
    }

    pub fn to_color_image(img: &DynamicImage) -> egui::ColorImage {
        let img_rgba8 = img.clone().into_rgba8();
        let size: [usize; 2] = [
            img_rgba8.dimensions().0 as usize,
            img_rgba8.dimensions().1 as usize,
        ];

        let pixels = img_rgba8
            .into_raw()
            .chunks_exact(4)
            .map(|chunk| {
                egui::Color32::from_rgba_unmultiplied(
                    chunk[0], chunk[1], chunk[2], chunk[3],
                )
            })
            .collect();

        egui::ColorImage { size, pixels }
    }
}

impl Step for LoadStep {
    fn name(&self) -> &str {
        "Load"
    }

    fn ui(&mut self, ui: &mut egui::Ui, _selected: bool) {
        if let Some(ref img) = self.image {
            if let Some(ref path) = self.path {
                let filename = path
                    .file_name()
                    .and_then(|f| f.to_str())
                    .unwrap_or("unknown");

                ui.label(filename);
            }

            ui.label(format!(
                "width: {}, height: {}",
                img.dimensions().0,
                img.dimensions().1
            ));

            ui.label(format!("{:?}", img.color()));
        };
    }

    fn process(&mut self, _input: &DynamicImage) -> &DynamicImage {
        // LoadStep is the source -- it ignores input and returns its own image
        self.image.as_ref().expect("no image loaded")
    }

    fn invalidate(&mut self) {
        // nothing to invalidate -- the image is the source
    }

    fn dirty(&mut self) -> bool {
        false // load step is never dirty from parameter changes
    }
}
