use std::iter;

use iced::advanced::layout;
use iced::advanced::mouse;
use iced::advanced::overlay;
use iced::advanced::renderer;
use iced::advanced::widget;
use iced::advanced::widget::Widget;
use iced::advanced::Shell;
use iced::advanced::{Clipboard, Layout};
use iced::{Element, Event, Length, Pixels, Point, Rectangle, Size, Vector};

/// A container that can have additional pinned elements positioned relative to the container
/// and doesn't cut them off because they are out of the container's bounds
#[allow(missing_debug_implementations)]
pub struct NodeContainer<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    main_content: Element<'a, Message, Theme, Renderer>,
    absolute_children: Vec<Element<'a, Message, Theme, Renderer>>,
    width: Length,
    height: Length,
    position: Point,
}

impl<'a, Message, Theme, Renderer> NodeContainer<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    /// Creates a Node container widget with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        absolute_children: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            main_content: content.into(),
            absolute_children,
            width: Length::Shrink,
            height: Length::Shrink,
            position: Point::ORIGIN,
        }
    }

    /// Sets the width of the NodeContainer
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the NodeContainer
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the position of the [`Pin`]; where the pinned widget will be displayed.
    pub fn position(mut self, position: impl Into<Point>) -> Self {
        self.position = position.into();
        self
    }

    /// Sets the X coordinate of the [`Pin`].
    pub fn x(mut self, x: impl Into<Pixels>) -> Self {
        self.position.x = x.into().0;
        self
    }

    /// Sets the Y coordinate of the [`Pin`].
    pub fn y(mut self, y: impl Into<Pixels>) -> Self {
        self.position.y = y.into().0;
        self
    }
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for NodeContainer<'_, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> widget::tree::Tag {
        self.main_content.as_widget().tag()
    }

    fn state(&self) -> widget::tree::State {
        self.main_content.as_widget().state()
    }

    fn children(&self) -> Vec<widget::Tree> {
        // the first child is always main_content, followed by the children
        iter::once(&self.main_content)
            .chain(&self.absolute_children)
            .map(widget::Tree::new)
            .collect()
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(&self.absolute_children);
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);

        let node = self
            .main_content
            .as_widget()
            .layout(
                &mut tree.children[0],
                renderer,
                &layout::Limits::new(Size::ZERO, limits.max()),
            )
            .move_to(self.position);

        let size = node.size();

        let mut binding = tree.children.iter_mut().skip(1);
        let children_node = self
            .absolute_children
            .iter()
            .zip(&mut binding)
            .map(|(e, tree)| {
                e.as_widget().layout(
                    tree,
                    renderer,
                    &layout::Limits::new(Size::ZERO, size * 2.), //give enough size for offset elements
                )
            });

        let size = limits.resolve(self.width, self.height, node.size());

        layout::Node::with_children(size, iter::once(node).chain(children_node).collect())
    }

    fn operate(
        &self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.main_content.as_widget().operate(
            tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    //TODO: make events work for children
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
        self.main_content.as_widget_mut().on_event(
            tree, event, layout, //.children().next().unwrap(),
            cursor, renderer, clipboard, shell, viewport,
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
        self.main_content.as_widget().mouse_interaction(
            tree, layout, //.children().next().unwrap(),
            cursor, viewport, renderer,
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
        //let bounds = layout.bounds();

        //if let Some(clipped_viewport) = bounds.intersection(viewport) {
        //    self.main_content.as_widget().draw(
        //        tree,
        //        renderer,
        //        theme,
        //        style,
        //        layout.children().next().unwrap(),
        //        cursor,
        //        &clipped_viewport,
        //    );
        //        self.absolute_children
        //            .iter()
        //            .for_each(|e| e.as_widget().draw(tree)),
        //}

        //if let Some(clipped_viewport) = layout.bounds().intersection(viewport) {
        renderer.with_layer(*viewport, |renderer| {
            for ((child, state), layout) in self
                .absolute_children
                .iter()
                .zip(tree.children.iter().skip(1))
                .zip(layout.children().skip(1))
            {
                child.as_widget().draw(
                    state,
                    renderer,
                    theme,
                    style,
                    layout,
                    cursor,
                    &layout.bounds(),
                );
            }
        });
        renderer.with_layer(*viewport, |renderer| {
            self.main_content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout.children().next().unwrap(),
                cursor,
                viewport,
            )
        });
        //}
    }

    //TODO: support this?
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.main_content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<NodeContainer<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(
        pin: NodeContainer<'a, Message, Theme, Renderer>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(pin)
    }
}
