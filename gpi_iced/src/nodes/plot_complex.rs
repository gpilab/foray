use super::{PortData, INNER_NODE_WIDTH, NODE_BORDER_WIDTH};
use crate::app::Message;
use crate::math::Vector;
use crate::node_data::NodeData;
use crate::OrderMap;
use iced::widget::canvas::Path;
use iced::widget::{button, container, horizontal_space, row, text, text_input};
use iced::Alignment::Center;
use iced::{mouse, Point};
use iced::{
    widget::{canvas, column},
    Element,
};
use iced::{Rectangle, Renderer, Theme};
use ndarray::Array2;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

// Rectanlge specified by center position, width and height
// y is up
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Rect {
    pub center: Vector,
    pub width: f32,
    pub height: f32,
}
impl Rect {
    pub fn right(&self) -> f32 {
        self.center.x + self.width / 2.
    }
    pub fn left(&self) -> f32 {
        self.center.x - self.width / 2.
    }
    pub fn top(&self) -> f32 {
        self.center.y + self.height / 2.
    }
    pub fn bottom(&self) -> f32 {
        self.center.y - self.height / 2.
    }
}
impl Default for Rect {
    fn default() -> Self {
        Rect {
            center: [0.5, 0.5].into(),
            width: 1.,
            height: 1.,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Plot2D {
    rect: Rect,
}

impl Plot2D {
    pub fn view<'a>(
        &self,
        _id: u32,
        input_data: Option<OrderMap<String, &RefCell<PortData>>>,
    ) -> Element<'a, Message> {
        let data = if let Some(i) = input_data {
            if let Some(port) = i.get("a") {
                if let PortData::Real2d(a) = port.borrow().clone() {
                    a
                    //(a.to_vec().into_iter().map(|f| f as f32).collect(),)
                } else {
                    panic!("unsuported plot types ")
                }
            } else {
                Array2::zeros((0, 0))
            }
        } else {
            Array2::zeros((0, 0))
        };
        container(
            canvas(PlotCanvas {
                data,
                config: *self,
            })
            .width(INNER_NODE_WIDTH * 2.)
            .height(INNER_NODE_WIDTH * 2.),
        )
        .padding(NODE_BORDER_WIDTH)
        .into()
    }

    pub fn config_view<'a>(
        &'a self,
        id: u32,
        _input_data: Option<OrderMap<String, &RefCell<PortData>>>,
    ) -> Option<Element<'a, Message>> {
        let center = self.rect.center;
        let width = self.rect.width;
        let height = self.rect.height;
        let message = move |rect| Message::UpdateNodeData(id, NodeData::Plot2D(Self { rect }));
        let zoom_speed = 0.125;
        Some(
            column![
                row![
                    text("center:"),
                    horizontal_space(),
                    text("x"),
                    text_input("0", &center.x.to_string()).on_input(move |value| {
                        let mut n = self.rect;
                        n.center.x = value.parse().unwrap_or(0.);
                        message(n)
                    }),
                    text("y"),
                    text_input("0", &center.y.to_string()).on_input(move |value| {
                        let mut n = self.rect;
                        n.center.y = value.parse().unwrap_or(0.);
                        message(n)
                    }),
                ]
                .align_y(Center)
                .spacing(4.),
                row![
                    text("width:"),
                    horizontal_space(),
                    text_input("0", &width.to_string()).on_input(move |value| {
                        let mut n = self.rect;
                        n.width = value.parse().unwrap_or(1.0f32).max(0.001);
                        message(n)
                    }),
                ]
                .align_y(Center),
                row![
                    text("height:"),
                    horizontal_space(),
                    text_input("0", &height.to_string()).on_input(move |value| {
                        let mut n = self.rect;
                        n.height = value.parse().unwrap_or(1.0f32).max(0.001);
                        message(n)
                    }),
                ]
                .align_y(Center),
                row![
                    horizontal_space(),
                    button("+").on_press_with(move || {
                        let mut n = self.rect;
                        let aspect = n.width / n.height;
                        n.height -= zoom_speed;
                        n.height = n.height.max(0.01);
                        n.width -= zoom_speed * aspect;
                        n.width = n.width.max(0.1 * aspect);
                        message(n)
                    }),
                    button("-").on_press_with(move || {
                        let mut n = self.rect;
                        let aspect = n.width / n.height;
                        n.height += zoom_speed;
                        n.width += zoom_speed * aspect;
                        message(n)
                    }),
                ]
                .spacing(5.0)
                .align_y(Center)
            ]
            .spacing(5.0)
            .into(),
        )
    }
}

#[derive(Debug)]
struct PlotCanvas {
    data: Array2<f64>,
    config: Plot2D,
}

impl<Message> canvas::Program<Message> for PlotCanvas {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let node_width = bounds.width;
        let node_height = bounds.height;

        frame.push_transform();
        ////// center canvas on the origin, y up
        frame.translate([frame.center().x, frame.center().y].into());
        //frame.scale_nonuniform([1., -1.]);

        ////// scale for the conifgured height/width
        frame.scale_nonuniform([
            node_width / self.config.rect.width,
            node_height / self.config.rect.height,
        ]);
        //
        ////move the center point to the center of our canvas
        frame.translate((-self.config.rect.center).into());

        // The frame is now centered on center, and goes from:
        // rect.left   -> rect.right
        // rect.bottom -> rect.top

        //// Grid
        //{
        //    let main_grid_stroke = Stroke::default()
        //        .with_color(theme.palette().primary.scale_alpha(0.3))
        //        .with_width(1.);
        //
        //    let secondary_grid_stroke = Stroke::default()
        //        .with_color(theme.palette().primary.scale_alpha(0.1))
        //        .with_width(1.);
        //    let tertiary_grid_strok = Stroke::default()
        //        .with_color(theme.palette().primary.scale_alpha(0.01))
        //        .with_width(1.);
        //
        //    grid_path(self.config.rect, 100.)
        //        .into_iter()
        //        .for_each(|p| frame.stroke(&p, main_grid_stroke));
        //
        //    grid_path(self.config.rect, 10.)
        //        .into_iter()
        //        .for_each(|p| frame.stroke(&p, secondary_grid_stroke));
        //
        //    grid_path(self.config.rect, 1.)
        //        .into_iter()
        //        .for_each(|p| frame.stroke(&p, tertiary_grid_strok));
        //}

        //let lixne_stroke = Stroke::default()
        //    .with_color(theme.extended_palette().success.strong.color)
        //
        //    .with_width(2.);
        let px_size = 1.0 / self.data.dim().0 as f32;
        self.data
            .rows()
            .into_iter()
            .enumerate()
            .for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, p)| {
                    frame.fill(
                        &Path::rectangle(
                            Point::new(px_size * i as f32, px_size * j as f32),
                            (px_size, px_size).into(),
                        ),
                        iced::Color::new(*p as f32, *p as f32, *p as f32, 1.0),
                    )
                })
            });
        //.into_iter()
        //.zip(self.y.clone())
        //.map_windows(|[from, to]| {
        //    if from.0.is_finite() && from.1.is_finite() && to.0.is_finite() && to.1.is_finite()
        //    {
        //        (
        //            Path::line(Point::from(*from), Point::from(*to)),
        //            line_stroke,
        //        )
        //    } else if from.0.is_finite() && to.0.is_finite() {
        //        (
        //            Path::line(
        //                Point::from((from.0, self.config.rect.center.y)),
        //                Point::from((to.0, self.config.rect.center.y)),
        //            ),
        //            line_stroke.with_color(theme.palette().danger),
        //        )
        //    } else {
        //        (
        //            Path::circle(
        //                Point::from((
        //                    self.config.rect.right() - 1.,
        //                    self.config.rect.top() - 1.,
        //                )),
        //                0.75,
        //            ),
        //            line_stroke.with_color(theme.palette().danger),
        //        )
        //    }
        //})
        //.for_each(|(path, stroke)| frame.stroke(&path, stroke));
        //
        frame.pop_transform();

        vec![frame.into_geometry()]
    }
}

//fn grid_path(plot_rect: Rect, tick_size: f32) -> Vec<Path> {
//    let left = ((plot_rect.left() / tick_size).floor()) * tick_size;
//    let right = ((plot_rect.right() / tick_size).ceil()) * tick_size;
//    let bottom = ((plot_rect.bottom() / tick_size).floor()) * tick_size;
//    let top = ((plot_rect.top() / tick_size).ceil()) * tick_size;
//
//    if left.is_nan() || right.is_nan() || top.is_nan() || bottom.is_nan() {
//        panic!("Encountered nan!{:?}", (plot_rect, tick_size))
//    }
//
//    let h_lines = linspace_delta(top, bottom, tick_size).into_iter().map(|y| {
//        if y.is_nan() {
//            panic!("Encountered nan!{:?}", (plot_rect, tick_size))
//        }
//        Path::line((left, y).into(), (right, y).into())
//    });
//
//    let v_lines = linspace_delta(right, left, tick_size).into_iter().map(|x| {
//        if x.is_nan() {
//            panic!("Encountered nan!{:?}", (plot_rect, tick_size))
//        }
//
//        Path::line((x, bottom).into(), (x, top).into())
//    });
//
//    h_lines.chain(v_lines).collect()
//}
