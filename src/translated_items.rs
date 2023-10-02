use std::collections::HashMap;

use crate::{models::ItemRecord, utils::write_json_file};

pub fn build_translated_items(english_items: HashMap<u32, ItemRecord>) {
static LANGUAGES: [&str; 3] = ["jpn", "deu", "fra"];
    for language in LANGUAGES {
        let mut translations = HashMap::new();

        let mut item_csv = csv::Reader::from_path(format!("data/{language}/Item.csv")).unwrap();

        let mut non_english_items = HashMap::new();

        for record in item_csv.deserialize::<ItemRecord>() {
            let item = record.unwrap();
            if item.name.trim().is_empty() {
                continue;
            }
            non_english_items.insert(item.id, item);
        }

        for (id, english_item) in &english_items {
            let non_english_item = non_english_items
                .get(id)
                .unwrap_or_else(|| panic!("item not found for {language}: {}", english_item.name));
            translations.insert(
                english_item.name.clone(),
                clean_item_name(non_english_item.name.clone()),
            );
        }

        write_json_file(&translations, format!("output/items_{language}.json").as_str());
    }
}

fn clean_item_name(s: String) -> String {
    s.replace("<SoftHyphen/>", "\u{00AD}")
}
