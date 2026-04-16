use crate::{backend::db::*, domain::models::{FileMoveDisplay, NodeMetadata}};

#[tauri::command]
pub fn save_folder_data(
    app: tauri::AppHandle,
    path: String,
    description: String,
) -> Result<(), String> {
    upsert_metadata(&app, &path, &description)?;
    Ok(())
}

#[tauri::command]
pub fn get_folder_data(
    app: tauri::AppHandle,
    path: String,
) -> Result<Option<NodeMetadata>, String> {
    get_metadata(&app, &path)
}

#[tauri::command]
pub fn get_description(
    app: tauri::AppHandle,
    path: String,
) -> Result<Option<String>, String> {
    let metadata = get_metadata(&app, &path)?;

    Ok(metadata.map(|m| m.description))
}

#[tauri::command]
pub fn get_all_data(
    app: tauri::AppHandle,
) -> Result<Vec<NodeMetadata>, String> {
    get_all_metadata(&app)
}

#[tauri::command]
pub fn delete_folder_data(
    app: tauri::AppHandle,
    path: String,
) -> Result<(), String> {
    remove_metadata(&app, &path)?;
    Ok(())
}

#[tauri::command]
pub fn get_recent_moves(app: tauri::AppHandle) -> Result<Vec<FileMoveDisplay>, String> {
    let mut conn = crate::backend::db::establish_connection(&app)?;
    
    crate::backend::db::get_recent_file_moves(&mut conn, 20)
}