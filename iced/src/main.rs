mod style;

use iced::{
    button, executor, text_input, Application, Button, Color, Column, Command, Container, Element,
    Font, Length, Row, Settings, Text, TextInput,
};

const DEFAULT_FONT: Font = Font::External {
    name: "DefaultFont",
    bytes: include_bytes!("resources/Cascadia.ttf"),
};

const COLORS: [[u8; 3]; 3] = [[245, 224, 80], [243, 142, 148], [203, 140, 243]];

const MAX_WIDTH: u16 = 800;

pub fn main() {
    Counter::run(Settings::default())
}

struct Counter {
    input_val: String,
    input_state: text_input::State,
    task_list: Vec<Task>,
    latest_task_id: i32,
    color: Color,
    color_palette: Vec<PaletteColor>,
}

impl std::default::Default for Counter {
    fn default() -> Self {
        let palette = vec![
            PaletteColor::new(
                "primary".to_string(),
                Color::from_rgb8(COLORS[0][0], COLORS[0][1], COLORS[0][2]),
            ),
            PaletteColor::new(
                "secondary".to_string(),
                Color::from_rgb8(COLORS[1][0], COLORS[1][1], COLORS[1][2]),
            ),
            PaletteColor::new(
                "tertiary".to_string(),
                Color::from_rgb8(COLORS[2][0], COLORS[2][1], COLORS[2][2]),
            ),
        ];

        return Counter {
            input_val: String::default(),
            input_state: text_input::State::default(),
            task_list: vec![],
            latest_task_id: i32::default(),
            color: palette
                .iter()
                .filter(|&p| p.id == "primary".to_string())
                .collect::<Vec<&PaletteColor>>()
                .first()
                .unwrap()
                .color,
            color_palette: palette,
        };
    }
}

struct Task {
    id: i32,
    description: String,
    is_complete: bool,
    mark_as_complete_button_state: button::State,
}

struct PaletteColor {
    pub id: String,
    pub state: button::State,
    pub color: Color,
}

impl PaletteColor {
    fn new(id: String, color: Color) -> Self {
        return PaletteColor {
            id,
            state: button::State::new(),
            color,
        };
    }
}

impl Task {
    fn new(id: i32, description: String) -> Self {
        Task {
            id,
            description,
            is_complete: false,
            mark_as_complete_button_state: button::State::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
    AddNewTask,
    SetColor(Color),
}

impl Application for Counter {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("A simple counter")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TextInputChanged(s) => {
                self.input_val = s;
            }
            Message::AddNewTask => {
                if !self.input_val.is_empty() {
                    self.task_list
                        .push(Task::new(self.latest_task_id + 1, self.input_val.clone()));
                    self.latest_task_id += 1;
                    self.input_val.clear();
                }
            }
            Message::SetColor(color) => {
                self.color = color;
            }
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let paint_color = self.color;
        let tasks = self
            .task_list
            .iter_mut()
            .fold(Column::new().spacing(5), |col, task| {
                col.push(
                    Row::new().push(
                        Text::new(format!("• {}. {}", task.id, task.description))
                            .size(24)
                            .color(paint_color)
                            .font(DEFAULT_FONT),
                    ),
                )
            });

        let color_palette = self.color_palette.iter_mut().fold(
            Row::new().spacing(20),
            |color_palette_row, palette_color| {
                color_palette_row.push(
                    Button::new(
                        &mut palette_color.state,
                        Text::new("".to_string())
                            // .size(18)
                            .font(DEFAULT_FONT)
                            .color(palette_color.color),
                    )
                    .width(Length::Units(120))
                    .height(Length::Units(120))
                    .style(style::Button::BGColor(palette_color.color))
                    .on_press(Message::SetColor(palette_color.color)),
                )
            },
        );

        let content = Column::new()
            .max_width(MAX_WIDTH as u32)
            .spacing(15)
            .padding(40)
            .width(Length::Fill)
            .push(
                Text::new("What are you gonna accomplish today?")
                    .size(42)
                    .font(DEFAULT_FONT)
                    .color(Color::WHITE),
            )
            .push(
                TextInput::new(
                    &mut self.input_state,
                    "Start writing what's on your mind ...",
                    &self.input_val,
                    Message::TextInputChanged,
                )
                .width(Length::Units(MAX_WIDTH))
                .size(20)
                .padding(10)
                .on_submit(Message::AddNewTask),
            )
            .push(tasks)
            .push(color_palette);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container)
            .into()
    }
}
