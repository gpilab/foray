use crate::app::{App, Message};
use crate::interface::{char_icon_button, debug_format, SEPERATOR};
use crate::nodes::{self, format_node_output, GUINode};
use iced::*;
use widget::{column, *};

/// Create the sidebar view
pub fn side_bar(app: &App) -> Element<Message> {
    fn button_style(t: &Theme, s: button::Status) -> button::Style {
        let mut style = button::secondary(t, s);
        style.border.radius = border::radius(0.);
        style
    }

    let file_commands = row![
        horizontal_space(),
        button(text("New"))
            .on_press(Message::Config(20.))
            .padding([1.0, 4.0])
            .style(button_style),
        horizontal_space(),
        button(text("Load"))
            .on_press(Message::Load)
            .padding([1.0, 4.0])
            .style(button_style),
        horizontal_space(),
        button(text("Save"))
            .on_press(Message::Save)
            .padding([1.0, 4.0])
            .style(button_style),
        horizontal_space(),
        button(text("Dbg"))
            .padding([1.0, 4.0])
            .on_press(Message::ToggleDebug)
            .style(button_style),
    ]
    .spacing(2.0)
    .padding([5., 5.]);

    let undo = char_icon_button(
        debug_format(app.debug, &"", &app.undo_stack.len()),
        !app.undo_stack.is_empty(),
        Message::Undo,
    );
    let redo = char_icon_button(
        debug_format(app.debug, &"", &app.redo_stack.len()),
        !app.redo_stack.is_empty(),
        Message::Redo,
    );
    let action_commands = row![horizontal_space(), undo, redo]
        .spacing(2.0)
        .padding([5., 5.]);

    //// Config
    let config: Element<Message, Theme, Renderer> = if let Some(selected_id) = app.selected_shape {
        let node = app.graph.get_node(selected_id);
        let input_data = app.graph.get_input_data(&selected_id);
        let out_port_display = if app.debug {
            format_node_output(&app.graph.get_output_data(selected_id))
        } else {
            text("").into()
        };
        column![
            container(text(node.name().clone()).size(20.)).center_x(Fill),
            horizontal_rule(0),
            vertical_space().height(10.),
            node.config_view(selected_id, input_data)
                .unwrap_or(text("...").into()),
            vertical_space(),
            scrollable(out_port_display),
            row![button("delete node").on_press(Message::DeleteNode(selected_id))]
        ]
        .height(Fill)
        .spacing(5.)
        .padding([10., 5.])
        .into()
    } else {
        let node_list = nodes::available_nodes_view();
        column![
            container(text("Add Node").size(20.)).center_x(Fill),
            horizontal_rule(0),
            vertical_space().height(10.),
            scrollable(node_list)
        ]
        .spacing(5.)
        .padding([10., 5.])
        .into()
    };
    container(
        column![
            //// File
            file_commands.align_y(Alignment::Center).width(Fill),
            //// Actions
            horizontal_rule(SEPERATOR),
            action_commands.align_y(Alignment::Center).width(Fill),
            horizontal_rule(SEPERATOR),
            //// Config
            if app.debug {
                config.explain(Color::from_rgba(0.7, 0.7, 0.8, 0.2))
            } else {
                config
            }
        ]
        .height(Fill)
        .width(200.),
    )
    .into()
}
