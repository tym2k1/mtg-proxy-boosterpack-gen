use gtk4 as gtk;
use std::rc::Rc;
use std::cell::RefCell;
use gtk::prelude::*;
use gtk::{ glib};
use crate::bulk::{load_cache};
use crate::generators::generate_boosters;

pub fn build_draft_ui(window: &gtk::ApplicationWindow){
    window.set_child(None::<&gtk::Widget>);
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
    let print = gtk::Button::with_label("print");
    let set_list_field = gtk::EntryCompletion::new();
    let amount = gtk::Entry::new();
    set_list_field.set_text_column(0);
    set_list_field.set_minimum_key_length(1);
    set_list_field.set_popup_completion(true);

    let ls = get_set_list();
    set_list_field.set_model(Some(&ls));
    let input_field = gtk::Entry::new();
    input_field.set_completion(Some(&set_list_field));

    let selected_set: Rc<RefCell<Option<String>>> = Rc::new(RefCell::new(None));
    let selected_set_clone = selected_set.clone();
    set_list_field.connect_match_selected(move |_completion, ls, iter| {
        let value = ls.get::<String>(iter, 0);
        *selected_set_clone.borrow_mut() = Some(value.clone());
        // print safely
        println!("Chosen: {}", value);
        false.into() // GTK4 propagation
    });


    let selected_set_clone = selected_set.clone();
    let text = amount.clone();
    print.connect_clicked(move |_| {
        let chosen = selected_set_clone.borrow().clone().unwrap_or("<none>".to_string());
        let count = text.text().parse::<i32>().unwrap_or(1);

        let fut = async move {
            println!("Printing: {} x{}", chosen, count);
            generate_boosters(&chosen, count).await;
        };
        glib::MainContext::default().spawn_local(fut);
    });
    /*let fut = async{
            generate_boosters("TMNT", 1).await;
        };

    glib::MainContext::default().spawn_local(fut); */

    grid.attach(&print, 8, 2, 1, 1);
    grid.attach(&input_field,0,2,7,1);
    grid.attach(&amount,9,2,1,1);
    window.set_child(Some(&grid));
}

//https://api.scryfall.com/sets


fn get_set_list() -> gtk::ListStore {
    let cache = load_cache();
    let sets = cache.sets;
        
    let store = gtk::ListStore::new(&[glib::Type::STRING]);
    if let Some(vec) = sets{
        for d in vec {
            if &d.set_type == "expansion" || &d.set_type == "funny" || &d.set_type == "core" || &d.set_type == "draft_innovation" || &d.set_type == "masters"{
                store.set(&store.append(), &[(0, &d.name)]);
            }
        }
    }
    store
}