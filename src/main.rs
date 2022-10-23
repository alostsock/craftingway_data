#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::items_after_statements,
    clippy::cast_sign_loss
)]

use craftingway_data::{actions::build_actions_json, recipes::build_recipes_json};

fn main() {
    println!("Building recipes.json...");
    build_recipes_json();

    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() == 2, "Path for action icons wasn't provided");
    let action_icons_path = std::path::Path::new(&args[1]);
    assert!(action_icons_path.exists(), "Invalid path for action icons");

    println!("Finding icons and building actions.json...");
    build_actions_json(action_icons_path);
}
