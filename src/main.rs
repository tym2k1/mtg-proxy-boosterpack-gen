mod model;
mod bulk;
mod pool;
mod booster;
mod pdf;
mod generators;
mod gui;

use generators::*;


#[tokio::main]
async fn main() {
    gui();
    //generate_boosters().await;
}
