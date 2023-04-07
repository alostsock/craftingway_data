use serde::Serialize;
use std::collections::HashMap;

use crate::models::{ItemActionRecord, ItemFoodRecord, ItemRecord};
use crate::utils::write_json_file;

// https://github.com/xivapi/ffxiv-datamining/blob/35e435494317723be856f18fb3b48f526316656e/docs/ItemActions.md#845
const ITEM_ACTION_BATTLE_FOOD_TYPE_ID: u32 = 844;
const ITEM_ACTION_DOH_FOOD_TYPE_ID: u32 = 845;
const ITEM_ACTION_DOH_POTION_TYPE_ID: u32 = 846;
const VALID_ITEM_ACTION_TYPE_IDS: &[u32] = &[
    ITEM_ACTION_BATTLE_FOOD_TYPE_ID,
    ITEM_ACTION_DOH_FOOD_TYPE_ID,
    ITEM_ACTION_DOH_POTION_TYPE_ID,
];

// https://github.com/xivapi/ffxiv-datamining/blob/35e435494317723be856f18fb3b48f526316656e/csv/BaseParam.csv
const CRAFTSMANSHIP_PARAM_ID: u32 = 70;
const CONTROL_PARAM_ID: u32 = 71;
const CP_PARAM_ID: u32 = 11;
const VALID_PARAMS: &[u32] = &[CRAFTSMANSHIP_PARAM_ID, CONTROL_PARAM_ID, CP_PARAM_ID];

pub fn build_consumables() {
    let mut item_csv = csv::Reader::from_path("data/Item.csv").unwrap();
    let mut item_action_csv = csv::Reader::from_path("data/ItemAction.csv").unwrap();
    let mut item_food_csv = csv::Reader::from_path("data/ItemFood.csv").unwrap();

    let mut item_food_by_id = HashMap::new();
    for record in item_food_csv.deserialize::<ItemFoodRecord>() {
        let item_food = record.unwrap();
        item_food_by_id.insert(item_food.id, item_food);
    }

    let mut consumable_by_item_action_id = HashMap::new();
    for record in item_action_csv.deserialize::<ItemActionRecord>() {
        let item_action = record.unwrap();

        if !VALID_ITEM_ACTION_TYPE_IDS.contains(&item_action.type_id) {
            continue;
        }

        let is_potion = item_action.type_id == ITEM_ACTION_DOH_POTION_TYPE_ID;
        let item_food = *item_food_by_id.get(&item_action.data_1).unwrap();

        let params: &[u32] = &[item_food.param_0, item_food.param_1, item_food.param_2];
        if params.iter().any(|param| VALID_PARAMS.contains(param)) {
            consumable_by_item_action_id.insert(
                item_action.id,
                Consumable {
                    is_potion,
                    item_food,
                },
            );
        }
    }

    let mut consumables = vec![];
    for record in item_csv.deserialize::<ItemRecord>() {
        let item = record.unwrap();

        if let Some(consumable) = consumable_by_item_action_id.get(&item.item_action) {
            let Consumable {
                is_potion,
                item_food,
            } = *consumable;

            let (craftsmanship, control, cp) = get_stats(item_food);

            consumables.push(ConsumableOutput {
                item_level: item.item_level,
                name: item.name,
                is_potion,
                craftsmanship,
                control,
                cp,
            });
        }
    }

    write_json_file(&consumables, "output/consumables.json");
}

#[allow(clippy::type_complexity)]
#[rustfmt::skip]
fn get_stats(item_food: ItemFoodRecord) -> (Option<Vec<u32>>, Option<Vec<u32>>, Option<Vec<u32>>) {
    let ItemFoodRecord {
        id: _,
        param_0, param_0_relative, param_0_value, param_0_max, param_0_hq_value, param_0_hq_max,
        param_1, param_1_relative, param_1_value, param_1_max, param_1_hq_value, param_1_hq_max,
        param_2, param_2_relative, param_2_value, param_2_max, param_2_hq_value, param_2_hq_max,
    } = item_food;

    let mut craftsmanship = None;
    let mut control = None;
    let mut cp = None;

    for (param, is_relative, value, max, hq_value, hq_max) in [
        (param_0, param_0_relative, param_0_value, param_0_max, param_0_hq_value, param_0_hq_max),
        (param_1, param_1_relative, param_1_value, param_1_max, param_1_hq_value, param_1_hq_max),
        (param_2, param_2_relative, param_2_value, param_2_max, param_2_hq_value, param_2_hq_max),
    ] {
        let values = vec![value as u32, max, hq_value as u32, hq_max];

        if param == CRAFTSMANSHIP_PARAM_ID {
            assert!(is_relative);
            assert!(values.iter().all(|&v| v > 0));
            craftsmanship = Some(values);
        } else if param == CONTROL_PARAM_ID {
            assert!(is_relative);
            assert!(values.iter().all(|&v| v > 0));
            control = Some(values);
        } else if param == CP_PARAM_ID {
            assert!(is_relative);
            assert!(values.iter().all(|&v| v > 0));
            cp = Some(values);
        }
    }

    (craftsmanship, control, cp)
}

#[derive(Debug)]
struct Consumable {
    is_potion: bool,
    item_food: ItemFoodRecord,
}

#[derive(Debug, Serialize)]
struct ConsumableOutput {
    item_level: u32,
    name: String,
    is_potion: bool,
    craftsmanship: Option<Vec<u32>>,
    control: Option<Vec<u32>>,
    cp: Option<Vec<u32>>,
}
