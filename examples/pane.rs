// Topic: Iced
// 1. https://docs.rs/iced/latest/iced/widget/pane_grid/struct.State.html
// 1. pane_grid::State<MyPane>,

use iced::alignment::Vertical;
use iced::widget::{button, column, pane_grid, row, text, text_input};
use iced::{Color, Element, Font, Length, Pixels, Settings, Task, Theme, font};

#[derive(Debug, Clone)]
enum Message {
    TextInput(String),
    PaneDragged(pane_grid::DragEvent),
    PaneResized(pane_grid::ResizeEvent),
    Clicked(pane_grid::Pane),
    Restore,
}

#[derive(Default)]
enum MyPane {
    #[default]
    Pane1,
    Pane2,
}

struct App {
    title: String,
    panes: pane_grid::State<MyPane>,
    focus: Option<pane_grid::Pane>,
}

impl App {
    fn new() -> Self {
        let (mut panes, p1) = pane_grid::State::new(MyPane::default());
        let _ = panes.split(pane_grid::Axis::Vertical, p1, MyPane::Pane2);
        Self {
            title: "hello world".to_string(),
            panes,
            focus: None,
        }
    }
    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::TextInput(res) => {
                self.title = res;
                Task::none()
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
                Task::none()
            }
            Message::PaneDragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
                Task::none()
            }
            Message::PaneDragged(_) => Task::none(),
            Message::Clicked(res) => {
                self.focus = Some(res);
                Task::none()
            }
            Message::Restore => {
                self.panes.restore();
                Task::none()
            }
        }
    }
    fn view(&self) -> Element<'_, Message> {
        let title = text_input(&self.title, &self.title)
            .style(|theme: &Theme, status| {
                let default = text_input::default(theme, status);
                text_input::Style {
                    placeholder: if self.title.contains("you") {
                        Color::from_rgb(0.0, 1.0, 0.0)
                    } else {
                        Color::from_rgb(1.0, 0.0, 0.0)
                    },
                    ..default
                }
            })
            .on_input(Message::TextInput);

        let preview = pane_grid(&self.panes, |pane_id, state, _is_maximized| {
            let is_focused = self.focus == Some(pane_id);
            pane_grid::Content::new(match state {
                MyPane::Pane1 => text("This is some pane").color(if is_focused {
                    Color::from_rgb(129.0 / 255.0, 206.0 / 255.0, 33.0 / 255.0)
                } else {
                    Color::from_rgb(239.0 / 255.0, 106.0 / 255.0, 133.0 / 255.0)
                }),
                MyPane::Pane2 => text("This is another kind of pane"),
            })
        })
        .on_drag(Message::PaneDragged)
        .on_resize(10, Message::PaneResized)
        .on_click(Message::Clicked)
        .height(Length::Fill);

        let btn = row![
            text!("click the button to preview")
                .font(Font {
                    style: font::Style::Italic,
                    ..Default::default()
                })
                .style(|_| text::Style {
                    color: Some(Color::from_rgb(1.0, 0.0, 0.0))
                }),
            button("restore").on_press(Message::Restore),
        ]
        .spacing(10)
        .align_y(Vertical::Center);

        column![title, preview, btn].spacing(8).padding(12).into()
    }
    fn theme(&self) -> Theme {
        Theme::GruvboxLight
    }
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .theme(App::theme)
        .settings(Settings {
            default_text_size: Pixels(12.0),
            ..Default::default()
        })
        .title("CA Analytics")
        .run()
}
