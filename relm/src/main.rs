use gtk::prelude::*;
use gtk::Orientation::{Horizontal, Vertical};
use gtk::{
    Align, Box, Button, ButtonExt, ContainerExt, Inhibit, Label, TextBuffer, TextTag, TextView,
    WidgetExt, Window, WindowType,
};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use std::fs::File;
use std::io::Read;

mod text_ops;
use text_ops::Ops;

const COLORS: [&str; 3] = ["#F5E050", "#F38E94", "#CC8CF3"];
const NOTE_FILE_NAME: &str = "note-backup.bin";

struct InsertTextEventData {
    offset: i32,
    content: String,
}

struct TextDelta {
    text: String,
    tag: String,
    offset_start: i32,
    offset_end: i32,
}

impl TextDelta {
    fn new(text: String, tag: String) -> Self {
        return TextDelta {
            text,
            tag,
            offset_start: 0,
            offset_end: 0,
        };
    }
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
    current_tag: String,
    previous_tag: String,
    ops: Vec<Ops>,
}

#[derive(Msg)]
enum Msg {
    Quit,
    InsertText(InsertTextEventData),
    SelectColor(String),
    SaveNote,
    Hydrate,
}

fn get_button_with_label(name: &str) -> Button {
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
    dialog.connect_response(|d, _r| {
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
    save_button: Button,
    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Win {
    fn apply_ops(&mut self, op: Ops) {
        match op {
            Ops::Insert(text) => {
                self.widgets.buffer.insert_at_cursor(text.as_str());
            }
            Ops::Delete(count) => {
                let offset_start = self.widgets.buffer.get_property_cursor_position();
                self.widgets.buffer.delete(
                    &mut self.widgets.buffer.get_iter_at_offset(offset_start),
                    &mut self.widgets.buffer.get_iter_at_offset(offset_start + count),
                );
            }
            Ops::MoveCursor(position) => self
                .widgets
                .buffer
                .place_cursor(&self.widgets.buffer.get_iter_at_offset(position)),
        }
    }
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
            current_tag: String::from("color_tag_1"),
            previous_tag: String::from("color_tag_1"),
            ops: vec![],
        }
    }

    fn update(&mut self, event: Msg) {
        let tb = &self.widgets.buffer;

        match event {
            Msg::SelectColor(color) => {
                self.model.previous_tag = self.model.current_tag.clone();
                self.model.current_tag = color;
            }
            Msg::InsertText(s) => {
                let tag_table = tb
                    .get_tag_table()
                    .expect("Couldn't get hold of a tag table!");

                let tag = tag_table
                    .lookup(self.model.current_tag.as_str())
                    .expect("Fatal: Cannot find tag color_tag_1");

                tb.apply_tag(
                    &tag,
                    &tb.get_iter_at_offset(s.offset),
                    &tb.get_iter_at_offset(s.offset + 1),
                );

                self.model.ops.push(Ops::Insert(s.content));

                println!("ops = {:?}", self.model.ops);

                // tb.serialize();
                // let format = tb.register_serialize_tagset(None);

                // let serialized =
                //     tb.serialize(tb, &format, &tb.get_start_iter(), &tb.get_end_iter());

                // for f in formats {
                //     println!("{:?}", f);
                // }

                // find text delta
                // let search_iter = tb.get_start_iter();
                // match search_iter.forward_search(
                //     "Hello",
                //     gtk::TextSearchFlags::CASE_INSENSITIVE,
                //     None,
                // ) {
                //     Some((res_start_iter, res_end_iter)) => println!(
                //         "{}, {}",
                //         res_start_iter.get_offset(),
                //         res_end_iter.get_offset()
                //     ),
                //     None => {}
                // };
            }
            Msg::SaveNote => {
                println!("Saving Note");
                println!("{:?}", self.model.ops);
                match bincode::serialize(&self.model.ops) {
                    Ok(serialized_note) => {
                        println!(
                            "serialized note = {}",
                            String::from_utf8_lossy(&serialized_note)
                        );
                        std::fs::write(NOTE_FILE_NAME, serialized_note);
                    }
                    Err(err) => {
                        show_error_dialog(err.to_string().as_str());
                    }
                }
            }
            Msg::Hydrate => {
                println!("Hydrating...");
                match File::open(NOTE_FILE_NAME) {
                    Ok(mut file) => {
                        let mut buf: Vec<u8> = vec![];
                        match file.read_to_end(&mut buf) {
                            Ok(_size) => {
                                let ops = bincode::deserialize::<Vec<Ops>>(&buf)
                                    .expect("Error trying to deserialize binary data");
                                for op in ops {
                                    self.apply_ops(op);
                                }
                            }
                            Err(err) => {
                                show_error_dialog(err.to_string().as_str());
                            }
                        }
                    }
                    Err(err) => {
                        show_error_dialog(err.to_string().as_str());
                    }
                }
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

        relm.stream().clone().emit(Msg::Hydrate);
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

        // let ops = vec![
        //     Ops::Insert("h".to_string()),
        //     Ops::Insert("e".to_string()),
        //     Ops::Insert("l".to_string()),
        //     Ops::Insert("l".to_string()),
        //     Ops::Insert("o".to_string()),
        //     Ops::Insert("!".to_string()),
        //     Ops::MoveCursor(2),
        //     Ops::Delete(2),
        // ];
        //
        // for op in ops {
        //     match op {
        //         Ops::Insert(text) => {
        //             buffer.insert_at_cursor(text.as_str());
        //         }
        //         Ops::Delete(count) => {
        //             let offset_start = buffer.get_property_cursor_position();
        //             buffer.delete(
        //                 &mut buffer.get_iter_at_offset(offset_start),
        //                 &mut buffer.get_iter_at_offset(offset_start + count),
        //             );
        //         }
        //         Ops::MoveCursor(position) => {
        //             buffer.place_cursor(&buffer.get_iter_at_offset(position))
        //         }
        //     }
        // }
        //
        let btn1 = get_button_with_label(COLORS[0]);
        let btn2 = get_button_with_label(COLORS[1]);
        let btn3 = get_button_with_label(COLORS[2]);
        let save_button = get_button_with_label("Save Note");

        tv.set_left_margin(20);
        tv.set_right_margin(20);
        tv.set_top_margin(15);
        tv.set_bottom_margin(15);

        button_box.add(&btn1);
        button_box.add(&btn2);
        button_box.add(&btn3);
        button_box.pack_end(&save_button, false, false, 10);
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
                    .size_points(16.0)
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

        connect!(relm, save_button, connect_clicked(_), Msg::SaveNote);
        Win {
            model,
            widgets: Widgets {
                window,
                buffer,
                save_button,
            },
        }
    }
}

fn main() {
    Win::run(()).expect("Win::run failed");
}
