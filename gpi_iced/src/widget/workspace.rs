use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{tree, Tree};
use iced::advanced::{Clipboard, Shell, Widget};
use iced::mouse::Event::{ButtonPressed, ButtonReleased, CursorMoved, WheelScrolled};
use iced::mouse::ScrollDelta;
use iced::touch::Event::{FingerLifted, FingerLost, FingerMoved, FingerPressed};

use iced::{event, keyboard, mouse, Color, Point, Theme, Vector};
use iced::{Element, Event};
use iced::{Length, Rectangle, Size};

use super::shapes::{Shape, ShapeId, Shapes};

#[derive(Clone)]
pub struct Camera {
    pub position: Vector,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            position: Vector::ZERO,
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
    camera: Camera,
    pan: Option<Box<dyn Fn(Vector) -> Message + 'a>>,
    zoom: Option<Box<dyn Fn(f32) -> Message + 'a>>,
    on_shape_click: Option<Box<dyn Fn(ShapeId) -> Message + 'a>>,
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

pub struct State<T> {
    pub camera: Camera,
    pub shapes: Shapes<T>,
}

impl<T: Default> Default for State<T> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<T> State<T> {
    pub fn new(shapes: Vec<(Point, T)>) -> State<T> {
        Self {
            camera: Camera::default(),
            shapes: Shapes(
                shapes
                    .into_iter()
                    .enumerate()
                    .map(|(i, (point, shape))| (i as ShapeId, Shape::new(point, shape)))
                    .collect(),
            ),
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
    pub fn new<T>(
        state: &'a State<T>,
        view: impl Fn(ShapeId, &'a T) -> Element<'a, Message, Theme, Renderer>,
    ) -> Self {
        let contents = Shapes(
            state
                .shapes
                .0
                .iter()
                .map(|(id, shape)| {
                    (
                        *id as ShapeId,
                        Shape::new(shape.position, view(*id, &shape.state)),
                    )
                })
                .collect(),
        );

        Self {
            contents,
            camera: state.camera.clone(),
            pan: None,
            zoom: None,
            on_shape_click: None,
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

    pub fn on_press(mut self, on_press: impl Fn(ShapeId) -> Message + 'a) -> Self {
        self.on_shape_click = Some(Box::new(on_press));
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

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.contents
            .0
            .iter()
            .map(|(_id, element)| Tree::new(element.state.as_widget()))
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
        viewport: &Rectangle,
    ) {
        //// Saved curves
        //let geo = self
        //    .cache
        //    .draw(renderer, workspace_layout.bounds().size() * 2., |frame| {
        //        //println!("drawing!");
        //        frame.translate(-self.camera.position);
        //
        //        //// Foreground
        //        //self.primitives.iter().for_each(|v| v.draw(frame));
        //    });

        //renderer.draw_geometry(geo);

        let padding = 0.0;

        //TODO: apply zoom transform
        //// Render Children in a layer that is bounded to the size of the workspace
        let elements = self.contents.0.values().zip(&tree.children);
        for ((shape, tree), c_layout) in elements.zip(workspace_layout.children()) {
            renderer.with_layer(workspace_layout.bounds().shrink(padding), |renderer| {
                shape
                    .state
                    .as_widget()
                    .draw(tree, renderer, theme, style, c_layout, cursor, viewport);
            });
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

        ////Pass event down to children
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
                    debug_assert!(inner_state.action == Action::Idle);

                    ////Find the first coliding shape
                    if let Some((id, offset)) = self.contents.find_shape(cursor_position, layout) {
                        //// publish event
                        if let Some(on_shape_click) = &self.on_shape_click {
                            shell.publish(on_shape_click(id));
                        }
                        //// update inner state
                        inner_state.action = Action::Drag(id, offset);
                        //// capture event
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
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
                            shell.publish(on_drag(*id, cursor_position - *offset));
                        }
                        //// capture event
                        event::Status::Captured
                    } else {
                        event::Status::Ignored
                    }
                }
                Event::Mouse(WheelScrolled { delta }) => {
                    if let Some(pan) = &self.pan {
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
                }
                _ => event::Status::Ignored,
            },
            _ => event::Status::Ignored,
        }
    }

    //fn mouse_interaction(
    //    &self,
    //    tree: &Tree,
    //    layout: Layout<'_>,
    //    cursor: mouse::Cursor,
    //    _viewport: &Rectangle,
    //    _renderer: &Renderer,
    //) -> mouse::Interaction {
    //    //let action = tree.state.downcast_ref::<InnerState>().action;
    //
    //    match action {
    //        Action::Dragging { .. } => mouse::Interaction::Grabbing,
    //        Action::Idle => {
    //            if layout.children().any(|l| {
    //                cursor
    //                    .position_over(
    //                        l.bounds()
    //                            .intersection(&layout.bounds())
    //                            .unwrap_or(Rectangle::new((0., 0.).into(), (0., 0.).into())),
    //                    )
    //                    .is_some()
    //            }) {
    //                //TODO: get mouse status of children?
    //                mouse::Interaction::Grab
    //            } else {
    //                mouse::Interaction::default()
    //            }
    //        }
    //    }
    //}
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
pub fn workspace<'a, T, Message, Theme, Renderer>(
    state: &'a State<T>,
    view: impl Fn(ShapeId, &'a T) -> Element<'a, Message, Theme, Renderer>,
) -> Workspace<'a, Message, Theme, Renderer>
where
    Theme: 'a + Catalog,
    Renderer: iced::advanced::graphics::geometry::Renderer,
{
    Workspace::new(state, view)
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
