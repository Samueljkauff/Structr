use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use chrono::Utc;
use crate::schema::folder_metadata::dsl::*;
use crate::domain::models::*;
use tauri::Manager;
use std::fs;
use diesel::result::Error;

pub fn establish_connection(app: &tauri::AppHandle) -> Result<SqliteConnection, String> {
    let mut db_path = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    fs::create_dir_all(&db_path).map_err(|e| e.to_string())?;

    db_path.push("metadata.db");

    println!("DB PATH: {:?}", db_path);

    SqliteConnection::establish(db_path.to_str().unwrap())
        .map_err(|e| e.to_string())
}

pub fn upsert_metadata(
    app: &tauri::AppHandle,
    target_path: &str,
    new_description: &str,
) -> Result<usize, String> {
    let mut conn = establish_connection(app)?;

    diesel::insert_into(folder_metadata)
        .values((
            path.eq(target_path),
            description.eq(new_description),
            updated_at.eq(now()),
        ))
        .on_conflict(path)
        .do_update()
        .set((
            description.eq(new_description),
            updated_at.eq(now()),
        ))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
}

pub fn remove_metadata(
    app: &tauri::AppHandle,
    target_path: &str,
) -> Result<usize, String> {
    let mut conn = establish_connection(app)?;

    diesel::delete(folder_metadata.filter(path.eq(target_path)))
        .execute(&mut conn)
        .map_err(|e| e.to_string())
}

pub fn get_metadata(
    app: &tauri::AppHandle,
    target_path: &str,
) -> Result<Option<NodeMetadata>, String> {
    let mut conn = establish_connection(app)?;

    match folder_metadata
        .filter(path.eq(target_path))
        .first(&mut conn)
    {
        Ok(data) => Ok(Some(data)),
        Err(Error::NotFound) => Ok(None),
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_all_metadata(
    app: &tauri::AppHandle,
) -> Result<Vec<NodeMetadata>, String> {
    let mut conn = establish_connection(app)?;

    folder_metadata
        .load::<NodeMetadata>(&mut conn)
        .map_err(|e| e.to_string())
}

pub fn get_all_descriptions(
    app: &tauri::AppHandle
) -> Result<Vec<(String, String)>, String> {
    let mut conn = establish_connection(app)?;

    let results = folder_metadata
        .select((path, description))
        .load::<(String, String)>(&mut conn)
        .map_err(|e| e.to_string())?;

    let valid = results
        .into_iter()
        .filter(|(p, _)| std::path::Path::new(p).exists())
        .collect();

    Ok(valid)
}

fn now() -> chrono::NaiveDateTime {
    Utc::now().naive_utc()
}