use gpi_iced::widget::shapes::ShapeId;
use gpi_iced::widget::workspace::{self, workspace};
use iced::advanced::graphics::core::Element;
use iced::border::rounded;
use iced::widget::{container, text};
use iced::{application, Length, Point, Renderer, Theme, Vector};

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
#[derive(Default)]
struct Node {
    name: String,
}
struct Example {
    shapes: workspace::State<Node>,
}

impl Default for Example {
    fn default() -> Self {
        Self {
            shapes: workspace::State::<Node>::new(vec![
                (
                    Point::new(100., 100.),
                    Node {
                        name: "reduce".into(),
                    },
                ),
                (Point::new(200., 300.), Node { name: "sum".into() }),
            ]),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {
    OnDrag(ShapeId, Point),
    Pan(Vector),
}

impl Example {
    fn update(&mut self, message: Message) {
        match message {
            Message::OnDrag(shape_index, cursor_position) => {
                self.shapes
                    .shapes
                    .0
                    .get_mut(&shape_index)
                    .expect("Shape index must exist")
                    .position = cursor_position
            }
            Message::Pan(delta) => {
                self.shapes.camera.position.x -= delta.x * 2.;
                self.shapes.camera.position.y -= delta.y * 2.;
            }
        };
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        container(
            workspace(&self.shapes, |_id, node| {
                container(text(&node.name))
                    .padding([20., 40.])
                    .style(|t| {
                        container::bordered_box(t).border(
                            rounded(5.)
                                .color(t.extended_palette().secondary.weak.color)
                                .width(2.),
                        )
                    })
                    .into()
            })
            .on_shape_drag(Message::OnDrag)
            .pan(Message::Pan),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
    }
}
