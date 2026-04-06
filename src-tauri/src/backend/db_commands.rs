use crate::backend::db::*;

pub fn save_folder_data(path: String, description: String) -> Result<(), String> {
    match upsert_metadata(&path, &description) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_folder_data(path: String) -> Result<(), String> {
    match get_metadata(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_all_data() -> Result<(), String> {
    match get_all_metadata() {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn delete_folder_data(path: String) -> Result<(), String> {
    match remove_metadata(&path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}