use gtk4 as gtk;
use gtk::prelude::*;
use crate::gui::draft::{build_draft_ui};

pub fn build_home_ui(window: &gtk::ApplicationWindow){
    let grid = gtk::Grid::builder()
        .margin_start(6)
        .margin_end(6)
        .margin_top(6)
        .margin_bottom(6)
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .row_spacing(6)
        .column_spacing(6)
        .build();
    let win_title = gtk::Label::default();
    win_title.set_markup("<big>which mode do you want</big>");
    grid.attach(&win_title, 0, 0, 3, 1);

    let draft = gtk::Button::with_label("draft");
    let cube = gtk::Button::with_label("cube");
    let momir = gtk::Button::with_label("momir");
    let tokens = gtk::Button::with_label("tokens");
    
    let window_clone = window.clone();
    draft.connect_clicked(move |_| {
        build_draft_ui(&window_clone);
    });
    

    grid.attach(&draft, 0, 2, 1, 1);
    grid.attach(&cube, 1, 2, 1, 1);
    grid.attach(&momir, 2, 2, 1, 1);
    grid.attach(&tokens, 2, 2, 1, 1);

    window.set_child(Some(&grid));
}