extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

//use gtk::{Box, ContainerExt, WidgetExt};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Keyboard Color Chooser");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 150);

    let box = Box::new(gtk::Orientation::Vertical, 12);
    window.add(&box);

    // Buttons
    let left = gtk::Button::new_with_label("Left");
    let center = gtk::Button::new_with_label("Center");
    let right = gtk::Button::new_with_label("Right");

    let container = Box::new(gtk::Orientation::Horizontal, 50);
    box.add(&container);
    

    // Labels
    let label = gtk::Label::new(None);
    label.set_text("Left");
    
    let container2 = Box::new(gtk::Orientation::Horizontal, 100);
    box.add(&container2);
    
    container2.add(&label);

    container.add(&left);
    container.add(&center);
    container.add(&right);

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
