use crate::schema::mediathek_entries;
use diesel::Insertable;

#[derive(Insertable, Debug)]
#[table_name = "mediathek_entries"]
pub struct InputData {
    pub station: String,
    pub topic: String,
    pub title: String,
    pub date: String,
    pub time: String,
    pub duration: String,
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
