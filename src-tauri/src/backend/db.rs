use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use chrono::Utc;
use crate::schema::folder_metadata::dsl::*;
use crate::domain::models::*;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("metadata.db")
        .expect("Error connecting to SQLite database")
}

pub fn upsert_metadata(
    target_path: &str,
    new_description: &str,
) -> QueryResult<usize> {
    let mut conn = establish_connection();

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
}

pub fn remove_metadata(target_path: &str) -> QueryResult<usize>{
    let mut conn = establish_connection();

    diesel::delete(folder_metadata.filter(path.eq(target_path)))
        .execute(&mut conn)
}

pub fn get_metadata(target_path: &str) -> QueryResult<NodeMetadata> {
    let mut conn = establish_connection();

        folder_metadata
        .filter(path.eq(target_path))
        .first(&mut conn)
}

pub fn get_all_metadata() -> QueryResult<Vec<NodeMetadata>> {
    let mut conn = establish_connection();

    folder_metadata.load::<NodeMetadata>(&mut conn)
}

fn now() -> chrono::NaiveDateTime {
    Utc::now().naive_utc()
}
