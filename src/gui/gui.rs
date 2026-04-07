use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{gio, glib};
use crate::gui::home::{build_home_ui};


pub fn gui_build() -> glib::ExitCode{
    let application = gtk::Application::builder()
        .application_id("com.github.gtk-rs.examples.menubar")
        .build();
    

    // When activated, shuts down the application
    let quit = gio::SimpleAction::new("quit", None);
    quit.connect_activate(glib::clone!(
        #[weak]
        application,
        move |_action, _parameter| {
            application.quit();
        }
    ));
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.add_action(&quit);
    application.connect_activate(build_main_ui);

    // Run the application
    application.run()
}



fn build_main_ui(application: &gtk::Application) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Mtg Printing tool")
        .default_width(600)
        .default_height(480)
        .show_menubar(true)
        .build();

    let about = gio::ActionEntry::builder("about")
        .activate(|_, _, _| println!("About was pressed"))
        .build();

    let quit = gio::ActionEntry::builder("quit")
        .activate(|app: &gtk::Application, _, _| app.quit())
        .build();
    let window_clone = window.clone();
    let home = gio::ActionEntry::builder("home")
        .activate(move|_, _, _| build_home_ui(&window_clone))
        .build();

    application.add_action_entries([about, quit,home]);

    let menubar = {
        let settings_menu = {
            let home_menu_item = gio::MenuItem::new(Some("home"), Some("app.home"));
            let options_menu_item = gio::MenuItem::new(Some("Options"), Some("app.about"));
            let about_menu_item = gio::MenuItem::new(Some("About"), Some("app.about"));
            let quit_menu_item = gio::MenuItem::new(Some("Quit"), Some("app.quit"));

            let settings_menu = gio::Menu::new();
            settings_menu.append_item(&home_menu_item);
            settings_menu.append_item(&options_menu_item);
            settings_menu.append_item(&about_menu_item);
            settings_menu.append_item(&quit_menu_item);
            settings_menu
        };

        let menubar = gio::Menu::new();
        menubar.append_submenu(Some("⚙"), &settings_menu);

        menubar
    };

    application.set_menubar(Some(&menubar));

    // Create a title label
    
    build_home_ui(&window);
    window.present();
    
}
