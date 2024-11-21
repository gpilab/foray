//! This example showcases a simple native custom widget that draws a circle.
use content::Content;
use iced::advanced::graphics::core::widget;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::Tree;
use iced::advanced::{renderer, Widget};
use iced::mouse;
use iced::widget::container::{self};
use iced::Element;
use iced::Length::Fill;
use iced::{Color, Length, Rectangle, Size};

pub mod content;

/// A workspace is a an infinite canvas that can be zoomed, panned,
/// and contains widgets that can be placed anywhere in 3d (stacking in Z)
pub struct Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    width: Length,
    height: Length,
    elements: Vec<Content<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    pub fn new(elements: Vec<Content<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            elements,
            width: Fill,
            height: Fill,
        }
    }
}

// Convenience function
/// Create a new `Workspace`
pub fn workspace<'a, Message, Theme, Renderer>(
    elements: Vec<Content<'a, Message, Theme, Renderer>>,
) -> Workspace<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    Workspace::new(elements)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Workspace<'_, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.elements
            .iter()
            .map(|content| content.state())
            .collect()
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.resolve(self.width, self.height, Size::ZERO);
        //layout::Node::new(size)
        println!("workspace layout");
        dbg!(size);
        dbg!(&tree);

        let children = self
            .elements
            .iter()
            .map(|e| e.layout(&mut tree.children[0], renderer, limits)) //TODO: do I just
            //pass in tree? or tree.children??
            .collect();

        dbg!(&children);
        layout::Node::with_children(size, children)
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
        renderer.fill_quad(
            renderer::Quad {
                bounds: workspace_layout.bounds(),
                border: iced::Border::default().width(5.).color(iced::Color::WHITE),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: 50.,
                    y: 50.,
                    width: 50.,
                    height: 200.,
                },
                border: iced::Border::default().width(5.).color(iced::Color::WHITE),
                ..renderer::Quad::default()
            },
            Color::WHITE,
        );

        let elements = self.elements.iter().zip(&tree.children);
        println!("workspace draw");
        dbg!(&elements.len());
        dbg!(&workspace_layout);

        for ((element, tree), c_layout) in elements.zip(workspace_layout.children()) {
            dbg!(&element.body.as_widget().size());
            dbg!(&c_layout);
            dbg!(&tree);
            //let custom_layout = layout(
            //    limits,
            //    width,
            //    height,
            //    max_width,
            //    max_height,
            //    padding,
            //    horizontal_alignment,
            //    vertical_alignment,
            //    layout_content,
            //);

            element.draw(
                &tree, renderer, theme, style, c_layout, //TODO: should this
                //change?
                cursor, viewport,
            )
        }
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
