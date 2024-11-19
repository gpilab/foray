use eframe::{
    egui::{lerp, vec2, Color32, Pos2, Rect, Rounding, Sense, Shape, Stroke, Ui, Vec2},
    emath::{self},
};
//pub struct Histogram<'a> {
//    counts: &'a [usize],
//}
//
//impl<'a> Histogram<'a> {
//    pub fn new(counts: &'a [usize], floor: f32, ceil: f32) -> Self {
//        Self { counts }
//    }
//}

pub fn toggle_ui(ui: &mut Ui, histogram: &[usize], floor: &mut f32, ceil: &mut f32) -> bool {
    let desired_size = ui.available_size().clamp(vec2(255., 25.), vec2(600., 100.));
    let total_width = desired_size.x;
    let total_height = desired_size.y;
    let mut changed = false;

    ui.allocate_space(vec2(10.0, 0.0));
    let rect = Rect::from_min_size(ui.next_widget_position(), desired_size);
    ui.horizontal(|ui| {
        draw_histogram(ui, rect, histogram, *floor, *ceil);

        let floor_width = lerp(0.0..=total_width, *floor);
        let ceil_width = lerp(0.0..=total_width, 1.0 - *ceil);
        let grey_width = (total_width - (floor_width + ceil_width)).max(1.);

        //assert!(floor_width + ceil_width + grey_width == total_width);

        //let radius = ui.style().visuals.widgets.open.rounding;

        //// Make selectable drag regions
        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);

        let draw_region = |ui: &mut Ui, width: f32, fill: Color32, round: Rounding| {
            let (rect, response) = ui.allocate_exact_size(vec2(width, total_height), Sense::drag());
            ui.painter()
                .rect(rect, round, fill, ui.style().interact(&response).bg_stroke);
            response.drag_delta().x
        };

        let black_dim = Color32::from_black_alpha(64);
        let grey_dim = Color32::from_rgba_unmultiplied(128, 128, 128, 64);
        let white_dim = Color32::from_white_alpha(64);
        let da = draw_region(ui, floor_width.max(1.0), black_dim, Rounding::ZERO) / total_width;

        let db = draw_region(ui, grey_width.max(1.0), grey_dim, Rounding::ZERO) / total_width;

        let dc = draw_region(ui, ceil_width.max(1.0), white_dim, Rounding::ZERO) / total_width;

        let a = *floor;
        let b = grey_width / total_width;
        let c = 1.0 - *ceil;

        println!("da: {},db: {},dc: {}", da, db, dc);
        println!(
            "old a: {:.2},b: {:.2},c: {:.2},t: {:.2}",
            a,
            b,
            c,
            a + b + c
        );
        let mut new_a = a;
        let mut new_c = c;
        let mut new_b = b;
        if da != 0.0 {
            new_b = b - da;
            new_a = a + da;
            new_a = new_a.clamp(0.0, 1.0 - c);
            changed = true;
        } else if db != 0.0 {
            new_a = a + db;
            new_c = c - db;
            new_a = new_a.clamp(0.0, 1.0 - new_b);
            new_c = new_c.clamp(0.0, 1.0 - new_b);
            changed = true;
        } else if dc != 0.0 {
            new_b = b + dc;
            new_c = c - dc;
            new_c = new_c.clamp(0.0, 1.0 - a);
            changed = true;
        }
        *floor = new_a;
        *ceil = 1.0 - new_c;
        println!(
            "new a: {:.2},b: {:.2},c: {:.2},t: {:.2}",
            new_a,
            new_b,
            new_c,
            new_a + new_b + new_c
        )
    });
    changed
}

pub fn draw_histogram(ui: &mut Ui, rect: Rect, histogram: &[usize], floor: f32, max: f32) {
    let width = rect.width();
    let height = rect.height();
    let painter = ui.painter();

    let to_screen =
        emath::RectTransform::from_to(Rect::from_min_size(Pos2::ZERO, rect.size()), rect);

    //// Histogram
    let histogram_max = *histogram.iter().max().unwrap_or(&1);
    histogram
        .iter()
        .map(|&c| (c as f32 / histogram_max as f32) * height)
        .enumerate()
        .map(|(i, y)| Pos2::new(i as f32 / 255. * width, height - y))
        .map(|p| [p, Pos2 { x: p.x, y: height }])
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
    let p1 = (floor * width, height - line_inset).into();
    let p2 = (max * width, line_inset).into();
    let p3 = (width, line_inset).into();

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
}

//fn draw_drag_point(
//    ui: &mut Ui,
//    painter: &Painter,
//    id: Id,
//    pos: Pos2,
//    to_screen: RectTransform,
//) -> Option<Vec2> {
//    let point_in_screen = to_screen.transform_pos(pos);
//    let point_rect = Rect::from_center_size(point_in_screen, (20., 20.).into());
//
//    let point_response = ui.interact(point_rect, id, Sense::drag());
//
//    let point1 = to_screen.from().clamp(pos);
//
//    let point_in_screen = to_screen.transform_pos(point1);
//    //let stroke = ui.style().interact(&point_response).fg_stroke;
//
//    let shape = Shape::circle_filled(point_in_screen, 10., Color32::GRAY);
//
//    //to_screen * point1;
//
//    painter.add(shape);
//    (point_response.drag_delta().length_sq() > 0.).then(|| point_response.drag_delta())
//}
