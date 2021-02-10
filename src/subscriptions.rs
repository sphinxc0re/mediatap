use std::path::PathBuf;

use crate::{errors::Result, models};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Quality {
    Low,
    Medium,
    High,
}

#[derive(Deserialize, Serialize)]
pub struct Subscription {
    /// The search term
    term: String,
    minimum_length: i64,
    quality: Quality,
    directory: PathBuf,
}

impl Subscription {
    pub fn new_boemi() -> Subscription {
        Subscription {
            term: "ZDF Magazin Royale".to_string(),
            minimum_length: 1500,
            quality: Quality::High,
            directory: PathBuf::from("./"),
        }
    }

    pub fn execute(&self, conn: &SqliteConnection) -> Result<Vec<String>> {
        use crate::schema::mediathek_entries::dsl::*;
        use diesel::prelude::*;

        let results: Vec<(String, Option<String>, Option<String>)> = mediathek_entries
            .filter(title.like(&self.term).or(topic.like(&self.term)))
            .filter(duration.gt(&self.minimum_length))
            .order_by(date.desc())
            .select((url, url_small, url_hd))
            .load(conn)?;

        Ok(results
            .into_iter()
            .map(
                |(data_url, data_url_small, data_url_hd)| match self.quality {
                    Quality::Low => data_url_small.unwrap_or_else(|| data_url),
                    Quality::Medium => data_url,
                    Quality::High => data_url_hd.unwrap_or_else(|| data_url),
                },
            )
            .collect())
    }
}

pub fn run() -> Result<()> {
    let connection = models::establish_connection()?;
    let sub = Subscription::new_boemi();
    let res = sub.execute(&connection)?;

    println!("{:#?}", res);

    Ok(())
}
