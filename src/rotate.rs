use eframe::egui;
use image::DynamicImage;

use crate::step::Step;

pub struct RotateStep {
    angle: u16, // 0, 90, 180, 270
    cached: Option<DynamicImage>,
    dirty: bool,
}

impl RotateStep {
    pub fn new() -> Self {
        Self { angle: 0, cached: None, dirty: false }
    }
}

impl Step for RotateStep {
    fn name(&self) -> &str {
        "Rotate"
    }

    fn ui(&mut self, ui: &mut egui::Ui, selected: bool) {
        ui.label(format!("{}deg", self.angle));

        // Only show controls when the card is selected
        if selected {
            let old_angle = self.angle;

            ui.horizontal(|ui| {
                ui.radio_value(&mut self.angle, 90, "90");
                ui.radio_value(&mut self.angle, 180, "180");
                ui.radio_value(&mut self.angle, 270, "270");
                ui.radio_value(&mut self.angle, 0, "0");
            });

            // invalidate cache if angle changed
            if self.angle != old_angle {
                self.cached = None;
                self.dirty = true;
            }
        }
    }

    fn process(&mut self, input: &DynamicImage) -> &DynamicImage {
        self.cached
            .get_or_insert_with(|| match self.angle {
                90 => input.rotate90(),
                180 => input.rotate180(),
                270 => input.rotate270(),
                _ => input.clone(),
            })
    }

    fn invalidate(&mut self) {
        self.cached = None;
        self.dirty = true;
    }

    fn dirty(&mut self) -> bool {
        let was_dirty = self.dirty;
        self.dirty = false;
        was_dirty
    }
}
