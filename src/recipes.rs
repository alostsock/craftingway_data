use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::utils::{bool_string, write_json_file};

pub fn build_recipes() {
    let mut item_csv = csv::Reader::from_path("data/Item.csv").unwrap();
    let mut recipe_lookup_csv = csv::Reader::from_path("data/RecipeLookup.csv").unwrap();
    let mut recipe_level_csv = csv::Reader::from_path("data/RecipeLevelTable.csv").unwrap();
    let mut recipe_csv = csv::Reader::from_path("data/Recipe.csv").unwrap();

    let mut items = HashMap::new();
    for record in item_csv.deserialize::<ItemRecord>() {
        let item = record.unwrap();
        if item.name.trim().is_empty() {
            continue;
        }
        items.insert(item.id, item);
    }

    let mut recipe_jobs: HashMap<u32, Vec<&str>> = HashMap::new();
    for record in recipe_lookup_csv.deserialize::<RecipeLookupRecord>() {
        let recipe_lookup = record.unwrap();

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
    for record in recipe_level_csv.deserialize::<RecipeLevelRecord>() {
        let recipe_level_record = record.unwrap();
        recipe_levels.insert(recipe_level_record.recipe_level, recipe_level_record);
    }

    let mut recipe_output: Vec<RecipeOutput> = vec![];
    for record in recipe_csv.deserialize::<RecipeRecord>() {
        let recipe = record.unwrap();

        if recipe.result_item_id == 0 {
            continue;
        }

        let item = items
            .get(&recipe.result_item_id)
            .unwrap_or_else(|| panic!("no item value for item id {:?}", &recipe.result_item_id));

        let jobs = recipe_jobs
            .get(&recipe.id)
            .unwrap_or_else(|| panic!("no job value for recipe id {:?}", &recipe.id));

        let recipe_level = recipe_levels
            .get(&recipe.recipe_level)
            .unwrap_or_else(|| panic!("no entry for recipe level {:?}", &recipe.recipe_level));

        /// Calculate progress/quality/durability requirements with base values from
        /// RecipeLevelTable.csv, and factors from Recipe.csv
        fn calculate_requirement(base_value: u32, factor: u32) -> u32 {
            (f64::from(base_value * factor) / 100.0).floor() as u32
        }

        let progress = calculate_requirement(recipe_level.progress, recipe.progress_factor);
        let quality = calculate_requirement(recipe_level.quality, recipe.quality_factor);
        let durability = calculate_requirement(recipe_level.durability, recipe.durability_factor);

        recipe_output.push(RecipeOutput {
            name: item.name.clone(),
            jobs: jobs.iter().map(|&job| String::from(job)).collect(),
            job_level: recipe_level.job_level,
            recipe_level: recipe.recipe_level,
            item_level: item.item_level,
            equip_level: item.equip_level,
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
        });
    }

    write_json_file(&recipe_output, "output/recipes.json");
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct RecipeRecord {
    #[serde(rename = "#")]
    id: u32,

    #[serde(rename = "RecipeLevelTable")]
    recipe_level: u32,

    #[serde(rename = "Item{Result}")]
    result_item_id: u32,

    #[serde(rename = "DifficultyFactor")]
    progress_factor: u32,

    #[serde(rename = "QualityFactor")]
    quality_factor: u32,

    #[serde(rename = "DurabilityFactor")]
    durability_factor: u32,

    #[serde(rename = "RequiredCraftsmanship")]
    required_craftsmanship: u32,

    #[serde(rename = "RequiredControl")]
    required_control: u32,

    #[serde(rename = "CanHq")]
    #[serde(deserialize_with = "bool_string")]
    can_hq: bool,

    #[serde(rename = "IsSpecializationRequired")]
    #[serde(deserialize_with = "bool_string")]
    is_spec: bool,

    #[serde(rename = "IsExpert")]
    #[serde(deserialize_with = "bool_string")]
    is_expert: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
struct RecipeLevelRecord {
    #[serde(rename = "#")]
    recipe_level: u32,

    #[serde(rename = "ClassJobLevel")]
    job_level: u32,

    #[serde(rename = "Stars")]
    stars: u32,

    #[serde(rename = "Durability")]
    durability: u32,

    #[serde(rename = "Difficulty")]
    progress: u32,

    #[serde(rename = "Quality")]
    quality: u32,

    #[serde(rename = "ProgressDivider")]
    progress_divider: u32,

    #[serde(rename = "QualityDivider")]
    quality_divider: u32,

    #[serde(rename = "ProgressModifier")]
    progress_modifier: u32,

    #[serde(rename = "QualityModifier")]
    quality_modifier: u32,

    #[serde(rename = "ConditionsFlag")]
    conditions_flag: u32,
}

#[derive(Debug, Deserialize)]
struct ItemRecord {
    #[serde(rename = "#")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Level{Item}")]
    item_level: u32,

    #[serde(rename = "Level{Equip}")]
    equip_level: u32,
}

#[derive(Debug, Deserialize)]
struct RecipeLookupRecord {
    #[serde(rename = "CRP")]
    crp: u32,

    #[serde(rename = "BSM")]
    bsm: u32,

    #[serde(rename = "ARM")]
    arm: u32,

    #[serde(rename = "GSM")]
    gsm: u32,

    #[serde(rename = "LTW")]
    ltw: u32,

    #[serde(rename = "WVR")]
    wvr: u32,

    #[serde(rename = "ALC")]
    alc: u32,

    #[serde(rename = "CUL")]
    cul: u32,
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
}
