#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::items_after_statements,
    clippy::cast_sign_loss
)]

use craftingway_data::{
    consumables::build_consumables, icons::build_icons, recipes::build_recipes,
};

fn main() {
    println!("Building recipes.json...");
    build_recipes();

    println!("Building consumables.json...");
    build_consumables();

    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2, "Path for action icons wasn't provided");
    let action_icons_path = std::path::Path::new(&args[1]);
    assert!(action_icons_path.exists(), "Invalid path for action icons");

    println!("Finding icons...");
    build_icons(action_icons_path);
}
