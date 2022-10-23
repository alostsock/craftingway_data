use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::utils::{bool_string, write_json_file};

pub fn build_actions_json(action_icons_path: &Path) {
    let mut action_csv = csv::Reader::from_path("data/Action.csv").unwrap();
    let mut craft_action_csv = csv::Reader::from_path("data/CraftAction.csv").unwrap();

    let mut icons_by_id: HashMap<u32, IconData> = HashMap::new();

    let mut record_icon = |icon_id: u32, name: String, job: Option<String>| {
        icons_by_id
            .entry(icon_id)
            .and_modify(|icon_data| {
                // if an entry already exists, the icon isn't specific to one job
                if icon_data.job.is_some() && icon_data.job != job {
                    icon_data.job = None
                }
            })
            .or_insert(IconData { name, job });
    };

    for record in action_csv.deserialize::<ActionRecord>() {
        let action = record.unwrap();

        if action.action_category != 7 || action.class_job <= 0 || !action.is_player_action {
            continue;
        }

        record_icon(action.icon, action.name, job_string(action.class_job));
    }

    for record in craft_action_csv.deserialize::<CraftActionRecord>() {
        let craft_action = record.unwrap();

        if craft_action.class_job <= 0 {
            continue;
        }

        record_icon(
            craft_action.icon,
            craft_action.name,
            job_string(craft_action.class_job),
        );
    }

    let icons_dir = Path::new("output/icons");
    if icons_dir.exists() {
        fs::remove_dir_all("output/icons").unwrap();
    }
    fs::create_dir("output/icons").unwrap();

    let mut output_data: HashMap<String, Vec<String>> = HashMap::new();

    // iterate through icon files and match them up with action data from above
    for entry in WalkDir::new(action_icons_path) {
        let entry = entry.unwrap();

        if !entry.metadata().unwrap().is_file() {
            continue;
        }

        let filename = entry.file_name().to_string_lossy();
        let icon_id = if let Ok(icon_id) = filename.split('.').next().unwrap().parse::<u32>() {
            icon_id
        } else {
            println!(
                "Unable to parse icon id from file {}",
                entry.path().to_string_lossy()
            );
            continue;
        };

        if let Some(icon_data) = icons_by_id.get(&icon_id) {
            let ext = entry.path().extension().unwrap().to_string_lossy();
            let output_filename = if let Some(job) = &icon_data.job {
                format!("{}-{}.{}", icon_data.name, job, ext)
            } else {
                format!("{}.{}", icon_data.name, ext)
            };

            fs::copy(entry.path(), format!("output/icons/{}", output_filename))
                .unwrap_or_else(|_| panic!("error copying {:?}", entry.path()));

            output_data
                .entry(icon_data.name.clone())
                .and_modify(|filenames| {
                    filenames.push(output_filename.clone());
                })
                .or_insert_with(|| vec![output_filename.clone()]);
        }
    }

    write_json_file(&output_data, "output/actions.json");
}

fn job_string(class_job: i32) -> Option<String> {
    match class_job {
        8 => Some("CRP"),
        9 => Some("BSM"),
        10 => Some("ARM"),
        11 => Some("GSM"),
        12 => Some("LTW"),
        13 => Some("WVR"),
        14 => Some("ALC"),
        15 => Some("CUL"),
        _ => None,
    }
    .map(String::from)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ActionRecord {
    #[serde(rename = "#")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Icon")]
    icon: u32,

    #[serde(rename = "ActionCategory")]
    action_category: i32,

    #[serde(rename = "ClassJob")]
    class_job: i32,

    #[serde(rename = "IsPlayerAction")]
    #[serde(deserialize_with = "bool_string")]
    is_player_action: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct CraftActionRecord {
    #[serde(rename = "#")]
    id: u32,

    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Icon")]
    icon: u32,

    #[serde(rename = "ClassJob")]
    class_job: i32,
}

struct IconData {
    name: String,
    job: Option<String>,
}
