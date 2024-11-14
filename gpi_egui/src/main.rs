use egui_plot::{Bar, BarChart, Plot};
use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::{env, fs::File, path::Path};

use eframe::egui::{self, Color32, Stroke};
use gpi_egui::image2d::Image2D;

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
    window: f64,
    level: f64,
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

        let window = 1.;
        let level = 0.;
        let test_slice = Image2D::from_array3d(test_image, 6, 5, window, level);
        Self {
            //image2d: Image2D::from_array2d(test_image),
            image2d: test_slice,
            window,
            level,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let aspect = self.image2d.aspect_ratio() as f32;
            let texture = self.image2d.get_handle(ui);

            let space = ui.available_size();
            let ui_width = space[0];
            let ui_height = space[1] - 200.;

            let (image_width, image_height) = if ui_width < ui_height * aspect {
                (ui_width, ui_width / aspect)
            } else {
                (ui_height * aspect, ui_height)
            };

            //// Image
            ui.image((texture.id(), [image_width, image_height].into()));

            //// HISTOGRAM
            let histogram = self.image2d.image_distribution();
            let bars = histogram
                .into_iter()
                .enumerate()
                .map(|(i, x)| {
                    Bar::new(i as f64, x as f64)
                        .stroke(Stroke {
                            width: 0.01,
                            color: Color32::WHITE,
                        })
                        .fill(Color32::WHITE)
                        .width(0.8)
                })
                .collect();
            let bar_chart: BarChart = BarChart::new(bars);
            egui::Frame::default()
                .inner_margin(1.0)
                .fill(egui::Color32::GRAY)
                .show(ui, |ui| {
                    egui::Frame::default()
                        .inner_margin(1.0)
                        .fill(egui::Color32::BLACK)
                        .show(ui, |ui| {
                            Plot::new("my_plot")
                                .height(100.)
                                .allow_zoom(false)
                                .allow_scroll(false)
                                .allow_drag(false)
                                .show_x(false)
                                .show_y(false)
                                .clamp_grid(true)
                                .show_background(false)
                                .show_axes(false)
                                .show_grid(false)
                                //.x_grid_spacer(|_grid_input| {
                                //    vec![GridMark {
                                //        value: 0.0,
                                //        step_size: 10.0,
                                //    }]
                                //})
                                //.y_grid_spacer(|_grid_input| {
                                //    vec![GridMark {
                                //        value: 0.0,
                                //        step_size: 10.0,
                                //    }]
                                //})
                                .show(ui, |plot_ui| plot_ui.bar_chart(bar_chart));
                        });
                });

            //// Sliders
            egui::Grid::new("my_grid")
                .num_columns(2)
                .spacing([20.0, 4.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Window");
                    const SLIDER_MARGIN: f32 = 100.;
                    ui.style_mut().spacing.slider_width =
                        (ui.available_width() - SLIDER_MARGIN).max(1.0);
                    let window_slider = ui.add(egui::Slider::new(&mut self.window, 0.0..=5.0));
                    ui.end_row();

                    ui.label("Level");
                    let level_slider = ui.add(egui::Slider::new(&mut self.level, -255.0..=255.0));
                    ui.end_row();

                    //// Update Image with new values
                    if window_slider.changed() || level_slider.changed() {
                        self.image2d.update_levels(self.window, self.level);
                    }
                });
        });
    }
}
