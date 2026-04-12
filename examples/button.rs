// Topic: Iced

use iced::widget::{button, column, container, right, stack, text, text_input};
use iced::{Color, Element, Font, Length, Pixels, Settings, Task, Theme, font};

#[derive(Debug, Clone)]
enum Message {
    TextInput(String),
    Record,
}

#[derive(Default)]
struct App {
    title: String,
    num: i32,
}

impl App {
    fn show(&self) -> bool {
        self.title.contains("you")
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::TextInput(res) => {
                self.title = res;
                if self.show() {
                    Task::done(Message::Record)
                } else {
                    Task::none()
                }
            }
            Message::Record => {
                self.num += 1;
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let title = text_input(&self.title, &self.title)
            .style(|theme: &Theme, status| {
                let default = text_input::default(theme, status);
                text_input::Style {
                    value: if self.show() {
                        Color::from_rgb(0.0, 1.0, 0.0)
                    } else {
                        Color::from_rgb(1.0, 0.0, 0.0)
                    },
                    ..default
                }
            })
            .on_input(Message::TextInput);

        let desc = text!("you change title {} times", self.num)
            .font(Font {
                style: font::Style::Italic,
                ..Default::default()
            })
            .style(|_| text::Style {
                color: Some(Color::from_rgb(1.0, 0.0, 0.0)),
            })
            .width(Length::FillPortion(2))
            .height(30);

        let btn = button("click me")
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();
                match status {
                    button::Status::Pressed => {
                        button::Style::default().with_background(palette.success.strong.color)
                    }
                    button::Status::Hovered => button::primary(theme, status),
                    _ => button::secondary(theme, status),
                }
            })
            .on_press(Message::Record);

        let btn_row = stack!(desc, right(btn));

        container(column![title, btn_row].spacing(10).padding(12))
            .style(container::rounded_box)
            .into()
    }
    fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(App::theme)
        .settings(Settings {
            default_text_size: Pixels(12.0),
            ..Default::default()
        })
        .title("CA Analytics")
        .run()
}
