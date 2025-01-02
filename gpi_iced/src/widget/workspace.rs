use iced::advanced::graphics::geometry::frame::Backend;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{self, tree, Tree};
use iced::advanced::{Clipboard, Shell, Widget};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorMoved, WheelScrolled};
use iced::mouse::ScrollDelta;
use iced::touch::Event::{FingerLifted, FingerLost, FingerMoved, FingerPressed};

use iced::widget::canvas::{Path, Stroke};
use iced::{event, keyboard, mouse, Color, Theme};
use iced::{Element, Event};
use iced::{Length, Rectangle, Size};
use serde::Serialize;

use crate::math::{Point, Vector};
use crate::OrderMap;

use super::shapes::{Shape, ShapeId, Shapes};

#[derive(Clone, Serialize)]
pub struct Camera {
    pub position: Vector,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: [0., 0.].into(),
            zoom: 1.0,
        }
    }
}

/// A workspace is a an infinite canvas that can be zoomed, panned,
/// and contains widgets that can be placed anywhere in 2d
pub struct Workspace<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    contents: Shapes<Element<'a, Message, Theme, Renderer>>,
    connections: Vec<(Path, Stroke<'a>)>,
    camera: Camera,
    pan: Option<Box<dyn Fn(Vector) -> Message + 'a>>,
    zoom: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_cursor_move: Option<Box<dyn Fn(Point) -> Message + 'a>>,
    on_click: Option<Box<dyn Fn(Option<ShapeId>) -> Message + 'a>>,
    on_shape_drag: Option<Box<dyn Fn(ShapeId, Point) -> Message + 'a>>,
    on_shape_release: Option<Box<dyn Fn(ShapeId) -> Message + 'a>>,
}
#[derive(Default, PartialEq, Clone, Debug)]
enum Action {
    #[default]
    Idle,
    /// ShapeId and offset of the cursor with respect to the shape position
    Drag(ShapeId, Vector),
}

#[derive(Serialize)]
pub struct State {
    pub camera: Camera,
    pub shape_positions: OrderMap<ShapeId, Point>,
}

impl Default for State {
    fn default() -> Self {
        Self::new([].into())
    }
}

impl State {
    pub fn new(shapes: OrderMap<ShapeId, Point>) -> State {
        Self {
            camera: Camera::default(),
            shape_positions: shapes,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct InnerState {
    modifiers: keyboard::Modifiers,
    action: Action,
}

impl<'a, Message, Theme, Renderer> Workspace<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    pub fn new(
        state: &'a State,
        node_view: impl Fn(ShapeId) -> Element<'a, Message, Theme, Renderer>,
        connections_view: impl Fn(ShapeId, &OrderMap<ShapeId, Point>) -> Vec<(Path, Stroke<'a>)>,
    ) -> Self {
        let positions = state
            .shape_positions
            .iter()
            .map(|(id, position)| (*id, *position))
            .collect();
        let contents = Shapes(
            state
                .shape_positions
                .iter()
                .map(|(id, position)| (*id as ShapeId, Shape::new(*position, node_view(*id))))
                .collect(),
        );
        let connections = state
            .shape_positions
            .iter()
            .flat_map(|(id, _p)| connections_view(*id, &positions))
            .collect();

        Self {
            contents,
            connections,
            camera: state.camera.clone(),
            pan: None,
            zoom: None,
            on_cursor_move: None,
            on_click: None,
            on_shape_drag: None,
            on_shape_release: None,
        }
    }

    pub fn pan(mut self, pan: impl Fn(Vector) -> Message + 'a) -> Self {
        self.pan = Some(Box::new(pan));
        self
    }

    pub fn zoom(mut self, zoom: impl Fn(f32) -> Message + 'a) -> Self {
        self.zoom = Some(Box::new(zoom));
        self
    }

    pub fn on_press(mut self, on_press: impl Fn(Option<ShapeId>) -> Message + 'a) -> Self {
        self.on_click = Some(Box::new(on_press));
        self
    }

    pub fn on_shape_drag(mut self, on_shape_drag: impl Fn(ShapeId, Point) -> Message + 'a) -> Self {
        self.on_shape_drag = Some(Box::new(on_shape_drag));
        self
    }

    pub fn on_release(mut self, on_release: impl Fn(ShapeId) -> Message + 'a) -> Self {
        self.on_shape_release = Some(Box::new(on_release));
        self
    }

    pub fn on_cursor_move(mut self, on_move: impl Fn(Point) -> Message + 'a) -> Self {
        self.on_cursor_move = Some(Box::new(on_move));
        self
    }
}

/// Implement Widet
impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Workspace<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: Catalog,
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<InnerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(InnerState::default())
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(
            &self
                .contents
                .0
                .values()
                .map(|shape| &shape.state)
                .collect::<Vec<_>>(),
        )
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.contents
            .0
            .values()
            .map(|shape| Tree::new(shape.state.as_widget()))
            .collect()
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::with_children(
            //// Fill the screen
            limits.resolve(Length::Fill, Length::Fill, Size::new(50., 50.)),
            ///// Layout child elements
            self.contents
                .0
                .values()
                .zip(&mut tree.children)
                .map(|(shape, tree_child)| {
                    shape
                        .state
                        .as_widget()
                        .layout(tree_child, renderer, limits)
                        .move_to(shape.position - self.camera.position)
                })
                .collect(),
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &iced::advanced::renderer::Style,
        workspace_layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = workspace_layout.bounds();
        let workspace_offset = Vector::new(bounds.position().x, bounds.position().y);

        //// Draw saved curves
        let mut frame = renderer.new_frame(bounds.size());

        frame.translate((self.camera.position * -1.0).into());

        self.connections
            .iter()
            .for_each(|(p, s)| frame.stroke(p, *s));

        renderer.with_layer(workspace_layout.bounds(), |renderer| {
            renderer.with_translation(workspace_offset.into(), |renderer| {
                renderer.draw_geometry(frame.into_geometry())
            });
        });
        //// Draw Elements
        {
            //TODO: apply zoom transform
            //// Render Children in a layer that is bounded to the size of the workspace
            let elements = self.contents.0.values().zip(&tree.children);
            for ((shape, tree), c_layout) in elements.zip(workspace_layout.children()) {
                renderer.with_layer(workspace_layout.bounds(), |renderer| {
                    shape
                        .state
                        .as_widget()
                        .draw(tree, renderer, theme, style, c_layout, cursor, &bounds);
                });
            }
        }
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
        let event_status = event::Status::Ignored;

        // update inner state
        let inner_state = tree::State::downcast_mut::<InnerState>(&mut tree.state);
        if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            inner_state.modifiers = modifiers
        }
        let bounds = layout.bounds();
        let workspace_offset = Vector::new(bounds.position().x, bounds.position().y);

        //// Pass event down to children
        let event_status = self
            .contents
            .0
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|(((_id, shape), tree), layout)| {
                shape.state.as_widget_mut().on_event(
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
            .fold(event_status, event::Status::merge);

        match (event_status, cursor.position()) {
            //// Only process events that are not captured by inner widgets
            (event::Status::Ignored, Some(cursor_position)) => match event.clone() {
                Event::Mouse(ButtonPressed(mouse::Button::Left))
                | Event::Touch(FingerPressed { .. }) => {
                    ////Find the first coliding shape
                    if let Some((id, offset)) =
                        self.contents.find_shape(cursor_position.into(), layout)
                    {
                        //// publish event
                        if let Some(on_shape_click) = &self.on_click {
                            shell.publish(on_shape_click(Some(id)));
                        }
                        //// update inner state
                        inner_state.action = Action::Drag(id, offset);
                        //// capture event
                        event::Status::Captured
                    } else {
                        //// deselect
                        if bounds.contains(cursor_position) {
                            if let Some(on_shape_click) = &self.on_click {
                                shell.publish(on_shape_click(None));
                            }
                            event::Status::Captured
                        } else {
                            event::Status::Ignored
                        }
                    }
                }
                Event::Mouse(ButtonReleased(mouse::Button::Left))
                | Event::Touch(FingerLifted { .. })
                | Event::Touch(FingerLost { .. }) => {
                    if let Action::Drag(id, _) = &inner_state.action {
                        //// publish event
                        if let Some(on_shape_release) = &self.on_shape_release {
                            shell.publish(on_shape_release(*id));
                        }
                        //// update inner state
                        inner_state.action = Action::Idle;
                        //// capture event
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                Event::Mouse(CursorMoved { .. }) | Event::Touch(FingerMoved { .. }) => {
                    //TODO: find shape
                    if let Action::Drag(id, offset) = &inner_state.action {
                        //// publish event
                        if let Some(on_drag) = &self.on_shape_drag {
                            shell.publish(on_drag(*id, Point::from(cursor_position) - *offset));
                        }
                        //// capture event
                        event::Status::Captured
                    } else {
                        if let Some(on_move) = &self.on_cursor_move {
                            shell.publish(on_move(Point::from(cursor_position) - workspace_offset));
                        }
                        event::Status::Ignored
                    }
                }
                Event::Mouse(WheelScrolled { delta }) => {
                    if let Some(pan) = &self.pan {
                        if bounds.contains(cursor_position) {
                            let offset = match delta {
                                ScrollDelta::Lines { x, y } => {
                                    if inner_state.modifiers.shift() {
                                        //scale scrolled lines to be equivalent to 16 pixels
                                        Vector::new(y, x) * 16.
                                    } else {
                                        Vector::new(x, y) * 16.
                                    }
                                }
                                ScrollDelta::Pixels { x, y } => Vector::new(x, y),
                            };
                            //// publish event
                            shell.publish(pan(offset));
                            //// capture event
                            event::Status::Captured
                        } else {
                            event::Status::Ignored
                        }
                    } else {
                        event::Status::Ignored
                    }
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.contents
            .0
            .values()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((shape, state), layout)| {
                shape
                    .state
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
            .unwrap_or_default()
    }
}

/// Convert to an element
impl<'a, Message, Theme, Renderer> From<Workspace<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: 'a + iced::advanced::graphics::geometry::Renderer,
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
    state: &'a State,
    node_view: impl Fn(ShapeId) -> Element<'a, Message, Theme, Renderer>,
    connections_view: impl Fn(ShapeId, &OrderMap<ShapeId, Point>) -> Vec<(Path, Stroke<'a>)>,
) -> Workspace<'a, Message, Theme, Renderer>
where
    Theme: 'a + Catalog,
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    Workspace::new(state, node_view, connections_view)
}

/// Very rough styling implementation
/// The appearance of a workspace.
#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub background: Color,

    pub foreground: Color,
}

pub trait Catalog: Sized {
    type Class<'a>;

    fn default<'a>() -> Self::Class<'a>;

    fn style(&self, class: &Self::Class<'_>) -> Style;
}

pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        class(self)
    }
}

pub fn default(theme: &Theme) -> Style {
    let palette = theme.palette();

    let background = palette.background;
    let foreground = palette.primary;

    Style {
        background,
        foreground,
    }
}
