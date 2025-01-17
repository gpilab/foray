use std::iter::once;

use crate::app::{Action, App};
use crate::math::Point;
use crate::style::theme::AppTheme;
use crate::OrderMap;
use canvas::{Path, Stroke};
use iced::widget::*;

impl App {
    pub fn wire_curve(
        &self,
        wire_end_node: u32,
        points: &OrderMap<u32, Point>,
    ) -> Vec<(Path, Stroke)> {
        let port_position = |port: &PortRef| {
            points[&port.node] + find_port_offset(port, self.graph.port_index(port)).into()
        };

        //// Handle currently active wire
        // TODO: test nodes with multiple out ports
        let active_wire = match &self.action {
            Action::CreatingInputWire(input, Some(tentative_output)) => {
                Some((port_position(input), port_position(tentative_output)))
            }
            Action::CreatingInputWire(input, None) => Some((
                port_position(input),
                self.cursor_position + self.shapes.camera.position,
            )),
            Action::CreatingOutputWire(output, Some(input)) => {
                Some((port_position(input), port_position(output)))
            }
            Action::CreatingOutputWire(output, None) => Some((
                self.cursor_position + self.shapes.camera.position,
                port_position(output),
            )),
            _ => None,
        };

        //// Handle all wires
        let incoming_wires = self.graph.incoming_edges(&wire_end_node);
        incoming_wires
            .iter()
            .map(|(from, to)| {
                let color = wire_status(from, to, &self.action, &self.app_theme);
                ((port_position(to), port_position(from)), color)
            })
            //// include the active wire
            .chain(once(active_wire.map(|w| (w, active_wire_color(&self.app_theme)))).flatten())
            //// build the wire curves
            .map(|((from, to), color)| {
                (
                    Path::new(|builder| {
                        builder.move_to(from.into());
                        let mid = f32::abs((to.y - from.y) * 0.5).max(PORT_RADIUS * 2.);
                        builder.bezier_curve_to(
                            (from.x, from.y - mid).into(),
                            (to.x, to.y + mid).into(),
                            to.into(),
                        );
                    }),
                    Stroke::default()
                        .with_width(3.0)
                        .with_color(color)
                        .with_line_cap(canvas::LineCap::Round),
                )
            })
            .collect()
    }
}

use super::node::{INNER_NODE_HEIGHT, INNER_NODE_WIDTH, NODE_RADIUS, PORT_RADIUS};
use crate::{
    app,
    graph::{PortRef, IO},
};
use iced::Vector;

/// Determine where a port should be positioned relative to the origin of the node
pub fn find_port_offset(port_ref: &PortRef, port_index: usize) -> Vector {
    let port_x = |i: usize| i as f32 * (INNER_NODE_WIDTH / 4.) + NODE_RADIUS * 2.;
    match port_ref.io {
        IO::In => Vector::new(port_x(port_index), 0.) + Vector::new(PORT_RADIUS, -PORT_RADIUS / 2.),
        IO::Out => {
            Vector::new(port_x(port_index), INNER_NODE_HEIGHT)
                + Vector::new(PORT_RADIUS, PORT_RADIUS / 2.)
        }
    }
}

/// Determine the status of a given *non-active* wire, and provide the corresponding color
/// The current action determines how existing wires should be displayed, to provide
/// context about how the current action will affect other wires
pub fn wire_status(
    output: &PortRef,
    input: &PortRef,
    current_action: &app::Action,
    theme: &AppTheme,
) -> iced::Color {
    assert!(output.io == IO::Out);
    assert!(input.io == IO::In);

    //let p = theme.extended_palette();

    let default_color = theme.secondary.base_color;
    let maybe_delete = theme.danger.weak_color();
    let will_delete = theme.danger.base_color;

    match current_action {
        app::Action::CreatingInputWire(active_input, active_output) => {
            //// if a new wire is created at an input, any existing wires will be deleted
            if active_input == input {
                //// differentiate between if the new wire is complete, and a MouseUp event
                //// would trigger wire deletion
                if active_output.is_some() {
                    will_delete
                } else {
                    maybe_delete
                }
            } else {
                default_color
            }
        }
        app::Action::CreatingOutputWire(_, None) => default_color,
        app::Action::CreatingOutputWire(_, Some(active_input)) => {
            //// if a new wire is created at an input, any existing wires will be deleted
            if active_input == input {
                will_delete
            } else {
                default_color
            }
        }
        app::Action::Idle => default_color,
        _ => default_color,
    }
    .into()
}

/// active wire color
pub fn active_wire_color(t: &AppTheme) -> iced::Color {
    t.secondary.strong_color().into()
}
