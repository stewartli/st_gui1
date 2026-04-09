// Topic: Iced
// 1. cargo run --example <example_name> -- --arg1 --arg2
// 2. cargo run -p <package_name> --example
// 3. Element, Task, Subscription, Program, Widget,

use iced::keyboard;
use iced::widget::{button, center, column, container, text};
use iced::window::Settings;
use iced::{Alignment, Element, Size, Subscription, Task, Theme};

#[derive(Debug, Clone)]
enum Message {
    Inc,
    Dec,
    Step(i32),
}

#[derive(Debug, Default)]
struct App {
    num: i32,
}

impl App {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Inc => {
                self.num += 1;
                Task::done(Message::Step(10))
            }
            Message::Dec => {
                self.num -= 1;
                Task::none()
            }
            Message::Step(res) => {
                self.num += res;
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        container(
            center(
                column![
                    text(self.num).size(20),
                    button("Inc").on_press(Message::Inc),
                    button("Dec").on_press(Message::Dec),
                ]
                .spacing(12),
            )
            .padding(12),
        )
        .align_y(Alignment::End)
        .into()
    }
    fn theme(&self) -> Theme {
        if self.num > 30 {
            Theme::GruvboxLight
        } else {
            Theme::Nord
        }
    }
    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|evt, _status, _id| match evt {
            iced::Event::Keyboard(iced::keyboard::Event::KeyPressed { key, .. }) => match key {
                keyboard::Key::Character(c) if c.as_str() == "j" => Some(Message::Dec),
                keyboard::Key::Character(c) if c.as_str() == "k" => Some(Message::Inc),
                _ => None,
            },
            _ => None,
        })
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("CA Analytics")
        .theme(App::theme)
        .subscription(App::subscription)
        .settings(iced::Settings {
            id: Some("CA Analytics".to_string()),
            ..Default::default()
        })
        .window(Settings {
            size: Size {
                width: 400.0,
                height: 300.0,
            },
            ..Default::default()
        })
        .run()
}
