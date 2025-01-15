use super::PortData;
use crate::app::Message;
use crate::interface::node::{INNER_NODE_WIDTH, NODE_BORDER_WIDTH};
use crate::math::Vector;
use crate::nodes::NodeTemplate;
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
        input_data: OrderMap<String, &RefCell<PortData>>,
    ) -> Element<'a, Message> {
        let data = if let Some(port) = input_data.get("a") {
            match port.borrow().clone() {
                PortData::Real2d(a) => a,
                PortData::Complex2d(a) => Array2::<f64>::from_shape_vec(
                    (a.len().isqrt(), a.len().isqrt()),
                    a.iter().map(|v| v.norm_sqr().sqrt()).collect::<Vec<_>>(),
                )
                .expect("square matrix"),
                _ => panic!("unsuported plot types {:?}", port),
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
        _input_data: OrderMap<String, &RefCell<PortData>>,
    ) -> Option<Element<'a, Message>> {
        let center = self.rect.center;
        let width = self.rect.width;
        let height = self.rect.height;
        let message =
            move |rect| Message::UpdateNodeTemplate(id, NodeTemplate::Plot2D(Self { rect }));
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
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let node_width = bounds.width;
        let node_height = bounds.height;

        frame.push_transform();
        //// center canvas on the origin, y up
        frame.translate([frame.center().x, frame.center().y].into());

        //// scale for the conifgured height/width
        frame.scale_nonuniform([
            node_width / self.config.rect.width,
            node_height / self.config.rect.height,
        ]);
        ////move the center point to the center of our canvas
        frame.translate((-self.config.rect.center).into());

        let px_size = 1.0 / self.data.dim().0 as f32;
        let max = self.data.iter().fold(-f64::INFINITY, |a, &b| a.max(b));
        let min = self.data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        self.data
            .rows()
            .into_iter()
            .enumerate()
            .for_each(|(i, row)| {
                row.iter().enumerate().for_each(|(j, p)| {
                    let p = ((p - min) / (max - min)) as f32;
                    let p = if p.is_nan() { 1.0 } else { p };
                    frame.fill(
                        &Path::rectangle(
                            Point::new(px_size * i as f32, px_size * j as f32),
                            (px_size, px_size).into(),
                        ),
                        iced::Color::new(p, p, p, 1.0),
                    )
                })
            });
        frame.pop_transform();

        vec![frame.into_geometry()]
    }
}
