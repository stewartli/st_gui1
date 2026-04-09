// Topic: Iced
// rfd = { version = "0.17.2", default-features = false, features = ["gtk3"] }
// 1. sudo apt install libgtk-3-dev
// 2. sudo apt install xdg-desktop-portal xdg-desktop-portal-gtk libgtk-3-dev

use std::path::PathBuf;
use std::process::Command;

use iced::alignment::Vertical;
use iced::widget::{button, column, container, pick_list, row, text, text_input};
use iced::window;
use iced::{Element, Length, Size, Task, Theme};

#[derive(Debug, Default, Clone)]
struct Job {
    name: String,
    entry: PathBuf,
    kind: String,
}

impl Job {
    fn pick(&mut self) {
        if let Some(res) = rfd::FileDialog::new().pick_file() {
            self.entry = res;
        }
    }
    fn process(&self) -> u8 {
        if self.entry.as_os_str().is_empty() {
            return 1;
        }
        let res = Command::new("Rscript")
            .arg(&self.entry)
            .status()
            .expect("fail to execute script");
        if res.success() { 0 } else { 1 }
    }
}

const JOB_KIND: [&str; 3] = ["adhoc", "urgent", "routine"];

#[derive(Debug, Clone)]
enum Message {
    Name(String),
    Kind(String),
    Entry,
    Process,
    End(u8),
}

#[derive(Debug, Default)]
struct App {
    job: Job,
}

impl App {
    fn show(&self) {
        println!(
            "\x1b[31m[{}]\x1b[0m {} executes {}",
            self.job.kind,
            self.job.name,
            self.job.entry.display()
        );
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Name(name) => {
                self.job.name = name;
                Task::none()
            }
            Message::Kind(kind) => {
                self.job.kind = kind;
                Task::none()
            }
            Message::Entry => {
                self.job.pick();
                Task::none()
            }
            Message::Process => {
                let job = self.job.clone();
                Task::perform(async move { job.process() }, Message::End)
            }
            Message::End(res) => {
                if res > 0 {
                    println!("job status: {res}");
                } else {
                    self.show();
                }
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let input = text_input("job name", &self.job.name).on_input(Message::Name);

        let selected = if self.job.kind.is_empty() {
            Some(JOB_KIND[0])
        } else {
            Some(self.job.kind.as_str())
        };
        let pick = pick_list(JOB_KIND, selected, |x| Message::Kind(x.to_string()))
            .placeholder("pick a job");

        let entry_btn = button("entry point").on_press(Message::Entry);
        let run_btn = button("run").on_press(Message::Process);

        column![
            container(
                column![
                    text!("create a job to execute").size(20),
                    row![input, pick, entry_btn, run_btn]
                        .spacing(10)
                        .align_y(Vertical::Center),
                ]
                .spacing(10),
            )
            .padding(10)
            .width(Length::Fill)
            .style(|_theme| {
                container::Style {
                    background: Some(iced::Color::from_rgb8(220, 210, 170).into()),
                    text_color: None,
                    ..Default::default()
                }
            })
        ]
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
        .window(window::Settings {
            size: Size {
                width: 600.0,
                height: 180.0,
            },
            resizable: false,
            ..Default::default()
        })
        .run()
}
