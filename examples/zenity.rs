// Topic: Iced

use iced::widget::{button, column, container, right, stack, text, text_input};
use iced::{Border, Color, Element, Font, Length, Pixels, Settings, Size, Task, Theme, font};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};

fn show_dialog(title: &str, msg: &str, kind: MessageLevel) -> MessageDialog {
    MessageDialog::new()
        .set_title(title)
        .set_description(msg)
        .set_level(kind)
        .set_buttons(MessageButtons::YesNo)
}

fn show_success() {
    if let MessageDialogResult::Yes =
        show_dialog("start my job", "report", MessageLevel::Info).show()
    {
        let _ = open::that_detached("https://iced.rs/");
    }
}

#[derive(Default, Debug)]
struct Job {
    name: String,
    num: i32,
}

#[derive(Debug, Clone)]
enum Message {
    TextInput(String),
    Record,
}

#[derive(Default)]
struct App {
    job: Job,
}

impl App {
    fn show(&self) -> Element<'_, Message> {
        container(text("hello"))
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(palette.background.weak.color.into()),
                    border: Border {
                        width: 2.0,
                        color: palette.background.strong.color,
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
            .width(Length::Fill)
            .into()
    }
    fn check(&self) -> bool {
        self.job.name.contains("you")
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::TextInput(res) => {
                let was_checked = self.check();
                self.job.name = res;
                if !was_checked && self.check() {
                    Task::done(Message::Record)
                } else {
                    Task::none()
                }
            }
            Message::Record => {
                self.job.num += 1;
                show_success();
                Task::none()
                // Task::perform(async move { show_success() }, |_| Message::Ignore).discard()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let title = text_input(&self.job.name, &self.job.name)
            .style(|theme: &Theme, status| {
                let default = text_input::default(theme, status);
                text_input::Style {
                    value: if self.check() {
                        Color::from_rgb(0.0, 1.0, 0.0)
                    } else {
                        Color::from_rgb(1.0, 0.0, 0.0)
                    },
                    ..default
                }
            })
            .on_input(Message::TextInput);

        let desc = text!("you change title {} times", self.job.num)
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

        container(column![title, btn_row, self.show()].spacing(10).padding(12))
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
        .window_size(Size {
            width: 400.0,
            height: 300.0,
        })
        .title("CA Analytics")
        .run()
}
