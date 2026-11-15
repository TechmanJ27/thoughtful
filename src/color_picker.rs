use iced::{
    Alignment, Color, Element, Length,
    widget::{Button, Container, Row, Text},
};

use iced_aw::helpers::color_picker;

#[derive(Clone, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Message {
    ChooseColor,
    SubmitColor(Color),
    CancelColor
}

#[derive(Debug)]
pub struct ColorPickerExample {
    color: Color,
    show_picker: bool,
}

impl Default for ColorPickerExample {
    fn default() -> Self {
        Self {
            color: Color::from_rgba8(0, 0, 0, 1.0),
            show_picker: false,
        }
    }
}
impl ColorPickerExample {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::ChooseColor => {
                self.show_picker = true;
            }
            Message::SubmitColor(color) => {
                self.color = color;
                self.show_picker = false;
            }
            Message::CancelColor => {
                self.show_picker = false;
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let but = Button::new(Text::new("Set Color")).on_press(Message::ChooseColor);

        let color_picker = color_picker(
            self.show_picker,
            self.color,
            but,
            Message::CancelColor,
            Message::SubmitColor,
        );

        let row = Row::new()
            .align_y(Alignment::Center)
            .spacing(10)
            .push(color_picker)
            .push(Text::new(format!("Color: {:?}", self.color)));

        Container::new(row)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}