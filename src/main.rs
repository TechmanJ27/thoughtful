mod color_picker;
mod thought_bubble;

use iced_aw::ICED_AW_FONT_BYTES;

#[derive(Default)]
struct App {
    color_picker: color_picker::ColorPickerExample,
    bubble: thought_bubble::Bubble,
}

#[derive(Debug, Clone)]
enum Message {
    ColorPicker(color_picker::Message),
    Bubble(thought_bubble::Message),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::ColorPicker(msg) => self.color_picker.update(msg),
            Message::Bubble(msg) => self.bubble.update(msg),
        }
    }

    fn view(&self) -> iced::Element<'_, Message> {
        iced::widget::column![
            self.color_picker.view().map(Message::ColorPicker),
            self.bubble.view().map(Message::Bubble),
        ]
        .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .font(ICED_AW_FONT_BYTES)
        .run()
}