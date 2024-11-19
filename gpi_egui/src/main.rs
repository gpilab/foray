use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::{env, fs::File, path::Path};

use eframe::egui;
use gpi_egui::{
    image2d::{Image2D, ImageDisplayOptions},
    widgets::histogram::toggle_ui,
};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    native_options.viewport.inner_size = Some((400., 600.).into());
    let _ = eframe::run_native(
        "gpi_v2",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    );
}

struct MyEguiApp {
    image2d: Image2D,
    image_display_options: ImageDisplayOptions,
}

impl MyEguiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        //let test_image = Array::from_shape_vec(
        //    (300, 300),
        //    Array::linspace(0., 100., 90000).into_raw_vec_and_offset().0,
        //)
        //.unwrap();

        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        //let path = Path::new(&dir).join("src/buf.png");
        //let image2d = Image2D::from_path(&path, window, level);

        let path = Path::new(&dir).join("src/30_slices.npy");
        let reader = File::open(path).unwrap();
        let test_image = Array3::<f64>::read_npy(reader).unwrap();

        let test_slice = Image2D::from_array3d(test_image, 6, 5);
        Self {
            //image2d: Image2D::from_array2d(test_image),
            image2d: test_slice,
            image_display_options: Default::default(),
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let aspect = self.image2d.aspect_ratio() as f32;
            let texture = self.image2d.get_handle(ui, &self.image_display_options);

            let space = ui.available_size();
            let ui_width = space[0];
            let ui_height = space[1] - 200.;

            let (image_width, image_height) = if ui_width < ui_height * aspect {
                (ui_width, ui_width / aspect)
            } else {
                (ui_height * aspect, ui_height)
            };

            let image_width = image_width.max(1.);
            let image_height = image_height.max(1.);

            //// Image
            ui.image((texture.id(), [image_width, image_height].into()));

            if toggle_ui(
                ui,
                &self.image2d.histogram,
                &mut self.image_display_options.floor,
                &mut self.image_display_options.ceiling,
            ) {
                println!("Chagned!");
                self.image2d.request_redraw();
            }
            ////// HISTOGRAM
            //if draw_histogram(
            //    ui,
            //    &self.image2d.histogram,
            //    &mut self.image_display_options.floor,
            //    &mut self.image_display_options.ceiling,
            //) {
            //    self.image2d.request_redraw();
            //}

            //// Sliders

            //    egui::Grid::new("my_grid")
            //        .num_columns(2)
            //        .spacing([20.0, 4.0])
            //        .striped(true)
            //        .show(ui, |ui| {
            //            ui.label("Window");
            //            const SLIDER_MARGIN: f32 = 100.;
            //            ui.style_mut().spacing.slider_width =
            //                (ui.available_width() - SLIDER_MARGIN).max(1.0);
            //
            //            let window_slider = ui.add(egui::Slider::new(
            //                &mut self.image_display_options.window,
            //                0.0..=5.0,
            //            ));
            //            ui.end_row();
            //
            //            ui.label("Level");
            //            let level_slider = ui.add(egui::Slider::new(
            //                &mut self.image_display_options.level,
            //                -255.0..=255.0,
            //            ));
            //            ui.end_row();
            //
            //            ui.label("Floor");
            //            let floor_slider = ui.add(egui::Slider::new(
            //                &mut self.image_display_options.floor,
            //                0.0..=255.0,
            //            ));
            //            ui.end_row();
            //
            //            ui.label("Ceiling");
            //            let ceiling_slider = ui.add(egui::Slider::new(
            //                &mut self.image_display_options.ceiling,
            //                0.0..=255.0,
            //            ));
            //            ui.end_row();
            //
            //            //// Constraints
            //            self.image_display_options.apply_constraints();
            //
            //            //// Update Image with new values
            //            if window_slider.changed()
            //                || level_slider.changed()
            //                || floor_slider.changed()
            //                || ceiling_slider.changed()
            //            {
            //                self.image2d.request_redraw();
            //            }
            //        });
            //});
        });
    }
}
