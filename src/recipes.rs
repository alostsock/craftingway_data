use serde::Serialize;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use crate::models::{ItemRecord, RecipeLevelRecord, RecipeLookupRecord, RecipeRecord};
use crate::utils::{calculate_hash, read_csv_data, write_json_file};

pub fn build_recipes() -> HashMap<u32, String> {
    let mut relevant_items = HashMap::new();

    let mut items = HashMap::new();
    for item in read_csv_data::<ItemRecord>("data/Item.csv") {
        if item.name.trim().is_empty() {
            continue;
        }
        items.insert(item.id, item);
    }

    let mut recipe_jobs: HashMap<u32, Vec<&str>> = HashMap::new();
    for recipe_lookup in read_csv_data::<RecipeLookupRecord>("data/RecipeLookup.csv") {
        for (recipe_id, job) in &vec![
            (recipe_lookup.crp, "CRP"),
            (recipe_lookup.bsm, "BSM"),
            (recipe_lookup.arm, "ARM"),
            (recipe_lookup.gsm, "GSM"),
            (recipe_lookup.ltw, "LTW"),
            (recipe_lookup.wvr, "WVR"),
            (recipe_lookup.alc, "ALC"),
            (recipe_lookup.cul, "CUL"),
        ] {
            if *recipe_id > 0 {
                recipe_jobs
                    .entry(*recipe_id)
                    .and_modify(|e| e.push(job))
                    .or_insert_with(|| vec![job]);
            }
        }
    }

    let mut recipe_levels = HashMap::new();
    for recipe_level_record in read_csv_data::<RecipeLevelRecord>("data/RecipeLevelTable.csv") {
        recipe_levels.insert(recipe_level_record.recipe_level, recipe_level_record);
    }

    let mut unique_recipes: HashMap<u64, RecipeOutput> = HashMap::new();
    for recipe in read_csv_data::<RecipeRecord>("data/Recipe.csv") {
        if recipe.result_item_id == 0 {
            continue;
        }

        let item = items
            .get(&recipe.result_item_id)
            .unwrap_or_else(|| panic!("no item value for item id {:?}", &recipe.result_item_id));

        relevant_items.insert(item.id, item.name.clone());

        let jobs = recipe_jobs
            .get(&recipe.id)
            .unwrap_or_else(|| panic!("no job value for recipe id {:?}", &recipe.id));

        let recipe_level = recipe_levels
            .get(&recipe.recipe_level)
            .unwrap_or_else(|| panic!("no entry for recipe level {:?}", &recipe.recipe_level));

        fn apply_factor(base_value: u32, factor: u32) -> u32 {
            (f64::from(base_value * factor) / 100.0).floor() as u32
        }

        // Calculate progress/quality/durability requirements with base values from
        // RecipeLevelTable.csv, and factors from Recipe.csv
        let progress = apply_factor(recipe_level.progress, recipe.progress_factor);
        let quality = apply_factor(recipe_level.quality, recipe.quality_factor);
        let durability = apply_factor(recipe_level.durability, recipe.durability_factor);

        let ingredients: Vec<Ingredient> = [
            (recipe.item_0, recipe.amount_0),
            (recipe.item_1, recipe.amount_1),
            (recipe.item_2, recipe.amount_2),
            (recipe.item_3, recipe.amount_3),
            (recipe.item_4, recipe.amount_4),
            (recipe.item_5, recipe.amount_5),
            (recipe.item_6, recipe.amount_6),
            (recipe.item_7, recipe.amount_7),
            (recipe.item_8, recipe.amount_8),
            (recipe.item_9, recipe.amount_9),
        ]
        .iter()
        .filter_map(|(item_id, amount)| {
            if *item_id <= 0 || *amount == 0 {
                return None;
            };

            let item_id = *item_id as u32;
            let item = items.get(&item_id).unwrap();

            relevant_items.insert(item.id, item.name.clone());

            Some(Ingredient {
                name: item.name.clone(),
                amount: *amount,
                item_level: item.item_level,
                can_hq: item.can_hq,
            })
        })
        .collect();

        let mut recipe_output = RecipeOutput {
            name: item.name.clone(),
            jobs: jobs.iter().map(|&job| String::from(job)).collect(),
            job_level: recipe_level.job_level,
            recipe_level: recipe.recipe_level,
            item_level: if item.equip_slot_category > 0 {
                item.item_level
            } else {
                0
            },
            equip_level: if item.equip_slot_category > 0 {
                item.equip_level
            } else {
                0
            },
            stars: recipe_level.stars,
            progress,
            quality,
            durability,
            progress_div: recipe_level.progress_divider,
            progress_mod: recipe_level.progress_modifier,
            quality_div: recipe_level.quality_divider,
            quality_mod: recipe_level.quality_modifier,
            is_specialist: recipe.is_spec,
            is_expert: recipe.is_expert,
            conditions_flag: recipe_level.conditions_flag,
            can_hq: recipe.can_hq,
            material_quality: recipe.material_quality_factor,
            ingredients,
        };

        let key = calculate_hash(&recipe_output);

        unique_recipes
            .entry(key)
            .and_modify(|existing_recipe| {
                existing_recipe.jobs.append(&mut recipe_output.jobs);
            })
            .or_insert(recipe_output);
    }

    write_json_file(
        &unique_recipes.into_values().collect::<Vec<RecipeOutput>>(),
        "output/recipes.json",
    );

    relevant_items
}

#[derive(Debug, Serialize)]
struct RecipeOutput {
    name: String,
    jobs: Vec<String>,
    job_level: u32,
    recipe_level: u32,
    item_level: u32,
    equip_level: u32,
    stars: u32,
    progress: u32,
    quality: u32,
    durability: u32,
    progress_div: u32,
    progress_mod: u32,
    quality_div: u32,
    quality_mod: u32,
    is_specialist: bool,
    is_expert: bool,
    conditions_flag: u32,
    can_hq: bool,
    material_quality: u32,
    ingredients: Vec<Ingredient>,
}

#[derive(Debug, Serialize)]
struct Ingredient {
    name: String,
    amount: u32,
    item_level: u32,
    can_hq: bool,
}

// traits used to dedupe recipes across multiple jobs.
// this is probably a bit overkill, though

impl PartialEq for RecipeOutput {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.job_level == other.job_level
            && self.recipe_level == other.recipe_level
            && self.item_level == other.item_level
            && self.equip_level == other.equip_level
            && self.stars == other.stars
            && self.progress == other.progress
            && self.quality == other.quality
            && self.durability == other.durability
            && self.is_specialist == other.is_specialist
            && self.is_expert == other.is_expert
    }
}

impl Eq for RecipeOutput {}

impl Hash for RecipeOutput {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.job_level.hash(state);
        self.recipe_level.hash(state);
        self.item_level.hash(state);
        self.equip_level.hash(state);
        self.stars.hash(state);
        self.progress.hash(state);
        self.quality.hash(state);
        self.durability.hash(state);
        self.is_specialist.hash(state);
        self.is_expert.hash(state);
    }
}
