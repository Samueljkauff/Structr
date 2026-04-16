use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct NodeMetadata {
    pub path: String,
    pub description: String,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::folder_metadata)]
pub struct NewNodeMetadata<'a> {
    pub path: &'a str,
    pub description: &'a str,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, serde::Serialize)]
pub struct FileMoveDisplay {
    pub id: i32,
    pub from_path: String,
    pub to_path: String,
    pub moved_at: Option<NaiveDateTime>,
}