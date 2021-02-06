#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod consts;
mod input_data;
mod paths;
mod schema;
mod url_util;

use crate::consts::{BASE_URL, FILM_LIST_FILE_NAME};
use crate::input_data::InputData;
use chrono::{NaiveDate, NaiveTime};
use clap::Clap;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::embed_migrations;
use json_minimal::Json;
use std::io::{self, Read, Write};
use xz2::read::XzDecoder;

#[derive(Clap)]
enum Cmd {
    /// Updates the film-list
    Update {
        #[clap(default_value = BASE_URL, long)]
        server_url: String,
    },
    #[cfg(debug_assertions)]
    /// Emits the os-specific path to the local database
    EmitDatabasePath,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Cmd::parse();

    match cmd {
        Cmd::Update { server_url } => run(server_url),
        #[cfg(debug_assertions)]
        Cmd::EmitDatabasePath => {
            println!("{}", paths::database_dir()?.display());

            Ok(())
        }
    }
}

embed_migrations!();

fn run(server_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let db_path = paths::database_dir()?;

    let db_path_str = db_path
        .to_str()
        .expect("database path contains invalid unicode");
    let connection = SqliteConnection::establish(db_path_str)?;

    embedded_migrations::run(&connection)?;

    let list_url = format!("{}/{}", server_url, FILM_LIST_FILE_NAME);

    print!("Fetching list...");
    io::stdout().flush().unwrap();
    let bytes = reqwest::blocking::get(&list_url)?.bytes()?;
    println!(" done!");

    print!("Decompressing...");
    io::stdout().flush().unwrap();
    let mut compressed_list = XzDecoder::new(&*bytes);

    let mut contents = Vec::new();
    compressed_list.read_to_end(&mut contents).unwrap();
    println!(" done!");

    print!("Parsing + transforming...");
    io::stdout().flush().unwrap();
    let root_value = Json::parse(&contents).unwrap();

    if let Json::JSON(inner) = root_value {
        let mut last_station = String::new();
        let mut last_topic = String::new();

        let values: Vec<_> = inner
            .iter()
            .skip(2)
            .filter_map(|entry| {
                if let Json::OBJECT { value, .. } = entry {
                    let inner = value.unbox();

                    if let Json::ARRAY(inner_ary) = inner {
                        let str_vec: Vec<_> = inner_ary
                            .iter()
                            .map(|jsn| {
                                if let Json::STRING(inner) = jsn {
                                    inner.to_owned()
                                } else {
                                    String::new()
                                }
                            })
                            .collect();

                        return Some(str_vec);
                    }
                }

                None
            })
            .map(|mut row| InputData {
                station: row.remove(0),
                topic: row.remove(0),
                title: row.remove(0),
                date: date_from(row.remove(0).as_str()),
                time: time_from(row.remove(0).as_str()),
                duration: duration_from(row.remove(0).as_str()),
                size: row.remove(0),
                description: row.remove(0),
                url: row.remove(0),
                website: row.remove(0),
                url_subtitles: row.remove(0),
                url_rtmp: row.remove(0),
                url_small: row.remove(0),
                url_rtmp_small: row.remove(0),
                url_hd: row.remove(0),
                url_rtmp_hd: row.remove(0),
                datuml: row.remove(0),
                url_history: row.remove(0),
                geo: row.remove(0),
                new: row.remove(0),
            })
            .map(|input_data| {
                if !input_data.station.is_empty() {
                    last_station = input_data.station;
                }

                if !input_data.topic.is_empty() {
                    last_topic = input_data.topic;
                }

                InputData {
                    station: last_station.clone(),
                    topic: last_topic.clone(),
                    ..input_data
                }
            })
            .map(|input_data| {
                let url_small = if !input_data.url_small.is_empty() {
                    url_util::expand_to_full_url(&input_data.url, &input_data.url_small)
                } else {
                    String::new()
                };

                let url_hd = if !input_data.url_hd.is_empty() {
                    url_util::expand_to_full_url(&input_data.url, &input_data.url_hd)
                } else {
                    String::new()
                };

                InputData {
                    url_small,
                    url_hd,
                    ..input_data
                }
            })
            .collect();
        println!(" done!");

        use crate::diesel::RunQueryDsl;

        print!("Building Database...");
        io::stdout().flush().unwrap();

        diesel::delete(crate::schema::mediathek_entries::table).execute(&connection)?;

        diesel::insert_into(crate::schema::mediathek_entries::table)
            .values(values.as_slice())
            .execute(&connection)?;

        println!(" done!");
    }

    Ok(())
}

fn duration_from(s: &str) -> Option<i64> {
    let fake_time = NaiveTime::parse_from_str(s, "%H:%M:%S").ok()?;

    let fake_start = NaiveTime::from_hms(0, 0, 0);

    let duration = fake_time.signed_duration_since(fake_start);

    Some(duration.num_seconds())
}

fn date_from(s: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(s, "%d.%m.%Y").ok()
}

fn time_from(s: &str) -> Option<NaiveTime> {
    NaiveTime::parse_from_str(s, "%H:%M:%S").ok()
}
