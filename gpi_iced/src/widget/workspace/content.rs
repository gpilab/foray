use iced::advanced::layout::Node;
use iced::advanced::widget::Tree;
use iced::advanced::{layout, mouse, renderer, Layout};
use iced::widget::container::{self};
use iced::{Element, Rectangle};

#[allow(missing_debug_implementations)]
pub struct Content<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    //title_bar: Option<TitleBar<'a, Message, Theme, Renderer>>,
    pub body: Element<'a, Message, Theme, Renderer>,
    position: (f32, f32), //class: Theme::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Content<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
    Renderer: renderer::Renderer,
{
    pub fn new(body: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Self {
            body: body.into(),
            position: (100., 300.),
        }
    }

    pub(super) fn state(&self) -> Tree {
        //let children =  self.body{
        //    vec![Tree::new(&self.body), title_bar.state()]
        //} else {
        //    vec![Tree::new(&self.body), Tree::empty()]
        //};
        //
        println!("Content state fn");
        //Tree::new(&self.body)
        Tree {
            children: vec![Tree::new(&self.body)],
            ..Tree::empty()
        }
    }
    //
    //pub(super) fn diff(&self, tree: &mut Tree) {
    //    if tree.children.len() == 2 {
    //        if let Some(title_bar) = self.title_bar.as_ref() {
    //            title_bar.diff(&mut tree.children[1]);
    //        }
    //
    //        tree.children[0].diff(&self.body);
    //    } else {
    //        *tree = self.state();
    //    }
    //}

    /// Draws the [`Content`] with the provided [`Renderer`] and [`Layout`].
    ///
    /// [`Renderer`]: core::Renderer
    pub fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        println!("Drawing content");
        dbg!(layout);

        let bounds = layout.bounds();

        {
            container::draw_background(renderer, &container::Style::default(), bounds);
        }
        //let mut children = layout.children();
        let node = Node::with_children(bounds.size(), vec![Node::new(bounds.size())])
            .move_to(self.position);
        let body_layout = Layout::new(&node); //with_offset(self.position);

        dbg!(self.body.as_widget().size());
        self.body.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            body_layout,
            cursor,
            viewport,
        );
        println!("drew content");
        //let bounds = layout.bounds();
        //
        //{
        //    let style = theme.style(&self.class);
        //
        //    container::draw_background(renderer, &style, bounds);
        //}
        //
        //if let Some(title_bar) = &self.title_bar {
        //    let mut children = layout.children();
        //    let title_bar_layout = children.next().unwrap();
        //    let body_layout = children.next().unwrap();
        //
        //    let show_controls = cursor.is_over(bounds);
        //
        //    self.body.as_widget().draw(
        //        &tree.children[0],
        //        renderer,
        //        theme,
        //        style,
        //        body_layout,
        //        cursor,
        //        viewport,
        //    );
        //
        //    title_bar.draw(
        //        &tree.children[1],
        //        renderer,
        //        theme,
        //        style,
        //        title_bar_layout,
        //        cursor,
        //        viewport,
        //        show_controls,
        //    );
        //} else {
        //    self.body.as_widget().draw(
        //        &tree.children[0],
        //        renderer,
        //        theme,
        //        style,
        //        layout,
        //        cursor,
        //        viewport,
        //    );
        //}
        //
    }
    pub(crate) fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        //self.body.as_widget().layout(tree, renderer, limits)

        self.body
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    pub fn position(&mut self, x: f32, y: f32) {
        self.position = (x, y);
    }
}

impl<'a, T, Message, Theme, Renderer> From<T> for Content<'a, Message, Theme, Renderer>
where
    T: Into<Element<'a, Message, Theme, Renderer>>,
    Theme: container::Catalog + 'a,
    Renderer: renderer::Renderer,
{
    fn from(element: T) -> Self {
        Self::new(element)
    }
}
