mod model;
mod bulk;
mod pool;
mod booster;
mod pdf;
mod generators;
mod gui;

use crate::bulk::{fetch_sets};

use crate::gui::gui::gui_build;


#[tokio::main]
async fn main() {
    print!("hello world");
    fetch_sets(false).await;
    gui_build();
    //generate_boosters().await;
}
