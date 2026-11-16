mod color_picker;
mod thought_bubble;
mod sql;

use iced_aw::ICED_AW_FONT_BYTES;
use iced::window::latest as latest;

#[derive(Default)]
struct App {
    bubble: thought_bubble::Bubble,
}

#[derive(Debug, Clone)]
enum Message {
    Bubble(thought_bubble::Message),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Bubble(msg) => self.bubble.update(msg),
        }
    }

    fn view(&self, x, y) -> iced::Element<'_, Message> {
        let on_screen_nodes = sql::load_nodes(latest(), x, y);
        for node in on_screen_nodes {
            
        }
        iced::widget::column![
            self.bubble.view(x, y).map(Message::Bubble),
        ]
        .into()
    }
}

pub fn main() -> iced::Result {
    sql::create_tables();
    sql::test_nodes();
    let mut x = 0;
    let mut y = 0;
    iced::application(App::default, App::update, App::view(x, y))
        .font(ICED_AW_FONT_BYTES)
        .run()
}
