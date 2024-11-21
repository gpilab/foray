//! This example showcases a simple native custom widget that draws a circle.
use iced::advanced::graphics::core::widget;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::Tree;
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
    elements: Vec<(Point, Element<'a, Message, Theme, Renderer>)>,
    action: Action,
}

impl<'a, Message, Theme, Renderer> Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    pub fn new(elements: Vec<(Point, Element<'a, Message, Theme, Renderer>)>) -> Self {
        Self {
            elements,
            action: Action::Idle,
        }
    }
}

//// Implement Widet
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Workspace<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(400.),
            height: Length::Fixed(400.),
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.elements
            .iter()
            .map(|(_, content)| Tree::new(content.as_widget()))
            .collect()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::with_children(
            //// Fill the screen
            limits.resolve(
                Length::Fixed(400.),
                Length::Fixed(400.),
                Size::new(50., 50.),
            ),
            ///// Layout child elements
            self.elements
                .iter()
                .zip(&mut tree.children)
                .map(|((p, e), mut t)| {
                    e.as_widget()
                        .layout(&mut t, renderer, limits)
                        //// Move them to their location here!
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
        //// Background
        renderer.fill_quad(
            renderer::Quad {
                bounds: workspace_layout.bounds(),
                border: iced::Border::default().width(5.).color(iced::Color::WHITE),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
        //// Render Children in a layer that is bounded to the size of the workspace
        renderer.with_layer(workspace_layout.bounds(), |renderer| {
            let elements = self.elements.iter().zip(&tree.children);
            for (((_p, e), tree), c_layout) in elements.zip(workspace_layout.children()) {
                e.as_widget()
                    .draw(&tree, renderer, theme, style, c_layout, cursor, viewport)
            }
        });
    }

    //// Move children based on input events
    fn on_event(
        &mut self,
        _tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        //let state = tree.state.downcast_mut::<InnerState>();
        //let bounds = state.bounds(layout);
        //let bounds = layout.bounds();

        match event {
            Event::Mouse(ButtonPressed(mouse::Button::Left))
            | Event::Touch(FingerPressed { .. }) => {
                //dbg!(&event);
                //dbg!(&state);
                //tree.children.into_iter().find(|n| )
                //self.elements.iter().find(|(p,e)| e.)
                if let Some((child_index, child_position, cursor_position)) = layout
                    .children()
                    .zip(&mut self.elements.iter().map(|(p, _)| p))
                    .enumerate()
                    .find_map(|(i, (l, e_p))| match cursor.position_over(l.bounds()) {
                        Some(cursor_position) => Some((i, e_p, cursor_position)),
                        None => None,
                    })
                {
                    self.action = Action::Dragging {
                        start_pos: *child_position,
                        shape_offset: cursor_position - *child_position,
                        child_index,
                    };
                    //// fire event
                    //TODO: publish events for the workspace so users can respond
                    //if let Some(on_pickup) = self.on_pickup.clone() {
                    //    shell.publish(on_pickup);
                    //}

                    //// update state
                    //state.action = Action::Dragging {
                    //    start_pos: state.position,
                    //    shape_offset: cursor_position - state.position,
                    //};

                    //// end event propogation?
                    dbg!("captured pickup");
                    return event::Status::Captured;
                }
            }
            Event::Mouse(ButtonReleased(mouse::Button::Left))
            | Event::Touch(FingerLifted { .. })
            | Event::Touch(FingerLost { .. }) => {
                if let Action::Dragging { .. } = self.action {
                    //// fire event
                    //if let Some(on_release) = self.on_release.clone() {
                    //    shell.publish(on_release);
                    //}

                    //// update state
                    self.action = Action::Idle;
                    //
                    dbg!("captured release");
                    //// end event propogation?
                    return event::Status::Captured;
                }
            }
            Event::Mouse(CursorMoved { .. }) | Event::Touch(FingerMoved { .. }) => {
                if let Action::Dragging {
                    start_pos: _,
                    shape_offset,
                    child_index,
                } = self.action
                {
                    //// update state
                    if let Some(cursor_position) = cursor.position() {
                        // force a relayout, so that inner widgets will move relative to the new position
                        shell.invalidate_layout(); // unsure if this should be invalidate_widgets,
                                                   // or invalidate_layout

                        self.elements[child_index].0.x = cursor_position.x - (shape_offset.x);
                        self.elements[child_index].0.y = cursor_position.y - (shape_offset.y);

                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }
}

/// Convert to an element
impl<'a, Message, Theme, Renderer> From<Workspace<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
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
