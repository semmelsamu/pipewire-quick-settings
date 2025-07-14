use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};

fn app() -> glib::ExitCode {
    let application = Application::builder()
        .application_id("com.github.semmelsamu.pipewire-quick-settings")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Pipewire Quick Settings")
            .default_width(350)
            .default_height(70)
            .build();

        let button = Button::with_label("Click me!");
        button.connect_clicked(|_| {
            eprintln!("Clicked!");
        });
        window.set_child(Some(&button));

        window.present();
    });

    application.run()
}