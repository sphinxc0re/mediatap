#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

mod input_data;
mod schema;
mod url_util;

use crate::input_data::InputData;
use async_std::task;
use clap::Clap;
use diesel::{Connection, SqliteConnection};
use diesel_migrations::embed_migrations;
use directories::ProjectDirs;
use json_minimal::Json;
use std::io::{self, Read, Write};
use xz2::read::XzDecoder;

const BASE_URL: &str = "https://liste.mediathekview.de";
const FILM_LIST_FILE_NAME: &str = "Filmliste-akt.xz";

const DB_FILE_NAME: &str = "db.sqlite";

#[derive(Clap)]
enum Cmd {
    /// Updates the film-list
    Update {
        #[clap(default_value = BASE_URL, long)]
        server_url: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = Cmd::parse();

    match cmd {
        Cmd::Update { server_url } => run(server_url),
    }
}

embed_migrations!();

fn run(server_url: String) -> Result<(), Box<dyn std::error::Error>> {
    let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME")).unwrap();

    let base_dir = dirs.data_dir().to_path_buf();
    if !base_dir.exists() {
        std::fs::create_dir_all(&base_dir)?;
    }

    let db_path = base_dir.join(DB_FILE_NAME);

    let connection = SqliteConnection::establish(db_path.to_str().unwrap())?;

    embedded_migrations::run(&connection)?;

    let list_url = format!("{}/{}", server_url, FILM_LIST_FILE_NAME);

    print!("Fetching list...");
    io::stdout().flush().unwrap();
    let bytes = task::block_on(surf::get(&list_url).recv_bytes())?;
    println!(" done!");

    print!("Decompressing...");
    io::stdout().flush().unwrap();
    let mut compressed_list = XzDecoder::new(bytes.as_slice());

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
                date: row.remove(0),
                time: row.remove(0),
                duration: row.remove(0),
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
                if !input_data.station.is_empty() && !input_data.topic.is_empty() {
                    last_station = input_data.station;
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
            .values(&values)
            .execute(&connection)?;

        println!(" done!");
    }

    Ok(())
}
