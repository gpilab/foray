use std::iter;

use iced::advanced::graphics::geometry;
use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::widget;
use iced::advanced::Clipboard;
use iced::advanced::Layout;
use iced::advanced::Shell;
use iced::advanced::Widget;
use iced::overlay;
use iced::widget::canvas::Path;
use iced::widget::canvas::Stroke;
use iced::Color;
use iced::{self, Element, Event, Length, Pixels, Point, Rectangle, Size, Vector};

type Inputs = Vec<iced::Color>;
type Outputs = Vec<iced::Color>;

const DEFAULT_NODE_WIDTH: f32 = 100.;
const DEFAULT_NODE_HEIGHT: f32 = 60.;
pub struct Node<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: geometry::Renderer,
{
    inputs: Inputs,
    outputs: Outputs,
    content: Element<'a, Message, Theme, Renderer>,
    width: f32,
    height: f32,
    //position: Point,
}

impl<'a, Message, Theme, Renderer> Node<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
{
    /// Creates a [`Pin`] widget with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        inputs: Inputs,
        outputs: Outputs,
    ) -> Self {
        Self {
            inputs,
            outputs,
            content: content.into(),
            width: DEFAULT_NODE_WIDTH,
            height: DEFAULT_NODE_HEIGHT,
            //position: Point::ORIGIN,
        }
    }

    /// Sets the width of the [`Pin`].
    pub fn width(mut self, width: impl Into<f32>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Pin`].
    pub fn height(mut self, height: impl Into<f32>) -> Self {
        self.height = height.into();
        self
    }

    ///// Sets the position of the [`Pin`]; where the pinned widget will be displayed.
    //pub fn position(mut self, position: impl Into<Point>) -> Self {
    //    self.position = position.into();
    //    self
    //}
    //
    ///// Sets the X coordinate of the [`Pin`].
    //pub fn x(mut self, x: impl Into<Pixels>) -> Self {
    //    self.position.x = x.into().0;
    //    self
    //}
    //
    ///// Sets the Y coordinate of the [`Pin`].
    //pub fn y(mut self, y: impl Into<Pixels>) -> Self {
    //    self.position.y = y.into().0;
    //    self
    //}
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Node<'_, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        self.content.as_widget().tag()
    }

    fn state(&self) -> widget::tree::State {
        self.content.as_widget().state()
    }

    fn children(&self) -> Vec<widget::Tree> {
        self.content.as_widget().children()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        self.content.as_widget().diff(tree);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fixed(self.width),
            height: Length::Fixed(self.height),
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        //dbg!(self.position);
        let limits = limits.width(self.width).height(self.height);

        let available = limits.max(); // - Size::new(self.position.x, self.position.y);

        let node = self.content.as_widget().layout(
            tree,
            renderer,
            &layout::Limits::new(Size::ZERO, available * 2.),
        );
        //.move_to(self.position);

        let size = limits.resolve(self.width, self.height, node.size());
        layout::Node::with_children(size * 2., vec![node])
    }

    fn operate(
        &self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.content.as_widget().operate(
            tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced::event::Status {
        self.content.as_widget_mut().on_event(
            tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        const NODE_BORDER_RADIUS: f32 = 2.0;
        const NODE_STROKE_WIDTH: f32 = 1.0;

        const PORT_RADIUS: f32 = 10.;
        const PORT_STROKE_WIDTH: f32 = 2.;
        const PORT_SPACING: f32 = 10.;

        let bounds = layout.bounds();
        dbg!(layout.position());

        let mut frame = iced::widget::canvas::Frame::new(renderer, bounds.size());
        frame.translate(Vector::new(layout.position().x, layout.position().y));
        //// Input/output
        let mut draw_ports = |inputs: &Inputs, y: f32| {
            inputs.iter().enumerate().for_each(|(i, color)| {
                let i = i as f32;
                let center_x = (i + 1.) * PORT_SPACING + PORT_RADIUS + i * (2. * PORT_RADIUS);
                frame.stroke(
                    &Path::circle(Point::new(center_x, y), PORT_RADIUS),
                    Stroke::default()
                        .with_color(*color)
                        .with_width(PORT_STROKE_WIDTH),
                );
            });
        };

        draw_ports(&self.inputs, PORT_RADIUS);
        draw_ports(&self.outputs, self.height - PORT_RADIUS);

        //// Node background
        frame.fill(
            &Path::rounded_rectangle(
                (0., 0.).into(),
                //self.position + Vector::new(0.0, PORT_RADIUS),
                bounds.size() - Vector::new(0.0, 2. * PORT_RADIUS).into(),
                NODE_BORDER_RADIUS.into(),
            ),
            Color::BLACK,
        );

        renderer.draw_geometry(frame.into_geometry());

        if let Some(clipped_viewport) = bounds.intersection(viewport) {
            self.content.as_widget().draw(
                tree,
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                &clipped_viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<Node<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + geometry::Renderer,
{
    fn from(pin: Node<'a, Message, Theme, Renderer>) -> Element<'a, Message, Theme, Renderer> {
        Element::new(pin)
    }
}
