use std::process::Command;

use iced::alignment::Horizontal;
use iced::widget::text_editor::Position;
use iced::widget::{
    button, checkbox, column, container, row, rule, scrollable, stack, text, text_editor,
};
use iced::{Alignment, Color, Element, Font, Length, Task, Theme, clipboard};

const MESLO: Font = Font::with_name("MesloLGM Nerd Font");

#[derive(Clone)]
enum Message {
    Edit(text_editor::Action),
    SaveFile(bool),
    Run(String),
    RunFinished(String),
    Clip,
}

#[derive(Default)]
struct App {
    content: text_editor::Content,
    save: bool,
    lang: String,
    output: String,
}

async fn apply(lang: String, code: String, is_save: bool) -> String {
    let out = match lang.as_str() {
        // python3 -c "print(1+1)"
        "py" => Command::new("python3").args(["-c", code.as_str()]).output(),
        // R -s -e "head(mtcars)"
        "r" => Command::new("R").args(["-s", "-e", &code]).output(),
        // ~/duckdb -c "select now()"
        "sql" => Command::new("/home/stli/duckdb")
            .args(["-c", &code])
            .output(),
        _ => return "unknown lang".into(),
    };
    match out {
        Ok(x) => {
            if x.status.success() {
                let mut res = String::new();
                res.push_str(
                    String::from_utf8_lossy(&x.stdout)
                        .as_ref()
                        .replace("\r\n", "\n")
                        .trim(),
                );
                res.push('\n');
                if is_save {
                    res.push_str(&save_file(lang, code));
                }
                res
            } else {
                String::from_utf8_lossy(&x.stderr).to_string()
            }
        }
        Err(e) => format!("{}: {}", e.kind(), e),
    }
}

fn save_file(lang: String, code: String) -> String {
    std::fs::write(format!("./test.{}", lang), code).expect("fail to write a file");
    format!("save file to ./test.{}", lang)
}

impl App {
    fn set_lang(&mut self, x: &str) {
        self.lang = match x {
            "py" => "python".into(),
            "r" => "r".into(),
            "sql" => "sql".into(),
            _ => "txt".into(),
        }
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Edit(x) => {
                self.content.perform(x);
                Task::none()
            }
            Message::SaveFile(x) => {
                self.save = x;
                Task::none()
            }
            Message::Run(x) => {
                self.set_lang(&x);
                let code = self.content.text();
                Task::perform(apply(x, code, self.save), Message::RunFinished)
            }
            Message::RunFinished(x) => {
                self.output = x;
                Task::none()
            }
            Message::Clip => {
                let res = self.output.clone();
                clipboard::write(res)
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let input = text_editor(&self.content)
            .on_action(Message::Edit)
            .placeholder("type Python/R/SQL code here")
            .highlight(&self.lang, iced_highlighter::Theme::Base16Ocean)
            .height(300)
            .padding(10);

        let sep = rule::horizontal(2).style(|_| rule::Style {
            color: Color::from_rgb8(212, 38, 100),
            radius: 0.0.into(),
            fill_mode: rule::FillMode::Full,
            snap: true,
        });

        let stat_bar = {
            let btn_lang = row![
                button(text("PYTHON").align_x(Alignment::Center))
                    .padding([5, 10])
                    .width(Length::Fill)
                    .on_press(Message::Run("py".into())),
                button(text("R").align_x(Alignment::Center))
                    .padding([5, 10])
                    .width(Length::Fill)
                    .on_press(Message::Run("r".into())),
                button(text("SQL").align_x(Alignment::Center))
                    .padding([5, 10])
                    .width(Length::Fill)
                    .on_press(Message::Run("sql".into())),
                checkbox(self.save)
                    .label("save")
                    .on_toggle(Message::SaveFile)
                    .width(Length::Fill),
            ]
            .spacing(2);

            let pos = {
                let Position { line, column: col } = self.content.cursor().position;
                text!("[{}-{}]", line + 1, col + 1).size(16)
            }
            .width(Length::Fill)
            .align_x(Horizontal::Right);

            row![btn_lang, pos].width(Length::Fill).padding(10)
        };

        let res = stack!(
            container(
                scrollable(text(&self.output).size(14))
                    .height(300)
                    .width(Length::Fill)
            )
            .padding(10)
            .style(|_theme| container::Style {
                background: Some(Color::from_rgb8(248, 249, 250).into()),
                border: iced::Border {
                    color: Color::from_rgb8(80, 80, 80),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }),
            container(button(text("Copy").font(MESLO)).on_press(Message::Clip))
                .align_right(Length::Fill),
        );

        container(column![input, sep, stat_bar, res].spacing(12))
            .padding(10)
            .into()
    }
    fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }
}

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .theme(App::theme)
        .title("CA Analytics")
        .run()
}
