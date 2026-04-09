use eframe::egui;
use image::DynamicImage;

pub trait Step {
    fn name(&self) -> &str;
    fn ui(&mut self, ui: &mut egui::Ui, selected: bool);
    fn process(&mut self, input: &DynamicImage) -> &DynamicImage;
    fn invalidate(&mut self);

    /// Returns true if parameters changed since last check, then resets the flag.
    fn dirty(&mut self) -> bool;

    /// Draw this step as a card in the side panel.
    /// Returns true if the card was clicked.
    fn card(&mut self, ui: &mut egui::Ui, selected: bool) -> bool {
        let fill = if selected {
            ui.visuals().selection.bg_fill
        } else {
            ui.visuals().widgets.inactive.bg_fill
        };

        let frame = egui::Frame::new()
            .fill(fill)
            .corner_radius(6)
            .inner_margin(8)
            .outer_margin(egui::Margin::symmetric(0, 2))
            .stroke(ui.visuals().widgets.inactive.bg_stroke);

        let mut clicked = false;
        frame.show(ui, |ui| {
            ui.set_width(ui.available_width());
            let title = ui.label(egui::RichText::new(self.name()).strong());
            if title.clicked() || title.interact(egui::Sense::click()).clicked() {
                clicked = true;
            }
            self.ui(ui, selected);
        });
        clicked
    }
}
