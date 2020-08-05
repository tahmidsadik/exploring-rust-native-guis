extern crate pango;

use gio::prelude::*;
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

mod tags;
mod text_ops;
use tags::{TagType, TextTagState};
use text_ops::{DeleteTextEventData, InsertOpsData, InsertTextEventData, Ops};

const COLORS: [&str; 3] = ["#F5E050", "#F38E94", "#CC8CF3"];
const NOTE_FILE_NAME: &str = "note-backup.bin";

struct Model {
    current_tag: String,
    previous_tag: String,
    ops: Vec<Ops>,
    is_hydrating: bool,
    relm: Relm<Win>,
    /**
     * should probably use a hashmap here
     */
    italic_tag_state: TextTagState,
}

#[derive(Msg)]
enum Msg {
    Quit,
    InsertText(InsertTextEventData),
    DeleteText(DeleteTextEventData),
    SelectColor(String),
    SaveNote,
    Hydrate,
    SetHydrating(bool),
    UpdateTagState((bool, i32)),
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
            Ops::Insert(insert_ops_data) => {
                self.model
                    .relm
                    .stream()
                    .clone()
                    .emit(Msg::SelectColor(self.model.current_tag.clone()));

                self.widgets
                    .buffer
                    .insert_at_cursor(insert_ops_data.content.as_str());

                let tag_table = self
                    .widgets
                    .buffer
                    .get_tag_table()
                    .expect("Couldn't get hold of a tag table!");

                let tag = tag_table
                    .lookup(insert_ops_data.tag.as_str())
                    .expect("Fatal: Cannot find tag color_tag_1");

                let cursor_offset = self.widgets.buffer.get_property_cursor_position();

                self.widgets.buffer.apply_tag(
                    &tag,
                    &self.widgets.buffer.get_iter_at_offset(cursor_offset - 1),
                    &self.widgets.buffer.get_iter_at_offset(cursor_offset),
                );
            }
            Ops::Delete(offsets) => {
                let (start_offset, end_offset) = offsets;
                self.widgets.buffer.delete(
                    &mut self.widgets.buffer.get_iter_at_offset(start_offset),
                    &mut self.widgets.buffer.get_iter_at_offset(end_offset),
                );
            }
            Ops::MoveCursor(position) => self
                .widgets
                .buffer
                .place_cursor(&self.widgets.buffer.get_iter_at_offset(position)),
            Ops::SelectColorTag(color) => {
                self.model.previous_tag = self.model.current_tag.to_string();
                self.model.current_tag = color.to_string();
            }
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

    fn model(relm: &Relm<Self>, _: ()) -> Model {
        Model {
            current_tag: String::from("color_tag_1"),
            previous_tag: String::from("color_tag_1"),
            ops: vec![],
            is_hydrating: true,
            relm: relm.clone(),
            italic_tag_state: TextTagState::new(TagType::Italic, '*'),
        }
    }

    fn update(&mut self, event: Msg) {
        let tb = &self.widgets.buffer;

        match event {
            Msg::SelectColor(color) => {
                self.model.ops.push(Ops::SelectColorTag(color.to_string()));
                self.model.previous_tag = self.model.current_tag.clone();
                self.model.current_tag = color;
            }
            Msg::SetHydrating(hydrating) => {
                self.model.is_hydrating = hydrating;
            }
            Msg::InsertText(insert_text_data) => {
                if self.model.is_hydrating == false {
                    let tag_table = tb
                        .get_tag_table()
                        .expect("Couldn't get hold of a tag table!");

                    let tag = tag_table
                        .lookup(self.model.current_tag.as_str())
                        .expect("Fatal: Cannot find tag color_tag_1");

                    tb.apply_tag(
                        &tag,
                        &tb.get_iter_at_offset(insert_text_data.offset),
                        &tb.get_iter_at_offset(insert_text_data.offset + 1),
                    );

                    if insert_text_data.content.chars().nth(0).unwrap()
                        == self.model.italic_tag_state.tag_symbol
                    {
                        self.model
                            .relm
                            .stream()
                            .clone()
                            .emit(Msg::UpdateTagState((true, insert_text_data.offset)));
                    }
                }

                self.model.ops.push(Ops::Insert(InsertOpsData::new(
                    String::from(insert_text_data.content),
                    self.model.current_tag.to_string(),
                )));
            }
            Msg::UpdateTagState((status, offset)) => {
                if self.model.italic_tag_state.is_active == false {
                    self.model.italic_tag_state.is_active = status;
                    self.model.italic_tag_state.start_offset = offset;
                } else {
                    let buffer = &self.widgets.buffer;

                    let first_char = buffer
                        .get_text(
                            &buffer
                                .get_iter_at_offset(self.model.italic_tag_state.start_offset + 1),
                            &buffer
                                .get_iter_at_offset(self.model.italic_tag_state.start_offset + 2),
                            false,
                        )
                        .expect("Cannot unwrap first char in UpdateTagState");

                    //check if first_char after first_symbol is a space
                    let last_char = buffer
                        .get_text(
                            &buffer.get_iter_at_offset(offset - 1),
                            &buffer.get_iter_at_offset(offset),
                            false,
                        )
                        .expect("Cannot unwrap last char in UpdateTagState");

                    if first_char == String::from(" ") || last_char == String::from(" ") {
                        self.model.italic_tag_state.start_offset = offset;
                    } else {
                        let tag = buffer
                            .get_tag_table()
                            .unwrap()
                            .lookup("italic")
                            .expect("Fatal: Cannot find tag color_tag_1");

                        buffer.apply_tag(
                            &tag,
                            &buffer
                                .get_iter_at_offset(self.model.italic_tag_state.start_offset + 1),
                            &buffer.get_iter_at_offset(offset - 1),
                        );

                        buffer.delete(
                            &mut buffer
                                .get_iter_at_offset(self.model.italic_tag_state.start_offset),
                            &mut buffer
                                .get_iter_at_offset(self.model.italic_tag_state.start_offset + 1),
                        );

                        buffer.delete(
                            &mut buffer.get_iter_at_offset(offset - 1),
                            &mut buffer.get_iter_at_offset(offset),
                        );

                        self.model.italic_tag_state.start_offset = -1;
                        self.model.italic_tag_state.is_active = false;
                    }
                }
            }
            Msg::DeleteText(delete_text_event_data) => self.model.ops.push(Ops::Delete((
                delete_text_event_data.start_offset,
                delete_text_event_data.end_offset,
            ))),
            Msg::SaveNote => match bincode::serialize(&self.model.ops) {
                Ok(serialized_note) => {
                    std::fs::write(NOTE_FILE_NAME, serialized_note);
                }
                Err(err) => {
                    show_error_dialog(err.to_string().as_str());
                }
            },
            Msg::Hydrate => {
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
                };

                self.model
                    .relm
                    .stream()
                    .clone()
                    .emit(Msg::SetHydrating(false));

                // timeout(self.model.relm.stream(), 1000 as u32, ||  Msg::SetHydrating(false));
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

        let hbox = Box::new(Horizontal, 0);
        let button_box = gtk::Box::new(Vertical, 10);
        button_box.set_size_request(220, 220);

        let tv = TextView::new();
        let buffer = match tv.get_buffer() {
            Some(buffer) => buffer,
            None => {
                show_error_dialog("Error: No text buffer found in the text view");
                panic!("Fatal: Cannot retrieve text buffer from gtk text view");
            }
        };

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
        let mut color_tags = COLORS
            .to_vec()
            .iter()
            .enumerate()
            .map(|(idx, color)| {
                gtk::TextTagBuilder::new()
                    .name(format!("color_tag_{}", idx + 1).as_str())
                    .size_points(16.0)
                    .foreground(*color)
                    .font("Mononoki Nerd Font Mono")
                    .build()
            })
            .collect::<Vec<TextTag>>();

        let italic_tag = gtk::TextTagBuilder::new()
            .name("italic")
            .style(pango::Style::Italic)
            .build();
        color_tags.push(italic_tag);

        let tag_table = buffer.get_tag_table().unwrap();

        for tag in &color_tags {
            tag_table.add(tag);
        }

        relm.stream().clone().emit(Msg::Hydrate);

        connect!(
            relm,
            buffer,
            connect_insert_text(_, iter, content),
            Msg::InsertText(InsertTextEventData::new(iter.get_offset(), content))
        );

        connect!(
            relm,
            buffer,
            connect_delete_range(_, s_itr, e_itr),
            Msg::DeleteText(DeleteTextEventData::new(
                s_itr.get_offset(),
                e_itr.get_offset()
            ))
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

        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

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
