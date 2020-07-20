use iced::{
    button, executor, text_input, Application, Column, Command, Container, Element, Font, Length,
    Row, Settings, Text, TextInput,
};

const DEFAULT_FONT: Font = Font::External {
    name: "DefaultFont",
    bytes: include_bytes!("resources/Cascadia.ttf"),
};

const MAX_WIDTH: u16 = 500;

pub fn main() {
    Counter::run(Settings::default())
}

#[derive(Default)]
struct Counter {
    input_val: String,
    input_state: text_input::State,
    task_list: Vec<Task>,
    latest_task_id: i32,
}

struct Task {
    id: i32,
    description: String,
    is_complete: bool,
    mark_as_complete_button_state: button::State,
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
        }

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let tasks = self
            .task_list
            .iter_mut()
            .fold(Column::new().spacing(5), |col, task| {
                col.push(
                    Row::new().push(
                        Text::new(format!("• {}. {}", task.id, task.description))
                            .size(18)
                            .font(DEFAULT_FONT),
                    ),
                )
            });

        let content = Column::new()
            .max_width(MAX_WIDTH as u32)
            .spacing(15)
            .padding(40)
            .width(Length::Fill)
            .push(Text::new("master of the todos").size(42).font(DEFAULT_FONT))
            .push(
                TextInput::new(
                    &mut self.input_state,
                    "Placeholder for text input...",
                    &self.input_val,
                    Message::TextInputChanged,
                )
                .width(Length::Units(MAX_WIDTH))
                .padding(10)
                .on_submit(Message::AddNewTask),
            )
            .push(tasks);
        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
