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

#[derive(Clone)]
pub struct ImageDisplayOptions {
    pub floor: f32,
    pub ceiling: f32,
}

impl ImageDisplayOptions {
    pub fn apply_constraints(&mut self) {
        let past = self.clone();
        self.ceiling = past.ceiling.max(past.floor + 1.);
        self.floor = past.floor.clamp(0., past.ceiling - 1.);
    }
}

impl Default for ImageDisplayOptions {
    fn default() -> Self {
        Self {
            floor: 0.0,
            ceiling: 1.0,
        }
    }
}

pub struct Image2D {
    ///cached texture for rendering
    texture: Option<TextureHandle>,
    //image_data: Option<ImageData>,
    base_array: Array2<u8>,
    pub histogram: Vec<usize>,
    width: usize,
    height: usize,
}

impl Default for Image2D {
    fn default() -> Self {
        let default_size = (300, 300);
        let array = Array2::zeros(default_size);
        Self {
            texture: None,
            //image_data: None,
            base_array: array.clone(),
            histogram: Self::image_distribution(&array),
            //adjusted_image: Self::array_to_image(array, default_size),
            width: 300,
            height: 300,
        }
    }
}

impl Image2D {
    pub fn get_handle(&mut self, ui: &Ui, display_options: &ImageDisplayOptions) -> &TextureHandle {
        // texture gets recalucated whenever it gets set to None
        self.texture.get_or_insert_with(|| {
            let shape = self.base_array.shape();

            let altered_quantized = self.base_array.map(|&v| {
                Self::quantize(
                    v as f32,
                    display_options.floor * 255.0,
                    display_options.ceiling * 255.0,
                )
            });

            let image_data = Self::array_to_image(altered_quantized, (shape[0], shape[1]));
            ui.ctx().load_texture(
                "my-image",
                image_data,
                eframe::egui::TextureOptions {
                    magnification: TextureFilter::Nearest,
                    ..Default::default()
                },
            )
        })
    }
    pub fn from_array2d(array2d: ndarray::Array2<f64>) -> Self {
        let max = *array2d.max().unwrap() as f32;
        let min = *array2d.min().unwrap() as f32;
        let shape = array2d.shape();
        let width = shape[1];
        let height = shape[0];

        let base_array = array2d.map(|&v| Self::quantize(v as f32, min, max));

        Self {
            texture: None,
            histogram: Self::image_distribution(&base_array),
            base_array,
            width,
            height,
        }
    }

    /// Create a 2d image from a 3d image by tileing the 2d image
    pub fn from_array3d(array3d: ndarray::Array3<f64>, n_rows: usize, n_columns: usize) -> Self {
        let shape = array3d.raw_dim();
        let num_slices = shape[0];
        let slice_height = shape[1];
        let slice_width = shape[2];

        //TODO: support non filled grids,
        //handle failure gracefully
        assert!(n_rows * n_columns == num_slices);

        let tiled_height = slice_height * n_rows;
        let tiled_width = slice_width * n_columns;

        // Allocate the final image size
        let mut out_image: Array2<f64> = Array2::zeros((tiled_height, tiled_width));

        // Loop over the coordinates of the top left image of
        // each desired image tile, and copy the image  from the input array
        // into the correct location in the 2d image
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

        Self::from_array2d(out_image)
    }

    pub fn from_path(path: &std::path::Path) -> Self {
        let image = ImageReader::open(path).unwrap().decode().unwrap();
        let size = (image.width() as usize, image.height() as usize);
        let image_vec = image.to_luma8().iter().map(|&b| b as f64).collect();
        let array2d = Array::from_shape_vec(size, image_vec).unwrap();
        Self::from_array2d(array2d)
    }

    /// Flag the image as needing to recompute it's texture
    pub fn request_redraw(&mut self) {
        self.texture = None;
    }

    /// width / height
    pub fn aspect_ratio(&self) -> f64 {
        self.width as f64 / self.height as f64
    }
}

// Helpers
impl Image2D {
    fn quantize(value: f32, min: f32, max: f32) -> u8 {
        let normalized = 255.0 * inverse_lerp(min..=max, value).unwrap_or(0.0);
        normalized.round() as u8
    }

    fn array_to_image(quantized: ndarray::Array2<u8>, size: (usize, usize)) -> ImageData {
        let (data, _) = quantized.into_raw_vec_and_offset();

        let image_buffer: ImageBuffer<Luma<u8>, _> =
            ImageBuffer::from_raw(size.1 as u32, size.0 as u32, data).unwrap();

        //let image_buffer = image.to_luma8();
        let pixels = image_buffer.as_flat_samples();
        ColorImage::from_gray([size.1, size.0], pixels.as_slice()).into()
    }

    //// PERF: Compute the histogram once, and store in self
    pub fn image_distribution(array2d: &Array2<u8>) -> Vec<usize> {
        let grid = Grid::from(vec![Bins::new(Edges::from((0..=255).collect::<Vec<_>>()))]);

        let size = array2d.len();
        let histogram = array2d
            .clone()
            .into_shape_with_order((size, 1))
            .unwrap()
            .histogram(grid);

        //// remove background
        let max_i = histogram.counts().argmax().unwrap()[0];
        let mut counts = histogram.counts().to_owned().into_raw_vec_and_offset().0;

        //let left = match max_i {
        //    0 => 0,
        //    1..255 => counts[max_i],
        //    _ => panic!("histogram  unexpectedly has more than 255 values"),
        //};
        //let right = match max_i {
        //    254 => 0,
        //    0..254 => counts[max_i],
        //    _ => panic!("histogram  unexpectedly has more than 255 values"),
        //};
        //let averaged = (left + right) / 2;
        counts[max_i] = 0;
        counts
    }
}
