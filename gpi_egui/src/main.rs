use ndarray::Array3;
use ndarray_npy::ReadNpyExt;
use std::{env, fs::File, path::Path};

use eframe::{
    egui::{self, Color32, Id, Painter, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2},
    emath::{self, RectTransform},
};
use gpi_egui::image2d::{Image2D, ImageDisplayOptions};

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

            //// HISTOGRAM
            if draw_histogram(
                ui,
                &self.image2d.histogram,
                &mut self.image_display_options.floor,
                &mut self.image_display_options.ceiling,
            ) {
                self.image2d.request_redraw();
            }

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

                    let window_slider = ui.add(egui::Slider::new(
                        &mut self.image_display_options.window,
                        0.0..=5.0,
                    ));
                    ui.end_row();

                    ui.label("Level");
                    let level_slider = ui.add(egui::Slider::new(
                        &mut self.image_display_options.level,
                        -255.0..=255.0,
                    ));
                    ui.end_row();

                    ui.label("Floor");
                    let floor_slider = ui.add(egui::Slider::new(
                        &mut self.image_display_options.floor,
                        0.0..=255.0,
                    ));
                    ui.end_row();

                    ui.label("Ceiling");
                    let ceiling_slider = ui.add(egui::Slider::new(
                        &mut self.image_display_options.ceiling,
                        0.0..=255.0,
                    ));
                    ui.end_row();

                    //// Constraints
                    self.image_display_options.apply_constraints();

                    //// Update Image with new values
                    if window_slider.changed()
                        || level_slider.changed()
                        || floor_slider.changed()
                        || ceiling_slider.changed()
                    {
                        self.image2d.request_redraw();
                    }
                });
        });
    }
}

//fn draw_histogram(
//    ui: &mut egui::Ui,
//    histogram: &[usize],
//    size: (f32, f32),
//    image_display_options: &mut ImageDisplayOptions,
//)-> {
//    lerp_ui(
//        ui,
//        &histogram,
//        &mut image_display_options.floor,
//        &mut image_display_options.ceiling,
//    )

//let max = (1. + *histogram.iter().max().unwrap() as f64).log10();
//let left = image_display_options.floor as f64;
//let right = image_display_options.ceiling as f64;
//let line_min = 1.0;
//let line_max = (max - 1.0) as f64;
//let lines = Line::new(vec![
//    [0.0, line_min],
//    [left, line_min],
//    [right, line_max],
//    [255.0, line_max],
//])
//.width(2.0);
//
//let bars = histogram
//    .into_iter()
//    .enumerate()
//    .map(|(i, &x)| {
//        Bar::new(i as f64, (1. + x as f64).log10())
//            .stroke(Stroke {
//                width: 0.01,
//                color: Color32::WHITE,
//            })
//            .fill(Color32::WHITE)
//            .width(0.8)
//    })
//    .collect();
//let bar_chart: BarChart = BarChart::new(bars);
//egui::Frame::default()
//    //.inner_margin(1.0)
//    //.fill(egui::Color32::GRAY)
//    //.show(ui, |ui| {
//    //    egui::Frame::default()
//    //        .inner_margin(1.0)
//    //        .fill(egui::Color32::BLACK)
//    .show(ui, |ui| {
//        Plot::new("my_plot")
//            .width(size.0)
//            .height(size.1)
//            .allow_zoom(false)
//            .allow_scroll(false)
//            .allow_drag(false)
//            .show_x(false)
//            .show_y(false)
//            .clamp_grid(true)
//            .show_background(false)
//            .show_axes(false)
//            .show_grid(false)
//            .set_margin_fraction((0., 0.).into())
//            //.x_grid_spacer(|_grid_input| {
//            //    vec![GridMark {
//            //        value: 0.0,
//            //        step_size: 10.0,
//            //    }]
//            //})
//            //.y_grid_spacer(|_grid_input| {
//            //    vec![GridMark {
//            //        value: 0.0,
//            //        step_size: 10.0,
//            //    }]
//            //})
//            .show(ui, |plot_ui| {
//                plot_ui.bar_chart(bar_chart);
//                plot_ui.line(lines)
//            });
//        //        });
//    });
//}

fn draw_histogram(ui: &mut Ui, histogram: &[usize], floor: &mut f32, max: &mut f32) -> bool {
    let width = ui.available_width();
    let height = 200.0;
    let (response, painter) = ui.allocate_painter(Vec2::new(width, height), Sense::hover());

    let to_screen = emath::RectTransform::from_to(
        Rect::from_min_size(Pos2::ZERO, response.rect.size()),
        response.rect,
    );

    //// Histogram
    let histogram_max = *histogram.iter().max().unwrap_or(&1);
    histogram
        .iter()
        .map(|&c| (c as f32 / histogram_max as f32) * height)
        .enumerate()
        .map(|(i, y)| Pos2::new(i as f32 / 255. * width, y))
        .map(|p| [p, Pos2 { x: p.x, y: 0.0 }])
        .map(|l| [to_screen.transform_pos(l[0]), to_screen.transform_pos(l[1])])
        .for_each(|l| {
            painter.add(Shape::line_segment(
                l,
                Stroke {
                    width: 1.0,
                    color: Color32::WHITE,
                },
            ));
        });

    //// Lerp Layer
    let line_inset = 10.0;
    let p0 = (0., height - line_inset).into();
    let p1 = (*floor, height - line_inset).into();
    let p2 = (*max, line_inset).into();
    let p3 = (width, line_inset).into();
    let point_id_1 = response.id.with(1);
    let point_id_2 = response.id.with(2);

    painter.add(Shape::line(
        [p0, p1, p2, p3]
            .iter()
            .map(|p| to_screen.transform_pos(*p))
            .collect(),
        Stroke {
            color: Color32::WHITE,
            width: 2.0,
        },
    ));

    let mut request_redraw = false;

    if let Some(d) = draw_drag_point(ui, &painter, point_id_1, p1, to_screen) {
        *floor += d.x;
        request_redraw = true;
    };
    if let Some(d) = draw_drag_point(ui, &painter, point_id_2, p2, to_screen) {
        *max += d.x;
        request_redraw = true;
    };

    request_redraw
}

fn draw_drag_point(
    ui: &mut Ui,
    painter: &Painter,
    id: Id,
    pos: Pos2,
    to_screen: RectTransform,
) -> Option<Vec2> {
    let point_in_screen = to_screen.transform_pos(pos);
    let point_rect = Rect::from_center_size(point_in_screen, (20., 20.).into());

    let point_response = ui.interact(point_rect, id, Sense::drag());

    let point1 = to_screen.from().clamp(pos);

    let point_in_screen = to_screen.transform_pos(point1);
    //let stroke = ui.style().interact(&point_response).fg_stroke;

    let shape = Shape::circle_filled(point_in_screen, 10., Color32::GRAY);

    //to_screen * point1;

    painter.add(shape);
    (point_response.drag_delta().length_sq() > 0.).then(|| point_response.drag_delta())
}
