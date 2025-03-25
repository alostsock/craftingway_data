use serde::Deserialize;

use crate::utils::bool_string;

#[derive(Debug, Deserialize, Clone)]
pub struct ActionRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Icon")]
    pub icon: u32,

    #[serde(rename = "ActionCategory")]
    pub action_category: i32,

    #[serde(rename = "ClassJob")]
    pub class_job: i32,

    #[serde(rename = "IsPlayerAction")]
    #[serde(deserialize_with = "bool_string")]
    pub is_player_action: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CraftActionRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Icon")]
    pub icon: u32,

    #[serde(rename = "ClassJob")]
    pub class_job: i32,
}

#[derive(Debug, Deserialize)]
pub struct ItemActionRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "Type")]
    pub type_id: u32,

    #[serde(rename = "Data[1]")]
    pub data_1: u32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ItemFoodRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "BaseParam[0]")]
    pub param_0: u32,

    #[serde(rename = "IsRelative[0]")]
    #[serde(deserialize_with = "bool_string")]
    pub param_0_relative: bool,

    #[serde(rename = "Value[0]")]
    pub param_0_value: i32,

    #[serde(rename = "Max[0]")]
    pub param_0_max: u32,

    #[serde(rename = "Value{HQ}[0]")]
    pub param_0_hq_value: i32,

    #[serde(rename = "Max{HQ}[0]")]
    pub param_0_hq_max: u32,

    #[serde(rename = "BaseParam[1]")]
    pub param_1: u32,

    #[serde(rename = "IsRelative[1]")]
    #[serde(deserialize_with = "bool_string")]
    pub param_1_relative: bool,

    #[serde(rename = "Value[1]")]
    pub param_1_value: i32,

    #[serde(rename = "Max[1]")]
    pub param_1_max: u32,

    #[serde(rename = "Value{HQ}[1]")]
    pub param_1_hq_value: i32,

    #[serde(rename = "Max{HQ}[1]")]
    pub param_1_hq_max: u32,

    #[serde(rename = "BaseParam[2]")]
    pub param_2: u32,

    #[serde(rename = "IsRelative[2]")]
    #[serde(deserialize_with = "bool_string")]
    pub param_2_relative: bool,

    #[serde(rename = "Value[2]")]
    pub param_2_value: i32,

    #[serde(rename = "Max[2]")]
    pub param_2_max: u32,

    #[serde(rename = "Value{HQ}[2]")]
    pub param_2_hq_value: i32,

    #[serde(rename = "Max{HQ}[2]")]
    pub param_2_hq_max: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ItemRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Level{Item}")]
    pub item_level: u32,

    #[serde(rename = "Level{Equip}")]
    pub equip_level: u32,

    #[serde(rename = "ItemAction")]
    pub item_action: u32,

    #[serde(rename = "CanBeHq")]
    #[serde(deserialize_with = "bool_string")]
    pub can_hq: bool,

    #[serde(rename = "EquipSlotCategory")]
    pub equip_slot_category: u32,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct RecipeRecord {
    #[serde(rename = "#")]
    pub id: u32,

    #[serde(rename = "RecipeLevelTable")]
    pub recipe_level: u32,

    #[serde(rename = "Item{Result}")]
    pub result_item_id: u32,

    #[serde(rename = "DifficultyFactor")]
    pub progress_factor: u32,

    #[serde(rename = "QualityFactor")]
    pub quality_factor: u32,

    #[serde(rename = "DurabilityFactor")]
    pub durability_factor: u32,

    #[serde(rename = "RequiredCraftsmanship")]
    pub required_craftsmanship: u32,

    #[serde(rename = "RequiredControl")]
    pub required_control: u32,

    #[serde(rename = "CanHq")]
    #[serde(deserialize_with = "bool_string")]
    pub can_hq: bool,

    #[serde(rename = "IsSpecializationRequired")]
    #[serde(deserialize_with = "bool_string")]
    pub is_spec: bool,

    #[serde(rename = "IsExpert")]
    #[serde(deserialize_with = "bool_string")]
    pub is_expert: bool,

    #[serde(rename = "MaterialQualityFactor")]
    pub material_quality_factor: u32,

    #[serde(rename = "Item{Ingredient}[0]")]
    pub item_0: i32,
    #[serde(rename = "Amount{Ingredient}[0]")]
    pub amount_0: u32,

    #[serde(rename = "Item{Ingredient}[1]")]
    pub item_1: i32,
    #[serde(rename = "Amount{Ingredient}[1]")]
    pub amount_1: u32,

    #[serde(rename = "Item{Ingredient}[2]")]
    pub item_2: i32,
    #[serde(rename = "Amount{Ingredient}[2]")]
    pub amount_2: u32,

    #[serde(rename = "Item{Ingredient}[3]")]
    pub item_3: i32,
    #[serde(rename = "Amount{Ingredient}[3]")]
    pub amount_3: u32,

    #[serde(rename = "Item{Ingredient}[4]")]
    pub item_4: i32,
    #[serde(rename = "Amount{Ingredient}[4]")]
    pub amount_4: u32,

    #[serde(rename = "Item{Ingredient}[5]")]
    pub item_5: i32,
    #[serde(rename = "Amount{Ingredient}[5]")]
    pub amount_5: u32,

    #[serde(rename = "Item{Ingredient}[6]")]
    pub item_6: i32,
    #[serde(rename = "Amount{Ingredient}[6]")]
    pub amount_6: u32,

    #[serde(rename = "Item{Ingredient}[7]")]
    pub item_7: i32,
    #[serde(rename = "Amount{Ingredient}[7]")]
    pub amount_7: u32,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Hash)]
pub struct RecipeLevelRecord {
    #[serde(rename = "#")]
    pub recipe_level: u32,

    #[serde(rename = "ClassJobLevel")]
    pub job_level: u32,

    #[serde(rename = "Stars")]
    pub stars: u32,

    #[serde(rename = "Durability")]
    pub durability: u32,

    #[serde(rename = "Difficulty")]
    pub progress: u32,

    #[serde(rename = "Quality")]
    pub quality: u32,

    #[serde(rename = "ProgressDivider")]
    pub progress_divider: u32,

    #[serde(rename = "QualityDivider")]
    pub quality_divider: u32,

    #[serde(rename = "ProgressModifier")]
    pub progress_modifier: u32,

    #[serde(rename = "QualityModifier")]
    pub quality_modifier: u32,

    #[serde(rename = "ConditionsFlag")]
    pub conditions_flag: u32,
}

#[derive(Debug, Deserialize)]
pub struct RecipeLookupRecord {
    #[serde(rename = "CRP")]
    pub crp: u32,

    #[serde(rename = "BSM")]
    pub bsm: u32,

    #[serde(rename = "ARM")]
    pub arm: u32,

    #[serde(rename = "GSM")]
    pub gsm: u32,

    #[serde(rename = "LTW")]
    pub ltw: u32,

    #[serde(rename = "WVR")]
    pub wvr: u32,

    #[serde(rename = "ALC")]
    pub alc: u32,

    #[serde(rename = "CUL")]
    pub cul: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StatusRecord {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Icon")]
    pub icon: u32,

    #[serde(rename = "MaxStacks")]
    pub max_stacks: u32,

    #[serde(rename = "ClassJobCategory")]
    pub category: u32,
}
