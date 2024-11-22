//! This example showcases a simple native custom widget that draws a circle.
use iced::advanced::graphics::core::widget;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{tree, Tree};
use iced::advanced::{renderer, Clipboard, Shell, Widget};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorMoved};
use iced::touch::Event::{FingerLifted, FingerLost, FingerMoved, FingerPressed};
use iced::widget::container::{self};
use iced::{event, mouse, Point, Vector};
use iced::{Color, Length, Rectangle, Size};
use iced::{Element, Event};

pub mod content;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Action {
    Dragging {
        /// where the drag started
        start_pos: Point,

        /// the intial offset between the shape's location and the cursor's location
        /// used to maintain a consititent relative position between the cursor
        /// and the shape throughout the drag event
        shape_offset: Vector,
        child_index: usize,
    },
    #[default]
    Idle,
}

/// A workspace is a an infinite canvas that can be zoomed, panned,
/// and contains widgets that can be placed anywhere in 3d (stacking in Z)
pub struct Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    elements: Vec<Element<'a, Message, Theme, Renderer>>,
    on_pickup: Option<Message>,
    on_release: Option<Message>,
    inner_state: InnerState,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct InnerState {
    positions: Vec<Point>,
    action: Action,
}

impl<'a, Message, Theme, Renderer> Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    pub fn new(children: Vec<(Point, Element<'a, Message, Theme, Renderer>)>) -> Self {
        let (positions, elements) = children.into_iter().unzip();
        Self {
            elements,
            inner_state: InnerState {
                positions,
                action: Action::Idle,
            },
            on_pickup: None,
            on_release: None,
        }
    }
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    pub fn on_pickup(mut self, on_pickup: Message) -> Self {
        self.on_pickup = Some(on_pickup);
        self
    }
}

//// Implement Widet
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Workspace<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<InnerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(self.inner_state.clone())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.elements
            .iter()
            .map(|content| Tree::new(content.as_widget()))
            .collect()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let positions = &tree.state.downcast_mut::<InnerState>().positions;
        layout::Node::with_children(
            //// Fill the screen
            limits.resolve(Length::Fill, Length::Fill, Size::new(50., 50.)),
            ///// Layout child elements
            self.elements
                .iter()
                .zip(&mut tree.children)
                .zip(positions)
                .map(|((e, mut t), p)| {
                    //let position = t.state.downcast_mut::<InnerState>();
                    e.as_widget()
                        .layout(&mut t, renderer, limits)
                        //// Move children to their location here!
                        .move_to(*p)
                })
                .collect(),
        )
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        workspace_layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let padding = 5.0;
        //// Background
        renderer.fill_quad(
            renderer::Quad {
                bounds: workspace_layout.bounds(),
                border: iced::Border::default()
                    .width(padding)
                    .color(iced::Color::WHITE),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
        //// Render Children in a layer that is bounded to the size of the workspace
        renderer.with_layer(workspace_layout.bounds().shrink(padding), |renderer| {
            let elements = self.elements.iter().zip(&tree.children);
            for ((e, tree), c_layout) in elements.zip(workspace_layout.children()) {
                e.as_widget()
                    .draw(&tree, renderer, theme, style, c_layout, cursor, viewport)
            }
        });
    }

    //// Move children based on input events
    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        //let state = tree.state.downcast_mut::<InnerState>();
        //let bounds = state.bounds(layout);
        //let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<InnerState>();

        let event_status = match (event.clone(), state.action, cursor.position()) {
            ////Pickup
            (
                Event::Mouse(ButtonPressed(mouse::Button::Left))
                | Event::Touch(FingerPressed { .. }),
                Action::Idle,
                Some(_),
            ) => {
                if let Some((child_index, child_position, cursor_position)) = layout
                    .children()
                    .zip(&state.positions)
                    .enumerate()
                    .find_map(|(i, (l, e_p))| match cursor.position_over(l.bounds()) {
                        Some(cursor_position) => Some((i, e_p, cursor_position)),
                        None => None,
                    })
                {
                    state.action = Action::Dragging {
                        start_pos: *child_position,
                        shape_offset: cursor_position - *child_position,
                        child_index,
                    };
                    //// fire event
                    if let Some(on_pickup) = self.on_pickup.clone() {
                        shell.publish(on_pickup);
                    }

                    //// update state
                    //state.action = Action::Dragging {
                    //    start_pos: state.position,
                    //    shape_offset: cursor_position - state.position,
                    //};

                    //// end event propogation?
                    dbg!("captured pickup");
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            ////Release
            (
                Event::Mouse(ButtonReleased(mouse::Button::Left))
                | Event::Touch(FingerLifted { .. })
                | Event::Touch(FingerLost { .. }),
                Action::Dragging { .. },
                _,
            ) => {
                if let Action::Dragging { .. } = state.action {
                    //// fire event
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }

                    //// update state
                    state.action = Action::Idle;
                    //
                    dbg!("captured release");
                    //// end event propogation?
                    event::Status::Captured
                } else {
                    event::Status::Ignored
                }
            }
            ////Drag
            (
                Event::Mouse(CursorMoved { .. }) | Event::Touch(FingerMoved { .. }),
                Action::Dragging {
                    start_pos: _,
                    shape_offset,
                    child_index,
                },
                Some(cursor_position),
            ) => {
                //// update state
                // force a relayout, so that inner widgets will move relative to the new position
                shell.invalidate_layout(); // unsure if this should be invalidate_widgets,
                                           // or invalidate_layout

                state.positions[child_index].x = cursor_position.x - (shape_offset.x);
                state.positions[child_index].y = cursor_position.y - (shape_offset.y);

                event::Status::Captured
            }

            _ => event::Status::Ignored,
        };

        ////Pass event down to children
        self.elements
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((element, tree), layout)| {
                element.as_widget_mut().on_event(
                    tree,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            })
            .fold(event_status, event::Status::merge)
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let action = tree.state.downcast_ref::<InnerState>().action;

        match action {
            Action::Dragging { .. } => mouse::Interaction::Grabbing,
            Action::Idle => {
                if layout
                    .children()
                    .find(|l| cursor.position_over(l.bounds()).is_some())
                    .is_some()
                {
                    //TODO: get mouse status of children?
                    mouse::Interaction::Grab
                } else {
                    mouse::Interaction::default()
                }
            }
        }
    }
}

/// Convert to an element
impl<'a, Message, Theme, Renderer> From<Workspace<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(
        workspace: Workspace<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Self::new(workspace)
    }
}

// Convenience function
/// Create a new `Workspace`
pub fn workspace<'a, Message, Theme, Renderer>(
    elements: Vec<(Point, Element<'a, Message, Theme, Renderer>)>,
) -> Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    Workspace::new(elements)
}
