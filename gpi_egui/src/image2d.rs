use eframe::{
    egui::{ColorImage, ImageData, TextureFilter, TextureHandle, Ui},
    emath::inverse_lerp,
};
use image::{ImageBuffer, ImageReader, Luma};
use ndarray::{Array, Array2};
use ndarray_stats::QuantileExt;

pub struct Image2D {
    ///cached texture for rendering
    texture: Option<TextureHandle>,
    base_array: Array2<u8>,
    adjusted_image: ImageData,
    window: f64,
    level: f64,
}

impl Default for Image2D {
    fn default() -> Self {
        let default_size = (300, 300);
        let array = Array2::zeros(default_size);
        Self {
            texture: None,
            base_array: array.clone(),
            adjusted_image: Self::array_to_image(array, default_size),
            window: 1.0,
            level: 0.0,
        }
    }
}

impl Image2D {
    pub fn get_handle(&mut self, ui: &Ui) -> &TextureHandle {
        self.texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "my-image",
                self.adjusted_image.clone(),
                eframe::egui::TextureOptions {
                    magnification: TextureFilter::Nearest,
                    ..Default::default()
                },
            )
        })
    }
    pub fn from_array2d(array2d: ndarray::Array2<f64>, window: f64, level: f64) -> Self {
        let max = *array2d.max().unwrap();
        let min = *array2d.min().unwrap();
        let shape = array2d.shape();

        let base_array = array2d.map(|&v| Self::quantize(v, min, max, 1., 0.));

        let altered_quantized =
            base_array.map(|&v| Self::quantize(v as f64, 0.0, 255.0, window, level));

        Self {
            texture: None,
            base_array,
            adjusted_image: Self::array_to_image(altered_quantized, (shape[0], shape[1])),
            window,
            level,
        }
    }
    pub fn from_path(path: &std::path::Path, window: f64, level: f64) -> Self {
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        let size = (image.width() as usize, image.height() as usize);
        let image_vec = image.to_luma8().iter().map(|&b| b as f64).collect();
        let array2d = Array::from_shape_vec(size, image_vec).unwrap();
        Self::from_array2d(array2d, window, level)
    }

    pub fn update_levels(&mut self, window: f64, level: f64) {
        self.window = window;
        self.level = level;
        let shape = self.base_array.shape();
        let altered_quantized = self
            .base_array
            .map(|&v| Self::quantize(v as f64, 0.0, 255.0, window, level));
        self.adjusted_image = Self::array_to_image(altered_quantized, (shape[0], shape[1]));
        self.texture = None;
    }
}

// Helpers
impl Image2D {
    fn quantize(value: f64, min: f64, max: f64, window: f64, level: f64) -> u8 {
        let normalized = 255.0 * inverse_lerp(min..=max, value).unwrap();
        let windowed = normalized * window + level;
        windowed.round() as u8
    }

    fn array_to_image(quantized: ndarray::Array2<u8>, size: (usize, usize)) -> ImageData {
        let (data, _) = quantized.into_raw_vec_and_offset();

        let image_buffer: ImageBuffer<Luma<u8>, _> =
            ImageBuffer::from_raw(size.0 as u32, size.1 as u32, data).unwrap();

        //let image_buffer = image.to_luma8();
        let pixels = image_buffer.as_flat_samples();
        ColorImage::from_gray([size.0, size.1], pixels.as_slice()).into()
    }
}
