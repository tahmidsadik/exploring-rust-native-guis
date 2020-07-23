use gdk::Screen;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{
    Align, Application, ApplicationWindow, Box, Button, ReliefStyle, TextBuffer, TextIter, TextTag,
    TextView,
};

use std::sync::{Arc, RwLock};

const COLORS: [&str; 3] = ["#F38E94", "#F5E050", "#CC8CF3"];

const STYLE: &str = "
    
    button {
        background-image: none;
        color: black;
        border-radius: 0;
        font-family: Gaegu;
    }

    button:hover {
        transition: 0.1s ease-in;
        font-size: 18px;
    }

    #button-1 {
        background-color: #F5E050;
        border-color: #F5E050;
    }

    #button-2 {
        background-color: #F38E94;
        border-color: #F38E94;
    }

    #button-3 {
        background-color: #CC8CF3;
        border-color: #CC8CF3;
    }

";

fn get_button(name: &str) -> Button {
    let button = Button::new();
    button.set_widget_name(name);
    button.set_size_request(140, 50);
    button.set_margin_start(10);
    button.set_margin_end(10);
    button.set_border_width(0);
    button.set_label(name);
    button.set_relief(ReliefStyle::None);
    return button;
}

fn main() {
    let selected_tag_lock = Arc::new(RwLock::new(String::from("color_tag_1")));
    let application =
        Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Failed to initialize GTK application");
    application.connect_activate(move |app| {
        let provider = gtk::CssProvider::new();
        provider
            .load_from_data(STYLE.as_bytes())
            .expect("Failed to load CSS");
        // We give the CssProvided to the default screen so the CSS rules we added
        // can be applied to our window.
        gtk::StyleContext::add_provider_for_screen(
            &Screen::get_default().expect("Error initializing gtk css provider."),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = ApplicationWindow::new(app);
        window.set_title("First GTK Program");
        window.set_default_size(600, 400);

        let hbox = Box::new(gtk::Orientation::Horizontal, 0);

        let button_container = Box::new(gtk::Orientation::Vertical, 5);
        let btn1 = get_button("button-1");
        let btn2 = get_button("button-2");
        let btn3 = get_button("button-3");

        let st1 = selected_tag_lock.clone();
        btn1.connect_clicked(move |_| {
            let mut stag = st1.write().expect("Error in btn-1 handler");
            *stag = String::from("color_tag_0");
        });

        let st2 = selected_tag_lock.clone();
        btn2.connect_clicked(move |_| {
            let mut stag = st2.write().expect("Error in btn 2 handler");
            *stag = String::from("color_tag_1");
        });

        let st3 = selected_tag_lock.clone();
        btn3.connect_clicked(move |_| {
            let mut stag = st3.write().expect("Error in btn 3 handler");
            *stag = String::from("color_tag_2");
        });

        button_container.pack_start(&btn1, false, false, 0);
        button_container.pack_start(&btn2, false, false, 0);
        button_container.pack_start(&btn3, false, false, 0);

        button_container.set_halign(Align::Center);
        button_container.set_valign(Align::Center);

        hbox.pack_start(&button_container, false, false, 0);

        let tv = TextView::new();
        tv.set_top_margin(15);
        tv.set_bottom_margin(15);
        tv.set_left_margin(20);
        tv.set_right_margin(20);
        let tv_buffer = tv
            .get_buffer()
            .expect("Error: Couldn't get access to the text buffer inside the view");

        let color_tags = COLORS
            .to_vec()
            .iter()
            .enumerate()
            .map(|(idx, color)| {
                gtk::TextTagBuilder::new()
                    .name(format!("color_tag_{}", idx).as_str())
                    .foreground(*color)
                    .font("Gaegu")
                    .build()
            })
            .collect::<Vec<TextTag>>();

        let tag_table = tv_buffer.get_tag_table().unwrap();

        for tag in &color_tags {
            tag_table.add(tag);
        }

        let tt = selected_tag_lock.clone();
        tv_buffer
            .connect("insert_text", true, move |args| {
                let tb = args[0].get::<TextBuffer>().unwrap().unwrap();
                let iter = args[1].get::<TextIter>().unwrap().unwrap();

                let tag = tt.read().expect("Cannot get read lock");
                let tag = tb
                    .get_tag_table()
                    .unwrap()
                    .lookup(tag.as_str())
                    .expect("Couldn't find tag in the tablet list in buffer");
                tb.apply_tag(
                    &tag,
                    &tb.get_iter_at_offset(&iter.get_offset() - 1),
                    &tb.get_iter_at_offset(iter.get_offset()),
                );

                None
            })
            .expect("Couldn't connect to signal insert_text");

        hbox.pack_start(&tv, true, true, 0);
        window.add(&hbox);
        // window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
