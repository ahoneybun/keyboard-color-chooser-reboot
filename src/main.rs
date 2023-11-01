use iced::widget::{button, column, text};
use iced::window;
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Test::run(Settings {
        antialiasing: true,
        window: window::Settings {
            size: (500, 450),
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

struct Test {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    DecrementPressed,
}

impl Sandbox for Test {
    type Message = Message;

    fn new() -> Self {
        Self { value: 0 }
    }

    fn title(&self) -> String {
        String::from("Keyboard Color Switcher")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            }
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("Increment").on_press(Message::IncrementPressed),
            text(self.value).size(50),
            button("Decrement").on_press(Message::DecrementPressed)
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}

