use crate::errors::Result;
use crate::paths;
use crate::schema::mediathek_entries;
use chrono::{NaiveDate, NaiveTime};
use diesel::Insertable;
use diesel::{Connection, SqliteConnection};

pub fn establish_connection() -> Result<SqliteConnection> {
    let db_path = paths::database_dir()?;

    let db_path_str = format!("{}", db_path.display());
    let connection = SqliteConnection::establish(&db_path_str)?;

    Ok(connection)
}

#[derive(Insertable, Debug)]
#[table_name = "mediathek_entries"]
pub struct NewEntry {
    pub station: String,
    pub topic: String,
    pub title: String,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i64>,
    /// The size in MB
    pub size: String,
    pub description: String,
    pub url: String,
    pub website: String,
    pub url_subtitles: String,
    pub url_rtmp: String,
    pub url_small: String,
    pub url_rtmp_small: String,
    pub url_hd: String,
    pub url_rtmp_hd: String,
    pub datuml: String,
    pub url_history: String,
    pub geo: String,
    pub new: String,
}
