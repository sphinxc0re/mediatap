use crate::schema::mediathek_entries;
use chrono::{NaiveDate, NaiveTime};
use diesel::Insertable;

#[derive(Insertable, Debug)]
#[table_name = "mediathek_entries"]
pub struct InputData {
    pub station: String,
    pub topic: String,
    pub title: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: i64,
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
