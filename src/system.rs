use std::env;
use std::fs;

use crate::{error::VroomError, structure::Vroomfile};

fn get_save_path() -> String {
    if let Ok(vroomfile_home) = env::var("VROOMFILE") {
        vroomfile_home
    } else if let Ok(xdg_data_home) = env::var("XDG_DATA_HOME") {
        format!("{}/vroomfile", xdg_data_home)
    } else {
        "~/.local/share/vroomfile".to_string()
    }
}

pub fn save(to_save: &Vroomfile) -> Result<(), VroomError> {
    // serialize from T
    let json = serde_json::to_string(to_save)?;

    // write to file
    Ok(fs::write(get_save_path(), json)?)
}

pub fn load() -> Result<Vroomfile, VroomError> {
    let file: Option<String> = fs::read_to_string(get_save_path()).ok();

    let json = if let Some(file_contents) = file {
        serde_json::from_str(&file_contents)?
    } else {
        Vroomfile::default()
    };

    Ok(json)
}
