use iced::{Theme, Vector};

use crate::{
    app,
    graph::{PortRef, IO},
    node::{NODE_HEIGHT, NODE_RADIUS, NODE_WIDTH, PORT_RADIUS},
};

/// Determine where a port should be positioned relative to the origin of the node
pub fn find_port_offset(port_ref: &PortRef, port_index: usize) -> Vector {
    let port_x = |i: usize| i as f32 * (NODE_WIDTH / 4.) + NODE_RADIUS * 2.;
    match port_ref.io {
        IO::In => Vector::new(port_x(port_index), 0.) + Vector::new(PORT_RADIUS, -PORT_RADIUS / 2.),
        IO::Out => {
            Vector::new(port_x(port_index), NODE_HEIGHT)
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
    theme: &Theme,
) -> iced::Color {
    assert!(output.io == IO::Out);
    assert!(input.io == IO::In);

    let p = theme.extended_palette();

    let default_color = p.secondary.base.color;
    let maybe_delete = p.danger.weak.color;
    let will_delete = p.danger.base.color;

    match current_action {
        app::Action::Idle => default_color,
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
    }
}

/// active wire color
pub fn active_wire_color(t: &Theme) -> iced::Color {
    t.extended_palette().secondary.strong.color
}
