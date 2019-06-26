extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

//use gtk::{Box, ContainerExt, WidgetExt};
use gtk::{GridExt};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Keyboard Color Chooser");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 150);

    // Buttons
    let leftButton = gtk::Button::new_with_label("Left");
    let centerButton = gtk::Button::new_with_label("Center");
    let rightButton = gtk::Button::new_with_label("Right");

    let grid = GridExt:;
    window.add(&grid);

    // Labels
    //let label = gtk::Label::new(None);
    //label.set_text("Left");
    
    grid.add(&leftButton);
    grid.add(&centerButton);
    grid.add(&rightButton);

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
