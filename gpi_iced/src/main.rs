use gpi_iced::widget::draggable::draggable;
use iced::advanced::graphics::core::Element;
use iced::widget::{button, center, column, text};
use iced::{application, Center, Renderer, Theme};

pub fn main() -> iced::Result {
    application("gpi_v2", Example::update, Example::view)
        .antialiasing(true)
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
            status: "start".into(),
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
            draggable(button("hello").on_press(Message::ButtonPressed))
                .on_pickup(Message::OnPickup)
                .on_release(Message::OnRelease),
            text(self.status.clone())
        ]
        //.padding(20)
        //.spacing(20)
        .align_x(Center)
        .into();

        //button("outside 2 ").height(100.).width(200.),
        //button("outside 1").height(200.).width(400.),
        //workspace(vec![
        //    button("hi").height(100.).width(200.).into(),
        //    button("how are you?").height(200.).width(400.).into()
        //]),
        //text!("Radius: {:.2}", self.radius),
        //slider(1.0..=100.0, self.radius, Message::RadiusChanged).step(0.01),

        center(content.explain(iced::Color::BLACK)).into()
    }
}

impl Default for Example {
    fn default() -> Self {
        Self::new()
    }
}
