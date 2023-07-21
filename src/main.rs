extern crate gtk;

use gio::glib::num_processors;
use gtk::prelude::*;
use gdk::{keys::constants as key};
use gtk::{Button, TextView, Window, WindowType, HeaderBar, Adjustment, Popover, ComboBoxText, TextBuffer, TextTagTable};
use mutter::{Model, ModelType};
use rfd::FileDialog;
use std::path::Path;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;
use std::fs::create_dir_all;

fn main() {
    // Initialize GTK
    gtk::init().expect("Failed to initialize GTK.");

    // Create the main window
    let window = Window::new(WindowType::Toplevel);
    window.set_title("Transcription App");
    window.set_default_size(500, 300);

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
    combo_box.set_active(Some(0));
    vbox.pack_start(&combo_box, false, false, 5);

    // Create the "Download Model" button
    let download_button = Button::with_label("Download Model");
    vbox.pack_start(&download_button, false, false, 0);

    popover.add(&vbox);
    menu_button.connect_clicked(move |_| {
        popover.show_all();
    });

    // Create the text view and its buffer
    let text_view = TextView::new();
    let mut buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    text_view.set_buffer(Some(&buffer));
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_border_width(10);
    text_view.set_editable(false);
    text_view.set_monospace(true);

    // Add the text view to the main window
    let scrolled_window = gtk::ScrolledWindow::new(None::<&Adjustment>, None::<&Adjustment>);
    scrolled_window.add(&text_view);
    window.add(&scrolled_window);

    open_button.connect_clicked(move |_| {
        let file = format!("{}", FileDialog::new()
            .set_directory("~/")
            .add_filter("Audio", &["mp3", "wav", "flac", "ogg"])
            .pick_file()
            .unwrap()
            .display());

        let home_dir = dirs::home_dir().unwrap();
        let ai_dir = home_dir.join(".ai");
        let model_path = ai_dir.join("ggml-tiny.en.bin");

        let model = Model::new(model_path.to_str().unwrap()).unwrap();

        let file_stream = std::fs::read(file).unwrap();
        let transcription = model
            .transcribe_audio(file_stream, false, false, None)
            .unwrap();
        println!("{}", transcription.as_text());
        buffer.set_text(&format!("{}", transcription.as_text()));
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
        let fakebuffer = text_view.buffer().unwrap();
        file.write_all(fakebuffer.text(&fakebuffer.start_iter(), &fakebuffer.end_iter(), false).unwrap().as_bytes()).unwrap();
    });

    download_button.connect_clicked(move |_| {
        headerbar.set_title(Some("Please wait..."));
        if let Err(err) = download_model() {
            eprintln!("Error: {:?}", err);
        }
        headerbar.set_title(Some("Grimace"));
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

fn download_model() -> Result<(), Box<dyn std::error::Error>> {
    const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-tiny.bin";
    let home_dir = dirs::home_dir().ok_or("Failed to get home directory")?;
    let ai_dir = home_dir.join(".ai");
    let model_path = ai_dir.join("ggml-tiny.en.bin");

    // If the model file already exists, skip the download
    if Path::new(&model_path).exists() {
        println!("Model already exists at {:?}", model_path);
        return Ok(());
    }

    // Create the ~/.ai/ directory if it doesn't exist
    create_dir_all(&ai_dir)?;

    // Download the model
    let mut response = reqwest::blocking::get(MODEL_URL)?;
    let mut model_file = File::create(&model_path)?;
    let mut content = Vec::new();
    response.copy_to(&mut content)?;
    model_file.write_all(&content)?;

    println!("Model downloaded and saved to {:?}", model_path);
    Ok(())
}