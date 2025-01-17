use crate::app::{App, Message};
use crate::gui_node::GUINode;
use crate::interface::node::{format_node_output, node_list_view};
use crate::interface::{debug_format, SEPERATOR};
use crate::style::button::{primary_icon, secondary_icon};
use crate::SYMBOL_FONT;
use iced::*;
use widget::{column, *};

/// Create the sidebar view
pub fn side_bar(app: &App) -> Element<Message> {
    let file_button = move |lbl: String, message| {
        button(text(lbl).font(SYMBOL_FONT))
            .padding([1.0, 4.0])
            .on_press(message)
            .style(primary_icon)
    };
    fn undo_button<'a>(lbl: String, enabled: bool, message: Message) -> Element<'a, Message> {
        button(text(lbl).font(SYMBOL_FONT))
            .on_press_maybe(if enabled { Some(message) } else { None })
            .padding([1.0, 4.0])
            .style(secondary_icon)
            .into()
    }

    let file_commands = row![
        file_button("".into(), Message::Config(20.)),
        file_button("".into(), Message::Load),
        file_button("󰆓".into(), Message::Save),
        file_button("".into(), Message::ToggleDebug),
        file_button("󰏘".into(), Message::TogglePaletteUI),
    ]
    .spacing(4.0);

    let undo = undo_button(
        debug_format(app.debug, &"", &app.undo_stack.len()),
        !app.undo_stack.is_empty(),
        Message::Undo,
    );
    let redo = undo_button(
        debug_format(app.debug, &"", &app.redo_stack.len()),
        !app.redo_stack.is_empty(),
        Message::Redo,
    );
    let action_commands = row![horizontal_space(), undo, redo].spacing(4.0);

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
            container(text(node.template.name().clone()).size(20.)).center_x(Fill),
            horizontal_rule(0),
            column![node.status.icon(), node.status.text_element().size(12.),],
            vertical_space().height(10.),
            node.template
                .config_view(selected_id, input_data)
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
        let node_list = node_list_view(&app.availble_nodes);
        column![
            container(text("Add Node").size(20.)).center_x(Fill),
            horizontal_rule(0),
            vertical_space().height(10.),
            row![vertical_rule(SEPERATOR), scrollable(node_list)],
        ]
        .spacing(5.)
        .padding([10., 5.])
        .into()
    };
    container(
        column![
            row![
                //// File
                file_commands.align_y(Alignment::Center),
                horizontal_space(),
                //// Actions
                action_commands.align_y(Alignment::Center),
            ]
            .padding([2., 4.]),
            horizontal_rule(SEPERATOR),
            //// Config
            config
        ]
        .height(Fill)
        .width(200.),
    )
    .into()
}
