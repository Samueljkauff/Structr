use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use crate::schema::folder_metadata;

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("metadata.db")
        .expect("Error connecting to SQLite database")
}

pub fn create_metadata() {
    let mut conn = establish_connection();

}

pub fn update_metadata() {
    let mut conn = establish_connection();

}

pub fn remove_metadata() {
    let mut conn = establish_connection();

}

pub fn get_metadata() {
    let mut conn = establish_connection();

}

pub fn get_all_metadata() {
    let mut conn = establish_connection();

}
