extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

//use gtk::{Box, ContainerExt, WidgetExt};
use gtk::Orientation::{Horizontal};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Keyboard Color Chooser");
    window.set_border_width(175);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 350);

    // Buttons
    let left_button = gtk::Button::new_with_label("Left");
    let center_button = gtk::Button::new_with_label("Center");
    let right_button = gtk::Button::new_with_label("Right");

    let hbox = gtk::Box::new(Horizontal, 10);
    window.add(&hbox);

    // Labels
    //let label = gtk::Label::new(None);
    //label.set_text("Left");
    
    hbox.add(&left_button);
    hbox.add(&center_button);
    hbox.add(&right_button);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.ahoneybun.keyboard-color-chooser"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}
