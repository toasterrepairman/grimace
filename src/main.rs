extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, TextView, Window, WindowType, HeaderBar};
use std::env::args;
use gtk::Adjustment;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Transcription App");
    window.set_default_size(600, 400);

    // Create the header bar
    let headerbar = HeaderBar::new();
    headerbar.set_title(Some("Transcription App"));
    headerbar.set_show_close_button(true);
    window.set_titlebar(Some(&headerbar));

    // Create the "Open" button
    let open_button = Button::with_label("Open");
    headerbar.pack_start(&open_button);

    // Create the "Save" button
    let save_button = Button::with_label("Save");
    headerbar.pack_start(&save_button);

    // Create the text view and its buffer
    let text_view = TextView::new();
    let buffer = text_view.buffer().expect("Failed to get buffer.");

    // Add the text view to the main window
    let scrolled_window = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    scrolled_window.add(&text_view);
    window.add(&scrolled_window);

    // Connect signals
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    // Show all the widgets
    window.show_all();

    // Run the main GTK event loop
    gtk::main();
}
