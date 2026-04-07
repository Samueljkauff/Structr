use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use chrono::Utc;
use crate::schema::folder_metadata::dsl::*;
use crate::domain::models::*;

pub fn establish_connection() -> Result<SqliteConnection, String> {
    SqliteConnection::establish("../metadata.db")
        .map_err(|e| e.to_string())
}

pub fn upsert_metadata(
    target_path: &str,
    new_description: &str,
) -> Result<usize, String> {
    let mut conn = establish_connection()?;

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
        .execute(&mut conn).map_err(|e| e.to_string())
}

pub fn remove_metadata(target_path: &str) -> Result<usize, String> {
    let mut conn = establish_connection()?;

    diesel::delete(folder_metadata.filter(path.eq(target_path)))
        .execute(&mut conn).map_err(|e| e.to_string())
}

pub fn get_metadata(target_path: &str) -> Result<NodeMetadata, String> {
    let mut conn = establish_connection()?;

        folder_metadata
        .filter(path.eq(target_path))
        .first(&mut conn).map_err(|e| e.to_string())
}

pub fn get_all_metadata() -> Result<Vec<NodeMetadata>, String> {
    let mut conn = establish_connection()?;

    folder_metadata.load::<NodeMetadata>(&mut conn).map_err(|e| e.to_string())
}

fn now() -> chrono::NaiveDateTime {
    Utc::now().naive_utc()
}
