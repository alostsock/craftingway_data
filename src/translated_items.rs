use std::collections::{BTreeMap, HashMap};

use crate::{
    models::{ActionRecord, CraftActionRecord, ItemRecord},
    utils::{read_csv_data, write_json_file},
};

static LANGUAGES: [&str; 3] = ["jpn", "deu", "fra"];

pub fn build_translated_items(english_items: HashMap<u32, String>) {
    for language in LANGUAGES {
        let mut translations = BTreeMap::new();

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

pub fn build_translated_actions(
    english_actions: HashMap<u32, String>,
    english_craft_actions: HashMap<u32, String>,
) {
    for language in LANGUAGES {
        let mut translations = BTreeMap::new();

        let mut non_english_actions = HashMap::new();
        for action in read_csv_data::<ActionRecord>(format!("data/{language}/Action.csv")) {
            non_english_actions.insert(action.id, action.name);
        }

        let mut non_english_craft_actions = HashMap::new();
        for craft_action in
            read_csv_data::<CraftActionRecord>(format!("data/{language}/CraftAction.csv"))
        {
            non_english_craft_actions.insert(craft_action.id, craft_action.name);
        }

        for (id, english_name) in &english_actions {
            let Some(non_english_name) = non_english_actions.get(id) else {
                continue;
            };
            translations.insert(english_name, non_english_name);
        }

        for (id, english_name) in &english_craft_actions {
            let Some(non_english_name) = non_english_craft_actions.get(id) else {
                continue;
            };
            translations.insert(english_name, non_english_name);
        }

        write_json_file(
            &translations,
            format!("output/actions_{language}.json").as_str(),
        );
    }
}
