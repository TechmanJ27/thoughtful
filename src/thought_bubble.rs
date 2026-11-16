use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Widget};
use iced::border;
use iced::mouse;
use iced::{Fill, Color, Center, Element, Length, Rectangle, Size};
use iced::widget::{pin, column, text};

pub struct Circle {
    radius: f32,
}

impl Circle {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn circle(radius: f32) -> Circle {
    Circle::new(radius)
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Circle
where
    Renderer: renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut widget::Tree,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.radius * 2.0, self.radius * 2.0))
    }

    fn draw(
        &self,
        _tree: &widget::Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::rounded(self.radius),
                ..renderer::Quad::default()
            },
            Color::BLACK,
        );
    }
}

impl<Message, Theme, Renderer> From<Circle>
    for Element<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn from(circle: Circle) -> Self {
        Self::new(circle)
    }
}

pub struct Bubble {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RadiusChanged(f32),
}

impl Bubble {
    fn new() -> Self {
        Bubble { radius: 50.0 }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    pub fn view(&self, x: i64, y: i64) -> Element<'_, Message> {
        let content = column![
            circle(self.radius),
        ]
        .padding(20)
        .spacing(20)
        .max_width(500)
        .align_x(Center);

        pin(content).x(x).y(y)
    }
}

impl Default for Bubble {
    fn default() -> Self {
        Self::new()
    }
}
