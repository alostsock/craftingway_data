use std::collections::HashMap;

use crate::{
    models::{ActionRecord, CraftActionRecord, ItemRecord},
    utils::{read_csv_data, write_json_file},
};

static LANGUAGES: [&str; 3] = ["jpn", "deu", "fra"];

pub fn build_translated_items(english_items: HashMap<u32, String>) {
    for language in LANGUAGES {
        let mut translations = HashMap::new();

        let mut non_english_items = HashMap::new();

        for item in read_csv_data::<ItemRecord>(format!("data/{language}/Item.csv")) {
            if item.name.trim().is_empty() {
                continue;
            }
            non_english_items.insert(item.id, item);
        }

        for (id, english_item_name) in &english_items {
            let non_english_item = non_english_items
                .get(id)
                .unwrap_or_else(|| panic!("item not found for {language}: {}", english_item_name));
            translations.insert(
                english_item_name.clone(),
                clean_item_name(non_english_item.name.clone()),
            );
        }

        write_json_file(
            &translations,
            format!("output/items_{language}.json").as_str(),
        );
    }
}

fn clean_item_name(s: String) -> String {
    s.replace("<SoftHyphen/>", "\u{00AD}")
}

pub fn build_translated_actions(action_names: Vec<String>) {
    let mut action_ids_by_name = HashMap::new();
    for action in read_csv_data::<ActionRecord>("data/Action.csv") {
        action_ids_by_name.insert(action.name, action.id);
    }

    let mut craft_action_ids_by_name = HashMap::new();
    for craft_action in read_csv_data::<CraftActionRecord>("data/CraftAction.csv") {
        craft_action_ids_by_name.insert(craft_action.name, craft_action.id);
    }

    for language in LANGUAGES {
        let mut translations = HashMap::new();

        let mut action_names_by_id = HashMap::new();
        for action in read_csv_data::<ActionRecord>(format!("data/{language}/Action.csv")) {
            action_names_by_id.insert(action.id, action.name);
        }

        let mut craft_action_names_by_id = HashMap::new();
        for craft_action in
            read_csv_data::<CraftActionRecord>(format!("data/{language}/CraftAction.csv"))
        {
            craft_action_names_by_id.insert(craft_action.id, craft_action.name);
        }

        for action_name in &action_names {
            let action_id = action_ids_by_name
                .get(action_name)
                .or_else(|| craft_action_ids_by_name.get(action_name))
                .unwrap();
            let translated_name = action_names_by_id
                .get(action_id)
                .or_else(|| craft_action_names_by_id.get(action_id))
                .unwrap();
            translations.insert(action_name, translated_name);
        }

        write_json_file(
            &translations,
            format!("output/actions_{language}.json").as_str(),
        );
    }
}
