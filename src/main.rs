#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::items_after_statements,
    clippy::cast_sign_loss
)]

use craftingway_data::{
    consumables::build_consumables, icons::build_icons, recipes::build_recipes, translated_items::build_translated_items,
};

fn main() {
    println!("Building recipes.json...");
    let mut item_names = build_recipes();

    println!("Building meals.json and potions.json...");
    let consumable_item_names = build_consumables();
    item_names.extend(consumable_item_names);

    println!("Building item translation files...");
    build_translated_items(item_names);

    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2, "Path for action icons wasn't provided");
    let action_icons_path = std::path::Path::new(&args[1]);
    assert!(action_icons_path.exists(), "Invalid path for action icons");

    println!("Finding icons...");
    build_icons(action_icons_path);
}
