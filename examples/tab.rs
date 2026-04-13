use iced::border::Radius;
use iced::widget::{Column, button, column, container, row, text, text_input};
use iced::{Border, Element, Length, Task, Theme};

#[derive(Clone)]
enum Message {
    SetFeat(String),
    GoConfig,
    GoInstall,
    GoTweak,
    GoUpdate,
}

#[derive(Default)]
enum App {
    Config {
        dir: String,
        feat: String,
    },
    Install,
    #[default]
    Tweak,
    Update,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        (
            Self::Config {
                dir: "test/config.toml".to_string(),
                feat: "basic".to_string(),
            },
            Task::done(Message::GoUpdate),
        )
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::SetFeat(res) => {
                if let Self::Config { feat, .. } = self {
                    *feat = res;
                }
            }
            Message::GoConfig => {
                *self = Self::Config {
                    dir: "test/config.toml".into(),
                    feat: "basic".into(),
                };
            }
            Message::GoInstall => *self = Self::Install,
            Message::GoTweak => *self = Self::Tweak,
            Message::GoUpdate => *self = Self::Update,
        }
        Task::none()
    }
    fn view(&self) -> Element<'_, Message> {
        let btn = container(
            row![
                container(button("Config").on_press(Message::GoConfig)),
                container(button("Install").on_press(Message::GoInstall)),
                container(button("Tweak").on_press(Message::GoTweak)),
                container(button("Update").on_press(Message::GoUpdate)),
            ]
            .spacing(10),
        )
        .center(Length::Fill)
        .padding(10)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style::default().border(Border {
                color: palette.background.strong.color,
                width: 2.0,
                radius: Radius::new(25.0),
            })
        });

        let ctx: Column<'_, Message> = match self {
            Self::Config { dir, feat } => column![
                text_input("features", feat).on_input(Message::SetFeat),
                text!("your config located {} with {} feature", dir, feat)
            ],
            Self::Update => column![text!("welcome to update software")],
            _ => column![text!("hello")],
        };

        column![btn, ctx].spacing(10).padding(10).into()
    }
    fn theme(&self) -> Theme {
        Theme::Dracula
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .run()
}
