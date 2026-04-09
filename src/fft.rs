use eframe::egui;
use image::{DynamicImage, GrayImage};
use rustfft::{FftPlanner, num_complex::Complex};

use crate::step::Step;

pub struct FftStep {
    cached: Option<DynamicImage>,
    dirty: bool,
}

impl FftStep {
    pub fn new() -> Self {
        Self {
            cached: None,
            dirty: false,
        }
    }

    /// Perform a 2D FFT on a grayscale image and return the log-magnitude spectrum.
    fn fft_2d(img: &GrayImage) -> GrayImage {
        let (width, height) = img.dimensions();
        let w = width as usize;
        let h = height as usize;

        // Convert pixels to complex numbers
        let mut data: Vec<Complex<f64>> = img
            .as_raw()
            .iter()
            .map(|&px| Complex::new(px as f64, 0.0))
            .collect();

        let mut planner = FftPlanner::<f64>::new();

        // TODO: FFT along each row
        // hint: planner.plan_fft_forward(w) gives you an FFT for length w
        // iterate over data.chunks_exact_mut(w) and apply fft.process() to each row

        // TODO: FFT along each column
        // hint: you need to extract each column into a temporary Vec<Complex>,
        // apply fft.process(), then write the results back
        // for col in 0..w { gather column, transform, scatter back }

        // TODO: compute log-magnitude and shift zero-frequency to center
        // 1. compute magnitude: c.norm() for each element
        // 2. apply log scale: (1.0 + mag).ln()
        // 3. shift: swap quadrants so DC is in the center
        //    swap top-left with bottom-right, top-right with bottom-left
        // 4. normalize to 0-255 range and create GrayImage

        todo!("implement 2D FFT")
    }
}

impl Step for FftStep {
    fn name(&self) -> &str {
        "FFT"
    }

    fn ui(&mut self, _ui: &mut egui::Ui, _selected: bool) {
        // no parameters for now
    }

    fn process(&mut self, input: &DynamicImage) -> &DynamicImage {
        self.cached.get_or_insert_with(|| {
            let gray = input.to_luma8();
            let spectrum = Self::fft_2d(&gray);
            DynamicImage::ImageLuma8(spectrum)
        })
    }

    fn invalidate(&mut self) {
        self.cached = None;
        self.dirty = true;
    }

    fn dirty(&mut self) -> bool {
        let was = self.dirty;
        self.dirty = false;
        was
    }
}
