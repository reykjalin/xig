extern crate gtk;
extern crate gio;

use gio::prelude::*;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    // Configure window
    window.set_default_size(800, 600);
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);

    window.connect_delete_event(move |win, _| {
        win.destroy();
        Inhibit(false)
    });

    // Configure header bar
    let header_bar = gtk::HeaderBar::new();
    header_bar.set_title("XiG");
    header_bar.set_show_close_button(true);
    window.set_titlebar(&header_bar);

    // New file button
    let new_button = gtk::Button::new_from_icon_name("document-new-symbolic", gtk::IconSize::Button.into());
    new_button.set_tooltip_text("New File");
    new_button.set_action_name("win.new-tab");
    header_bar.pack_start(&new_button);

    // Open file button
    let open_button = gtk::Button::new_from_icon_name("document-open-symbolic", gtk::IconSize::Button.into());
    new_button.set_tooltip_text("Open File");
    new_button.set_action_name("win.open");
    header_bar.pack_start(&open_button);

    // Save button
    let save_button = gtk::Button::new_from_icon_name("document-save-symbolic", gtk::IconSize::Button.into());
    new_button.set_tooltip_text("Save File");
    new_button.set_action_name("win.save");
    header_bar.pack_end(&save_button);

    // Show window
    window.show_all();

    // Button methods
    // let new_tab_action = gio::SimpleAction::new("win.new-tab", None);
    // new_tab_action.connect_activate(move |_, _| {
    //     println!("new file!");
    // });

    window.show_all();
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let application = gtk::Application::new("com.github.basic",
                                            gio::ApplicationFlags::empty())
                                        .expect("Initialization failed...");
    
    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&std::env::args().collect::<Vec<_>>());
}
