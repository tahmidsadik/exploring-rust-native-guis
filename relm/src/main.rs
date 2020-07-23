use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Align, Box, Button, ButtonExt, ContainerExt, Inhibit, Label, TextBuffer, TextTag, TextView,
    WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
const COLORS: [&str; 3] = ["#F5E050", "#F38E94", "#CC8CF3"];

struct InsertTextEventData {
    offset: i32,
    content: String,
}

impl InsertTextEventData {
    fn new(offset: i32, content: &str) -> Self {
        InsertTextEventData {
            offset,
            content: String::from(content),
        }
    }
}

struct Model {
    selected_tag: String,
}

#[derive(Msg)]
enum Msg {
    Quit,
    InsertText(InsertTextEventData),
    SelectColor(String),
}

fn get_button(name: &str) -> Button {
    let button = Button::new();
    button.set_widget_name(name);
    button.set_size_request(140, 50);
    button.set_margin_start(10);
    button.set_margin_end(10);
    button.set_border_width(0);
    button.set_label(name);
    return button;
}

fn show_error_dialog(error_msg: &str) {
    let dialog = gtk::Dialog::new();
    dialog.set_title("Error");
    dialog.add_button("Okay", gtk::ResponseType::Ok);
    dialog.set_valign(Align::Center);
    dialog.connect_response(|d, r| {
        d.close();
    });
    dialog.set_size_request(400, 250);
    let content_box = dialog.get_content_area();
    let error_label = Label::new(Some(error_msg));
    content_box.pack_start(&error_label, false, false, 0);

    dialog.show_all();
}

// Create the structure that holds the widgets used in the view.
#[derive(Clone)]
struct Widgets {
    buffer: TextBuffer,
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    // Specify the model used for this widget.
    type Model = Model;
    // Specify the model parameter used to init the model.
    type ModelParam = ();
    // Specify the type of the messages sent to the update function.
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            selected_tag: String::from("color_tag_1"),
        }
    }

    fn update(&mut self, event: Msg) {
        let tb = &self.widgets.buffer;

        match event {
            Msg::SelectColor(color) => {
                self.model.selected_tag = color;
            }
            Msg::InsertText(s) => {
                let tag_table = tb
                    .get_tag_table()
                    .expect("Couldn't get hold of a tag table!");

                let tag = tag_table
                    .lookup(self.model.selected_tag.as_str())
                    .expect("Fatal: Cannot find tag color_tag_1");

                tb.apply_tag(
                    &tag,
                    &tb.get_iter_at_offset(s.offset),
                    &tb.get_iter_at_offset(s.offset + 1),
                );
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for Win {
    // Specify the type of the root widget.
    type Root = Window;

    // Return the root widget.
    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        // Create the view using the normal GTK+ method calls.
        //
        let hbox = Box::new(Horizontal, 0);
        let button_box = gtk::Box::new(Vertical, 10);
        button_box.set_size_request(220, 220);

        let tv = TextView::new();
        let buffer = match tv.get_buffer() {
            Some(buffer) => buffer,
            None => {
                show_error_dialog("Error: Now text buffer found in the text view");
                panic!("Fatal: Cannot retrieve text buffer from gtk text view");
            }
        };

        let btn1 = get_button(COLORS[0]);
        let btn2 = get_button(COLORS[1]);
        let btn3 = get_button(COLORS[2]);

        tv.set_left_margin(20);
        tv.set_right_margin(20);
        tv.set_top_margin(15);
        tv.set_bottom_margin(15);

        button_box.add(&btn1);
        button_box.add(&btn2);
        button_box.add(&btn3);
        hbox.pack_start(&button_box, false, false, 0);
        hbox.pack_start(&tv, true, true, 0);

        let window = Window::new(WindowType::Toplevel);
        window.set_title("Pretty Notes");
        window.set_size_request(600, 500);

        window.add(&hbox);
        window.show_all();

        // TODO: move this into a sepaate function
        let color_tags = COLORS
            .to_vec()
            .iter()
            .enumerate()
            .map(|(idx, color)| {
                gtk::TextTagBuilder::new()
                    .name(format!("color_tag_{}", idx + 1).as_str())
                    .size_points(13.0)
                    .foreground(*color)
                    .font("Gaegu")
                    .build()
            })
            .collect::<Vec<TextTag>>();

        let tag_table = buffer.get_tag_table().unwrap();

        for tag in &color_tags {
            tag_table.add(tag);
        }

        connect!(
            relm,
            buffer,
            connect_insert_text(_tb, iter, content),
            Msg::InsertText(InsertTextEventData::new(iter.get_offset(), content))
        );
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        connect!(
            relm,
            btn1,
            connect_clicked(_),
            Msg::SelectColor(String::from("color_tag_1"))
        );

        connect!(
            relm,
            btn2,
            connect_clicked(_),
            Msg::SelectColor(String::from("color_tag_2"))
        );

        connect!(
            relm,
            btn3,
            connect_clicked(_),
            Msg::SelectColor(String::from("color_tag_3"))
        );
        Win {
            model,
            widgets: Widgets { window, buffer },
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}