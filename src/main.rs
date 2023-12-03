extern crate gtk;

use gio::glib::num_processors;
use gtk::prelude::*;
use gdk::{keys::constants as key};
use gtk::{Button, TextView, Window, WindowType, Statusbar, HeaderBar, Adjustment, Popover, ComboBoxText, TextBuffer, Label, MessageDialog, DialogFlags, MessageType, ButtonsType, Align};
use mutter::{Model, ModelType};
use rfd::FileDialog;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use std::fs::create_dir_all;
use words_count;

struct TranscriptionModel {
    name: String,
    download_link: String,
    filename: String,
}

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Grimace");
    window.set_default_size(500, 300);

    // Create the header bar
    let headerbar = HeaderBar::new();
    headerbar.set_title(Some("Grimace"));
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
    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 10);
    vbox.set_border_width(5);

    // Create the label
    let download_label = Label::new(Some("Model selection"));
    vbox.pack_start(&download_label, false, false, 0);

    // Create the ComboBox
    let combo_box = ComboBoxText::new();
    vbox.pack_start(&combo_box, false, false, 0);

    // Create the "Download Model" button
    let download_button = Button::with_label("Download Model");
    vbox.pack_start(&download_button, false, false, 0);

    popover.add(&vbox);
    menu_button.connect_clicked(move |_| {
        popover.show_all();
    });

    // Create the text view and its buffer
    let text_view = TextView::new();
    let buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&buffer));
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_border_width(10);
    text_view.set_editable(false);
    text_view.set_monospace(true);
    text_view.set_cursor_visible(false);

    // Add the text view to the main window
    let main_box = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let scrolled_window = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    scrolled_window.add(&text_view);
    scrolled_window.set_vexpand(true);
    main_box.add(&scrolled_window);
    window.add(&main_box);

    // buffer factory factory
    let buffer_factory = text_view.buffer().unwrap();

    // Create a label for each status bar section
    let status_label1 = Label::new(Some(""));
    let status_label2 = Label::new(Some(""));

    // Create the status bar and add the labels
    let status_bar = Statusbar::new();
    status_bar.pack_start(&status_label1, true, false, 5);
    status_bar.pack_start(&status_label2, true, false, 5);

    // Align the status bar to the bottom of the window
    main_box.pack_end(&status_bar, false, true, 0);
    status_bar.set_halign(Align::Center);
    status_bar.set_valign(Align::Center);

    open_button.connect_clicked(move |_| {
        let file = format!("{}", FileDialog::new()
            .set_directory("~/")
            .add_filter("Audio", &["mp3", "wav", "flac", "ogg"])
            .pick_file()
            .unwrap_or_else(|| dirs::home_dir().unwrap()) // FIXME
            .display());

        let home_dir = dirs::home_dir().unwrap();
        let ai_dir = home_dir.join(".ai");
        let model_path = ai_dir.join("ggml-tiny.bin");

        let model_result = Model::new(model_path.to_str().unwrap());

        if let Err(_) = &model_result {
            println!("Cannot find model");
            show_message_popup("Cannot find model");
        } else {
            println!("Found model");
            // You can use the 'model' instance here

            if let Err(_) = std::fs::read(&file) {
                println!("Please select an audio file from the picker");
                show_message_popup("Please select an audio file from the picker");
            } else {
                println!("Found model");
            }
            
            let file_stream = std::fs::read(file).unwrap();
            let transcription = model_result.unwrap()
                .transcribe_audio(file_stream, false, false, Some(2))
                .unwrap();
            println!("{}", transcription.as_text());
            buffer.set_text(&format!("{}", transcription.as_text()));
        }
    });

    &text_view.buffer().unwrap().connect_changed(move |_| {
        // Summoning ritual for contents of textview
        let contents = &text_view.buffer()
            .unwrap().text(&text_view.buffer().unwrap().start_iter(),
                  &text_view.buffer().unwrap().end_iter(),
                  false).unwrap();

        status_label1.set_label(&format!("Characters: {:?}", &contents.chars().count()));
        status_label2.set_label(&format!("Words: {:?}", words_count::count(&contents).words));
    });


    save_button.connect_clicked(move |_|{
        let file = format!("{}", FileDialog::new()
            .set_directory("~/")
            .add_filter("Text", &["txt", ""])
            .save_file()
            .unwrap()
            .display());
        let path = Path::new(&file);
        let mut file = File::create(&path).unwrap();
        file.write_all(&buffer_factory.text(&buffer_factory.start_iter(), &buffer_factory.end_iter(), false).unwrap().as_bytes()).unwrap();
    });

    download_button.connect_clicked(move |_| {
        if let Err(err) = download_model(&combo_box.active_text().unwrap()) {
            show_message_popup(&format!("Error: {:?}", err));
        }
    });

    // Connect signals
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    window.connect_key_press_event(|_, event| {
        if let Some(key) = event.keyval().into() {
            if event.state().contains(gdk::ModifierType::CONTROL_MASK) && key == key::q {
                gtk::main_quit();
                Inhibit(true);
            }
        }
        Inhibit(false)
    });

    // Show all the widgets
    window.show_all();

    // Run the main GTK event loop
    gtk::main();
}

fn download_model(model: &str) -> Result<(), Box<dyn std::error::Error>> {
    let model_url: &str = &format!("https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-{}.bin", model.to_lowercase());
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let ai_dir = home_dir.join(".ai");
    let model_path = ai_dir.join(format!("ggml-{}.bin", model.to_lowercase()));

    // If the model file already exists, skip the download
    if Path::new(&model_path).exists() {
        show_message_popup(&format!("Model already exists at {:?}", model_path));
        return Ok(());
    }

    // Create the ~/.ai/ directory if it doesn't exist
    create_dir_all(&ai_dir)?;

    // Download the model
    let mut response = reqwest::blocking::get(model_url)?;
    let mut model_file = File::create(&model_path)?;
    let mut content = Vec::new();
    response.copy_to(&mut content)?;
    model_file.write_all(&content)?;

    println!("Model downloaded and saved to {:?}", model_path);
    Ok(())
}

fn show_message_popup(message: &str) {
    // Initialize GTK
    if let Err(err) = gtk::init() {
        println!("Failed to initialize GTK: {}", err);
        return;
    }

    // Create a new window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Message Popup");
    window.set_default_size(400, 200);

    // Create a message dialog
    let dialog = MessageDialog::new(
        Some(&window),
        DialogFlags::MODAL,
        MessageType::Warning,
        ButtonsType::Close,
        message,
    );

    // Add a response callback
    dialog.connect_response(|dialog, _| {
        dialog.close();
    });

    // Show all components
    dialog.show_all();

    // Start the GTK main loop
    gtk::main();
}
