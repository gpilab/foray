use gpi_iced::widget::node::Node;
use gpi_iced::widget::workspace::workspace;
use iced::advanced::graphics::core::Element;
use iced::widget::{button, column, container, text};
use iced::{application, Color, Length, Point, Renderer, Theme, Vector};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", Example::update, Example::view)
        .antialiasing(false)
        .theme(theme)
        .run()
}

fn theme(_state: &Example) -> Theme {
    Theme::Ferra
}

enum Action {
    Idle,
    Drag { offset: Vector },
}

struct Example {
    status: String,
    action: Action,
    position: Point,
    cursor_position: Point,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnPickup,
    OnWorkspaceCursorMove(Point),
    OnRelease,
    ButtonPressed,
}

impl Example {
    fn new() -> Self {
        Example {
            status: "start!".into(),
            position: (50., 50.).into(),
            action: Action::Idle,
            cursor_position: Point::ORIGIN,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OnPickup => {
                self.action = Action::Drag {
                    offset: self.cursor_position - self.position,
                }
            }
            Message::OnRelease => self.action = Action::Idle,
            Message::OnWorkspaceCursorMove(cursor_position) => {
                self.cursor_position = cursor_position;
                if let Action::Drag { offset } = self.action {
                    self.position = cursor_position - offset;
                    dbg!(self.position);
                }
            }
            Message::ButtonPressed => {
                self.status = "button pressed".to_owned();
            }
        }
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        column![workspace(vec![
            (
                Point::new(20., 10.),
                Node::new(
                    button("hi"),
                    vec![
                        Color::from_rgb8(200, 50, 50),
                        Color::from_rgb8(50, 200, 50),
                        Color::from_rgb8(50, 200, 50)
                    ],
                    vec![Color::from_rgb8(200, 50, 50),]
                )
                .into()
            ),
            (
                Point::new(20., 10.),
                container(column![
                    text("text"),
                    button("hi")
                        .height(50.)
                        .width(80.)
                        .on_press(Message::ButtonPressed)
                ])
                .padding(10.)
                .style(|theme: &Theme| {
                    let palette = theme.extended_palette();
                    container::Style::default().background(palette.background.strong.color)
                })
                .into()
            ),
            (
                Point::new(50., 50.),
                button("how are you?").height(50.).width(100.).into()
            )
        ])
        .on_pickup(Message::OnPickup)
        .on_move(Message::OnWorkspaceCursorMove)
        .on_release(Message::OnRelease),]
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}
