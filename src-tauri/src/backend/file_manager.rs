use std::fs;
use std::path::Path;

pub fn move_file(from: &Path, to: &Path) -> Result<(), String> {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    if to.exists() {
        return Err("Target file already exists".into());
    }

    match fs::rename(from, to) {
        Ok(_) => Ok(()),
        Err(_) => {
            fs::copy(from, to).map_err(|e| e.to_string())?;
            fs::remove_file(from).map_err(|e| e.to_string())?;
            Ok(())
        }
    }
}