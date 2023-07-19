use gtk::prelude::*;
use gtk::{FileChooserAction, FileChooserButton, HeaderBar, TextView, Window, WindowType};

fn main() {
    // Initialize GTK and create the main window
    gtk::init().expect("Failed to initialize GTK.");
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Transcription App");
    window.set_default_size(600, 400);

    // Create a header bar and add "Open" and "Save" buttons
    let header_bar = HeaderBar::new();
    header_bar.set_title(Some("Transcription App"));
    header_bar.set_show_close_button(true);

    let open_button = FileChooserButton::new("Open", FileChooserAction::Open);
    let save_button = FileChooserButton::new("Save", FileChooserAction::Save);

    header_bar.pack_start(&open_button);
    header_bar.pack_end(&save_button);

    // Create a text view with a buffer
    let text_view = TextView::new();

    // Set up the window layout
    let layout = gtk::Box::new(gtk::Orientation::Vertical, 0);
    layout.pack_start(&header_bar, false, false, 0);
    layout.pack_start(&text_view, true, true, 0);

    window.add(&layout);
    window.show_all();

    // Connect the "destroy" signal to terminate the GTK main loop
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
