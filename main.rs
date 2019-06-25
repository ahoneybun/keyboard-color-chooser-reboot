extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Keyboard Color Chooser");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(500, 150);

    let button1 = gtk::Button::new_with_label("Left");
    //let button2 = gtk::Button::new_with_label("Center");
    //let button3 = gtk::Button::new_with_label("Right");

    window.add(&button1);
    //window.add(&button2);
    //window.add(&button3);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}

