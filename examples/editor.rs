// Topic: Iced

use iced::alignment::Vertical;
use iced::widget::{button, column, markdown, row, scrollable, text, text_editor};
use iced::{Color, Element, Font, Length, Pixels, Settings, Task, Theme, font};

#[derive(Debug, Clone)]
enum Message {
    Ide(text_editor::Action),
    Review,
    MarkdownUrl(markdown::Uri),
}

#[derive(Debug, Default)]
struct App {
    content: text_editor::Content,
    desc: Vec<markdown::Item>,
}

impl App {
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Ide(act) => {
                self.content.perform(act);
                Task::none()
            }
            Message::Review => {
                let res = self.content.text();
                self.desc = markdown::parse(&res).collect();
                Task::none()
            }
            Message::MarkdownUrl(url) => {
                let _ = open::that(url.as_str());
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let preview = scrollable(
            markdown::view(
                &self.desc,
                markdown::Settings::with_text_size(12, self.theme()),
            )
            .map(Message::MarkdownUrl),
        )
        .height(Length::Fill);

        let editor = text_editor(&self.content)
            .placeholder("write markdown here...")
            .on_action(Message::Ide)
            .height(200);

        let review_btn = row![
            text!("click the button to preview")
                .size(10)
                .font(Font {
                    style: font::Style::Italic,
                    ..Default::default()
                })
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(1.0, 0.0, 0.0))
                }),
            button("review").on_press(Message::Review),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        column![preview, editor, review_btn]
            .spacing(8)
            .padding(12)
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
