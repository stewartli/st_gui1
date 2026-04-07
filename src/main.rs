// Topic: Iced

use iced::widget::{button, center, column, container, text};
use iced::{Alignment, Element, Task, Theme};

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
        Theme::GruvboxLight
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(App::theme)
        .run()
}
