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
    // let (_width, _height) = (50, 150);
    // window.set_default_size(50, 150);

    //Destroy window on exit
    window.connect_delete_event(|_,_| {gtk::main_quit(); Inhibit(false) });

    // Buttons
    let left_button = gtk::ColorButton::new();
    let center_button = gtk::ColorButton::new();
    let right_button = gtk::ColorButton::new();

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

    // Grid
    // let grid = gtk::Grid::new();

    // Boxes
    let buttonbox = gtk::Box::new(Horizontal, 10);

    window.add(&buttonbox);

    // grid.add(&buttonbox);
    
    buttonbox.add(&left_button);
    buttonbox.add(&center_button);
    buttonbox.add(&right_button);

    // labelbox.add(&left_label);
    window.show_all();
    gtk::main();
}
