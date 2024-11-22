use gpi_iced::widget::workspace::workspace;
use iced::advanced::graphics::core::Element;
use iced::widget::{button, center, column, text};
use iced::{application, Center, Length, Point, Renderer, Theme};

pub fn main() -> iced::Result {
    #[cfg(target_arch = "wasm32")]
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    application("gpi_v2", Example::update, Example::view)
        .antialiasing(false)
        .theme(theme)
        .run()
}

fn theme(_state: &Example) -> Theme {
    Theme::TokyoNight
}

struct Example {
    status: String,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnPickup,
    OnRelease,
    ButtonPressed,
}

impl Example {
    fn new() -> Self {
        Example {
            status: "start!".into(),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::OnPickup => {
                self.status = "on pickup".to_owned();
            }
            Message::OnRelease => {
                self.status = "on release".to_owned();
            }
            Message::ButtonPressed => {
                self.status = "button pressed".to_owned();
            }
        }
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        let content: Element<Message, Theme, Renderer> = column![
            text(self.status.clone()).width(Length::Fill),
            workspace(vec![
                (
                    Point::new(20., 10.),
                    button("hi")
                        .height(50.)
                        .width(80.)
                        .on_press(Message::ButtonPressed)
                        .into()
                ),
                (
                    Point::new(50., 50.),
                    button("how are you?").height(50.).width(100.).into()
                )
            ])
            .on_pickup(Message::OnPickup)
            .on_release(Message::OnRelease),
        ]
        .align_x(Center)
        .into();

        center(content).into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}
