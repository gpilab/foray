use eframe::{
    egui::{ColorImage, ImageData, TextureFilter, TextureHandle, Ui},
    emath::inverse_lerp,
};
use image::{ImageBuffer, ImageReader, Luma};
use ndarray::{s, Array, Array2};
use ndarray_stats::{
    histogram::{Bins, Edges, Grid},
    HistogramExt, QuantileExt,
};

pub struct Image2D {
    ///cached texture for rendering
    texture: Option<TextureHandle>,
    base_array: Array2<u8>,
    adjusted_image: ImageData,
    width: usize,
    height: usize,
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
            width: 300,
            height: 300,
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
        let width = shape[1];
        let height = shape[0];

        let base_array = array2d.map(|&v| Self::quantize(v, min, max, 1., 0.));

        let altered_quantized =
            base_array.map(|&v| Self::quantize(v as f64, 0.0, 255.0, window, level));

        Self {
            texture: None,
            base_array,
            adjusted_image: Self::array_to_image(altered_quantized, (shape[0], shape[1])),
            width,
            height,
            window,
            level,
        }
    }
    /// Create a 2d image from a 3d image by tileing the 2d image
    ///
    pub fn from_array3d(
        array3d: ndarray::Array3<f64>,
        n_rows: usize,
        n_columns: usize,
        window: f64,
        level: f64,
    ) -> Self {
        let shape = array3d.raw_dim();
        let num_slices = shape[0];
        let slice_height = shape[1];
        let slice_width = shape[2];

        println!("w: {slice_width}, h: {slice_height}, z: {num_slices}");

        let tiled_height = slice_height * n_rows;
        let tiled_width = slice_width * n_columns;

        let mut out_image: Array2<f64> = Array2::zeros((tiled_height, tiled_width));

        //let mut windows =
        //    mut_view.windows_with_stride((slice_height, slice_width), (slice_height, slice_width));

        //windows.into_iter().enumerate().for_each(|(i, w)| {
        //    let mut_w = w.slice_mut();
        //    view_mut().assign(&array3d.slice(s![i, .., ..]))
        //});
        for n_y in 0..n_rows {
            for n_x in 0..n_columns {
                let z = n_x + n_y * n_columns;
                let x = n_x * slice_width;
                let y = n_y * slice_height;

                let slice = array3d.slice(s![z, .., ..]);
                out_image
                    .slice_mut(s![y..y + slice_height, x..x + slice_width])
                    .assign(&slice);
            }
        }
        //let array2d = array3d
        //    .into_shape_with_order((tiled_height, tiled_width))
        //    .unwrap();

        let f_shape = out_image.raw_dim();
        let final_height = f_shape[0];
        let final_width = f_shape[1];

        println!("final w: {final_width}, final h: {final_height}");

        //let array2d = array3d.slice(s![1, .., ..]).to_owned();
        Self::from_array2d(out_image, window, level)
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
    //// PERF: Compute the histogram once, and store in self
    pub fn image_distribution(&self) -> Vec<usize> {
        let grid = Grid::from(vec![Bins::new(Edges::from((0..=255).collect::<Vec<_>>()))]); //let grid = GridBuilder::<u8>::from_array(&self.base_array)
                                                                                            //    .build();
        let size = self.base_array.len();
        let histogram = self
            .base_array
            .clone()
            .into_shape_with_order((size, 1))
            .unwrap()
            .histogram(grid);

        //// remove background
        let max_i = histogram.counts().argmax().unwrap()[0];
        let mut counts = histogram.counts().to_owned().into_raw_vec_and_offset().0;

        let left = match max_i {
            0 => 0,
            1..255 => counts[max_i],
            _ => panic!("histogram  unexpectedly has more than 255 values"),
        };
        let right = match max_i {
            254 => 0,
            0..254 => counts[max_i],
            _ => panic!("histogram  unexpectedly has more than 255 values"),
        };
        let averaged = (left + right) / 2;
        counts[max_i] = averaged;
        counts
    }
    /// width / height
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

// Helpers
impl Image2D {
    fn quantize(value: f64, min: f64, max: f64, window: f64, level: f64) -> u8 {
        let normalized = 255.0 * inverse_lerp(min..=max, value).unwrap_or(0.0);
        let windowed = normalized * window + level;
        windowed.round() as u8
    }

    fn array_to_image(quantized: ndarray::Array2<u8>, size: (usize, usize)) -> ImageData {
        let (data, _) = quantized.into_raw_vec_and_offset();

        let image_buffer: ImageBuffer<Luma<u8>, _> =
            ImageBuffer::from_raw(size.1 as u32, size.0 as u32, data).unwrap();

        //let image_buffer = image.to_luma8();
        let pixels = image_buffer.as_flat_samples();
        ColorImage::from_gray([size.1, size.0], pixels.as_slice()).into()
    }
}
