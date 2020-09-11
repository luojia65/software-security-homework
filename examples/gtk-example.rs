// use gio::prelude::*;
// use glib::clone;
// use gtk::prelude::*;

// // When the application is launched…
// fn on_activate(application: &gtk::Application) {
//     // … create a new window …
//     let window = gtk::ApplicationWindow::new(application);
//     // … with a button in it …
//     let button = gtk::Button::with_label("Hello World!");
//     // … which closes the window when clicked
//     button.connect_clicked(clone!(@weak window => move |_| window.close()));
//     window.add(&button);
//     window.show_all();
// }

fn main() {
    // // Create a new application
    // let app = gtk::Application::new(Some("com.github.gtk-rs.examples.basic"), Default::default())
    //     .expect("Initialization failed...");
    // app.connect_activate(|app| on_activate(app));
    // // Run the application
    // app.run(&std::env::args().collect::<Vec<_>>());
}
