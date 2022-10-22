#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::items_after_statements,
    clippy::cast_sign_loss
)]

use craftingway_data::recipes::build_recipes_json;

fn main() {
    build_recipes_json();
}
