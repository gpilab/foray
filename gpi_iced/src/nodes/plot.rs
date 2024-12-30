use std::cell::RefCell;

use iced::widget::canvas::{Path, Stroke};
use iced::widget::container;
use iced::Length::Fill;
use iced::{mouse, Color, Point};
use iced::{widget::canvas, Element};
use iced::{Rectangle, Renderer, Theme};
use ordermap::OrderMap;
use smol_str::SmolStr;

use crate::math::linspace_delta;
use crate::{app::Message, graph::GraphNode};

use super::NetworkNode;
use super::{Node, PortData, PortType, NODE_BORDER_WIDTH};

pub fn node() -> NetworkNode {
    GraphNode::new(
        Node::Plot,
        vec![("x", &PortType::Real), ("y", &PortType::Real)],
        vec![("out", &PortType::Real)],
        Box::new(|_inputs, _| [].into()),
    )
}
pub fn view<'a>(
    _id: u32,
    input_data: Option<OrderMap<SmolStr, &RefCell<PortData>>>,
) -> Element<'a, Message> {
    let (x, y) = if let Some(i) = input_data {
        if let (Some(x_port), Some(y_port)) = (i.get("x"), i.get("y")) {
            if let (PortData::Real(x), PortData::Real(y)) =
                (x_port.borrow().clone(), y_port.borrow().clone())
            {
                (
                    x.to_vec().into_iter().map(|f| f as f32).collect(),
                    y.to_vec().into_iter().map(|f| f as f32).collect(),
                )
            } else {
                panic!("unsuported plot types ") //, //input_data.clone())
            }
        } else {
            (vec![], vec![])
        }
    } else {
        (vec![], vec![])
    };
    container(canvas(Plot { x, y }).width(Fill).height(Fill))
        .padding(NODE_BORDER_WIDTH)
        .into()
}

#[derive(Debug)]
struct Plot {
    x: Vec<f32>,
    y: Vec<f32>,
}

impl<Message> canvas::Program<Message> for Plot {
    // No internal state
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        // We prepare a new `Frame`
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let width = bounds.width;
        let height = bounds.height;

        frame.push_transform();

        //// center canvas
        frame.translate([frame.center().x, frame.center().y].into());
        frame.scale_nonuniform([1., -1.]);

        //// Grid
        {
            let main_grid_stroke = Stroke::default()
                .with_color(theme.palette().primary.scale_alpha(0.5))
                .with_width(1.);

            let secondary_grid_stroke = Stroke::default()
                .with_color(theme.palette().primary.scale_alpha(0.1))
                .with_width(1.);
            let tertiary_grid_strok = Stroke::default()
                .with_color(theme.palette().primary.scale_alpha(0.01))
                .with_width(1.);

            let center = Point::new(0., 0.);

            grid_path(width, height, center, 500.)
                .into_iter()
                .for_each(|p| frame.stroke(&p, main_grid_stroke));

            grid_path(width, height, center, 30.)
                .into_iter()
                .for_each(|p| frame.stroke(&p, secondary_grid_stroke));

            grid_path(width, height, center, 10.)
                .into_iter()
                .for_each(|p| frame.stroke(&p, tertiary_grid_strok));
        }

        let line_stroke = Stroke::default().with_color(Color::WHITE).with_width(3.);
        self.x
            .clone()
            .into_iter()
            .zip(self.y.clone())
            .map_windows(|[from, to]| Path::line(Point::from(*from), Point::from(*to)))
            .for_each(|p| frame.stroke(&p, line_stroke));

        frame.pop_transform();

        vec![frame.into_geometry()]
    }
}

fn grid_path(width: f32, height: f32, center: Point, tick_size: f32) -> Vec<Path> {
    let left = center.x - width / 2.;
    let right = width + left;
    let bottom = center.y - height / 2.;
    let top = height + bottom;
    //dbg!((tick_size, ((left, right), (top, bottom))));

    let h_lines = linspace_delta(center.y, bottom, tick_size)
        .into_iter()
        .chain(linspace_delta(center.y, top, tick_size).into_iter().skip(1))
        .map(|y| Path::line((-width / 2., y).into(), (width / 2., y).into()));

    let v_lines = linspace_delta(center.x, left, tick_size)
        .into_iter()
        .chain(
            linspace_delta(center.x, right, tick_size)
                .into_iter()
                .skip(1),
        )
        .map(|x| Path::line((x, bottom).into(), (x, top).into()));

    h_lines.chain(v_lines).collect()
}
