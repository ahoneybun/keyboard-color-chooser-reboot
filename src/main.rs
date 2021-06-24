extern crate gio;
extern crate gtk;

// use gio::prelude::*;
use gtk::prelude::*; 
use gtk::{Window, WindowType};

use gtk::Orientation::Horizontal;

fn main() {

    // Initialize Gtk
    if gtk::init().is_err() {  
        panic!("Can't init GTK");
    }

    // Window Traits
    let window = Window::new(WindowType::Toplevel); 
    window.set_title("Keyboard Color Chooser");
    window.set_border_width(175);
    window.set_position(gtk::WindowPosition::Center);
    // window.set_default_size(50, 150);

    //Destroy window on exit
    window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });

    // Buttons
    let left_button = gtk::Button::with_label("Left");
    let center_button = gtk::Button::with_label("Center");
    let right_button = gtk::Button::with_label("Right");

    // Button Actions
    left_button.connect_clicked(|_| (
        println!("Left Pressed")
    ));

    center_button.connect_clicked(|_| (
        println!("Center Pressed")
    ));

    right_button.connect_clicked(|_| (
        println!("Right Pressed")
    ));

    // Labels
    // let left_label = Label::new(Some("Left"));

    // Grid
    let grid = gtk::Grid::new();

    // Boxes
    let buttonbox = gtk::Box::new(Horizontal, 10);
    // let labelbox = gtk::Box::new(Horizontal, 20);

    window.add(&grid);

    grid.add(&buttonbox);
    // grid.add(&labelbox);
    
    buttonbox.add(&left_button);
    buttonbox.add(&center_button);
    buttonbox.add(&right_button);

    // labelbox.add(&left_label);
    window.show_all();
    gtk::main();
}
