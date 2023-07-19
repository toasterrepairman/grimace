extern crate gtk;
use gtk::prelude::*;
use gtk::{Button, TextView, Window, WindowType, HeaderBar, Adjustment, Popover, ComboBoxText};
use mutter::{Model, ModelType};
use strum::IntoEnumIterator;

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

    // Create the menu button
    let menu_button = Button::with_label("Menu");
    headerbar.pack_end(&menu_button);

    // Create the popover for the menu
    let popover = Popover::new(Some(&menu_button));
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);

    // Create the ComboBox
    let combo_box = ComboBoxText::new();
    combo_box.append_text("Tiny");
    combo_box.append_text("Small");
    combo_box.append_text("Medium");
    combo_box.set_active(Some(0));
    vbox.pack_start(&combo_box, false, false, 5);

    // Create the "Download Model" button
    let download_button = Button::with_label("Download Model");
    vbox.pack_start(&download_button, false, false, 0);

    popover.add(&vbox);
    menu_button.connect_clicked(move |_| {
        popover.show_all();
    });
    download_button.connect_activate(move |_| {

    });

    // Create the text view and its buffer
    let text_view = TextView::new();
    let buffer = text_view.buffer().expect("Failed to get buffer.");

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
