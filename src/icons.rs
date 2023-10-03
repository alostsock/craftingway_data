use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use crate::models::{ActionRecord, CraftActionRecord, StatusRecord};
use crate::utils::{read_csv_data, write_json_file};

struct IconData {
    pub name: String,
    pub job: Option<String>,
}

pub fn build_icons(action_icons_path: &Path) -> (HashMap<u32, String>, HashMap<u32, String>) {
    let icons_dir = Path::new("output/icon");
    if icons_dir.exists() {
        fs::remove_dir_all("output/icon").unwrap();
    }
    fs::create_dir_all("output/icon/action").unwrap();
    fs::create_dir_all("output/icon/status").unwrap();

    let mut relevant_actions = HashMap::new();
    let mut relevant_craft_actions = HashMap::new();

    // read in action icons
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

    for action in read_csv_data::<ActionRecord>("data/Action.csv") {
        if action.action_category != 7 || action.class_job <= 0 || !action.is_player_action {
            continue;
        }
        relevant_actions.insert(action.id, action.name.clone());
        record_icon(action.icon, action.name, job_string(action.class_job));
    }

    for craft_action in read_csv_data::<CraftActionRecord>("data/CraftAction.csv") {
        if craft_action.class_job <= 0 {
            continue;
        }
        relevant_craft_actions.insert(craft_action.id, craft_action.name.clone());
        record_icon(
            craft_action.icon,
            craft_action.name,
            job_string(craft_action.class_job),
        );
    }

    // read in status icons
    let mut statuses_by_id: HashMap<u32, String> = HashMap::new();

    for status in read_csv_data::<StatusRecord>("data/Status.csv") {
        if status.category != 33 {
            continue;
        }

        if status.max_stacks == 0 {
            statuses_by_id.insert(status.icon, status.name);
        } else {
            for stacks in 0..status.max_stacks {
                let icon_id = status.icon + stacks;
                let name = format!("{}-{}", status.name, stacks + 1);
                statuses_by_id.insert(icon_id, name);
            }
        }
    }

    let mut action_output: Vec<String> = vec![];
    let mut status_output: Vec<String> = vec![];

    // iterate through icon files and match them up with action data from above
    let mut min_icon_id: u32 = 999_999;
    let mut max_icon_id: u32 = 0;
    for entry in WalkDir::new(action_icons_path) {
        let entry = entry.unwrap();

        if !entry.metadata().unwrap().is_file() {
            continue;
        }

        // 000000.png
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

        // copy action icons
        if let Some(icon_data) = icons_by_id.get(&icon_id) {
            let ext = entry.path().extension().unwrap().to_string_lossy();
            // some icons are class-specific, others aren't
            let action_name = if let Some(job) = &icon_data.job {
                format!("{}-{}", icon_data.name, job)
            } else {
                icon_data.name.clone()
            };
            let filename = format!("{action_name}.{ext}");

            fs::copy(entry.path(), format!("output/icon/action/{filename}"))
                .unwrap_or_else(|_| panic!("error copying {:?}", entry.path()));

            action_output.push(action_name);
            min_icon_id = min_icon_id.min(icon_id);
            max_icon_id = max_icon_id.max(icon_id);
        }

        // copy status icons
        if let Some(status_name) = statuses_by_id.get(&icon_id) {
            let ext = entry.path().extension().unwrap().to_string_lossy();
            let filename = format!("{status_name}.{ext}");

            fs::copy(entry.path(), format!("output/icon/status/{filename}"))
                .unwrap_or_else(|_| panic!("error copying {:?}", entry.path()));

            status_output.push(String::from(status_name));
            min_icon_id = min_icon_id.min(icon_id);
            max_icon_id = max_icon_id.max(icon_id);
        }
    }

    println!(
        "Found {} icons in id range {} to {}",
        action_output.len() + status_output.len(),
        min_icon_id,
        max_icon_id
    );

    write_json_file(&action_output, "output/actions.json");
    write_json_file(&status_output, "output/statuses.json");

    (relevant_actions, relevant_craft_actions)
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
