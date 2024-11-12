use std::{env, path::Path};

use eframe::egui::{self, Layout};
use gpi_egui::image2d::Image2D;
use ndarray::Array;

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
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        let test_image = Array::from_shape_vec(
            (300, 300),
            Array::linspace(0., 100., 90000).into_raw_vec_and_offset().0,
        )
        .unwrap();
        let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let path = Path::new(&dir).join("src/buf.png");
        //+ "/src/buf.png"));
        println!("{:?}", path.parent());

        let window = 1.;
        let level = 0.;
        Self {
            //image2d: Image2D::from_array2d(test_image),
            image2d: Image2D::from_path(&path, window, level),
            window,
            level,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let texture = self.image2d.get_handle(ui);

            let space = ui.available_size();
            let size = space[0].min(space[1]);

            //// Image
            ui.image((texture.id(), [size, size].into()));

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
