use iced::{
    advanced::{
        layout::{self},
        mouse, renderer,
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, touch,
    widget::{container, Space},
    Alignment, Element, Event, Length, Point, Rectangle, Size, Vector,
};

/// Widget State,
/// not stored in the tree.
/// Only InnerState get stored in the tree, I think...
pub struct Draggable<'a, Message, Theme, Renderer> {
    //action: Action,
    on_pickup: Option<Message>,
    on_release: Option<Message>,
    inner_widget: Element<'a, Message, Theme, Renderer>,
    inner_state: InnerState,
}

impl<'a, Message, Theme, Renderer> Draggable<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    pub fn new() -> Self {
        dbg!("new draggable");
        Self {
            on_pickup: None,
            on_release: None,
            inner_widget: Space::new(100., 100.).into(),
            inner_state: InnerState::default(),
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

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Draggable<'_, Message, Theme, Renderer>
where
    Message: Clone,
    Theme: container::Catalog,
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<InnerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(self.inner_state)
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(100.),
            height: Length::Fixed(100.),
        }
    }
    fn diff(&self, tree: &mut Tree) {
        tree.children = vec![Tree::new(&self.inner_widget)];
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.inner_widget.as_widget())]
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits, //TODO: put in limits so we don't draw outside the container?
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<InnerState>();

        let mut child_node =
            self.inner_widget
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits);

        let size_of_this_node = child_node.size().expand(Size::new(50., 50.));

        child_node = child_node.align(Alignment::Center, Alignment::Center, size_of_this_node);

        layout::Node::with_children(
            Size::new(100., state.position.x.max(50.0)),
            vec![child_node.move_to(state.position)],
        )
        .move_to(state.position)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: iced::Border {
                    color: style.text_color,
                    width: 5.,
                    radius: 5.into(),
                },
                ..renderer::Quad::default()
            },
            iced::Color::from_rgb8(10, 10, 15),
        );

        self.inner_widget.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<InnerState>();
        let bounds = state.bounds(layout);

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                dbg!(&event);
                dbg!(&state);
                if let Some(cursor_position) = cursor.position_over(bounds) {
                    //// fire event
                    if let Some(on_pickup) = self.on_pickup.clone() {
                        shell.publish(on_pickup);
                    }

                    //// update state
                    state.action = Action::Dragging {
                        start_pos: state.position,
                        shape_offset: cursor_position - state.position,
                    };

                    //// end event propogation?
                    dbg!("captured pickup");
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                dbg!(&event);
                dbg!(&state);
                if let Action::Dragging { .. } = state.action {
                    //// fire event
                    if let Some(on_release) = self.on_release.clone() {
                        shell.publish(on_release);
                    }

                    //// update state
                    state.action = Action::Idle;

                    dbg!("captured release");
                    //// end event propogation?
                    return event::Status::Captured;
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                if let Action::Dragging {
                    start_pos: _,
                    shape_offset,
                } = state.action
                {
                    //// update state
                    if let Some(cursor_position) = cursor.position() {
                        // force a relayout, so that inner widgets will move relative to the new position
                        shell.invalidate_layout(); // unsure if this should be invalidate_widgets,
                                                   // or invalidate_layout

                        state.position.x = cursor_position.x - (shape_offset.x);
                        state.position.y = cursor_position.y - (shape_offset.y);

                        return event::Status::Captured;
                    }
                }
            }
            _ => {}
        }

        event::Status::Ignored
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<InnerState>();

        let bounds = state.bounds(layout);
        let is_mouse_over = cursor.is_over(bounds);

        if let Action::Dragging { .. } = state.action {
            mouse::Interaction::Grabbing
        } else if is_mouse_over {
            mouse::Interaction::Grab
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Draggable<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(
        slider: Draggable<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(slider)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct InnerState {
    action: Action,
    position: Point,
}
impl InnerState {
    fn bounds(&self, layout: Layout) -> Rectangle {
        let pos: Point<f32> =
            layout.bounds().position() + Vector::new(self.position.x, self.position.y);
        Rectangle {
            x: pos.x,
            y: pos.y,
            width: layout.bounds().width,
            height: layout.bounds().height,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum Action {
    Dragging {
        /// where the drag started
        start_pos: Point,

        /// the intial offset between the shape's location and the cursor's location
        /// used to maintain a consititent relative position between the cursor
        /// and the shape throughout the drag event
        shape_offset: Vector,
    },
    #[default]
    Idle,
}

// Convenience Function
pub fn draggable<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Draggable<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer,
{
    Draggable {
        on_pickup: None,
        on_release: None,
        inner_widget: content.into(),
        inner_state: InnerState::default(),
    }
}

impl<'a, Message, Theme, Renderer> Default for Draggable<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: container::Catalog + iced::widget::button::Catalog + iced::widget::text::Catalog + 'a,
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'a,
{
    fn default() -> Self {
        Self::new()
    }
}
