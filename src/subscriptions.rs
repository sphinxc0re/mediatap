use crate::{
    config::Config,
    errors::Result,
    models,
    paths::{self, subscriptions_dir},
};
use chrono::NaiveDate;
use dialoguer::{Input, Select};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use std::{
    fmt::{self, Display},
    fs::{self, read_to_string},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Quality {
    Low,
    Medium,
    High,
}

impl Display for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let quality = match self {
            Quality::Low => "low",
            Quality::Medium => "medium",
            Quality::High => "high",
        };

        write!(f, "{}", quality)
    }
}

/// LAZY: Impl Default
fn default_quality() -> Quality {
    Quality::Medium
}

#[derive(Deserialize, Serialize)]
struct Subscription {
    /// The search term
    term: String,
    minimum_length: i64,
    #[serde(default = "default_quality")]
    quality: Quality,
    identifier: String,
}

impl Subscription {
    pub fn load(path: &Path) -> Result<Subscription> {
        let content = read_to_string(path)?;

        let sub = toml::from_str(&content)?;

        Ok(sub)
    }

    pub fn execute(
        &self,
        conn: &SqliteConnection,
    ) -> Result<Vec<(String, Option<NaiveDate>, String)>> {
        use crate::schema::mediathek_entries::dsl::*;
        use diesel::prelude::*;

        let results: Vec<(
            String,
            Option<NaiveDate>,
            String,
            Option<String>,
            Option<String>,
        )> = mediathek_entries
            .filter(title.like(&self.term).or(topic.like(&self.term)))
            .filter(duration.gt(&self.minimum_length))
            .order_by(date.desc())
            .select((title, date, url, url_small, url_hd))
            .load(conn)?;

        Ok(results
            .into_iter()
            .map(
                |(data_title, data_date, data_url, data_url_small, data_url_hd)| {
                    let quality_url = match self.quality {
                        Quality::Low => data_url_small.unwrap_or_else(|| data_url),
                        Quality::Medium => data_url,
                        Quality::High => data_url_hd.unwrap_or_else(|| data_url),
                    };

                    (data_title, data_date, quality_url)
                },
            )
            .collect())
    }
}

pub fn execute_all() -> Result<()> {
    let connection = models::establish_connection()?;
    let config = Config::load()?;

    let sub_dir = subscriptions_dir()?;

    let iter = sub_dir
        .read_dir()?
        .filter_map(std::result::Result::ok)
        .map(|dir_entry| dir_entry.path())
        .filter(|entry| entry.is_file());

    let mut all_download_futures = Vec::new();

    for file in iter {
        let sub = Subscription::load(&file)?;

        let download_dir = config.base_directory.join(sub.identifier.clone());

        fs::create_dir_all(&download_dir)?;

        let urls = sub.execute(&connection)?;

        let mut futures: Vec<_> = urls
            .into_iter()
            .map(|(title, date, url)| {
                let dir = download_dir.clone();
                let date_str = date
                    .map(|date| format!("{}", date.format("%Y-%m-%d")))
                    .unwrap_or("unknown_date".to_string());

                let title_str = title.trim().to_lowercase().replace(' ', "_");

                let ext = url
                    .rsplit(".")
                    .nth(0)
                    .expect("This has to be ending with something");

                let file_name_base = format!("{date}_{title}", date = date_str, title = title_str,);

                let file_name = PathBuf::from(file_name_base).with_extension(ext);

                download(url.clone(), dir.clone(), file_name)
            })
            .collect();

        all_download_futures.append(&mut futures);
    }

    let runtime = tokio::runtime::Runtime::new()?;

    let results = runtime.block_on(futures::future::join_all(all_download_futures));

    let errors: Vec<_> = results.into_iter().filter_map(|res| res.err()).collect();

    if !errors.is_empty() {
        eprintln!("{} errors:", errors.len());
        eprintln!();

        for error in errors {
            eprintln!("Error: {}", error);
        }
    }

    Ok(())
}

async fn download(url: String, target_dir: PathBuf, file_name: PathBuf) -> Result<()> {
    println!("start download... {}", file_name.display());
    let response = reqwest::get(&url).await?;
    let bytes = response.bytes().await?;

    fs::write(target_dir.join(&file_name), bytes)?;
    println!("Finished {}!", file_name.display());

    Ok(())
}

pub fn new() -> Result<()> {
    let term: String = Input::new().with_prompt("Term").interact_text()?;
    let minimum_length: u32 = Input::new()
        .with_prompt("Minimum length (in seconds)")
        .interact_text()?;

    let mut quality_items = vec![Quality::High, Quality::Medium, Quality::Low];
    let quality_idx: usize = Select::new()
        .with_prompt("Please select a preferred quality")
        .items(quality_items.as_slice())
        .default(1)
        .interact()?;
    let quality = quality_items.remove(quality_idx);

    let identifier: String = Input::new()
        .with_prompt("Specify a unique identifier for the subscription")
        .interact_text()?;

    let subscription = Subscription {
        term,
        minimum_length: minimum_length as i64,
        quality,
        identifier: identifier.clone(),
    };

    let sub_content = toml::to_vec(&subscription)?;

    let sub_path = paths::subscriptions_dir()?;
    let full_sub_path = sub_path.join(identifier).with_extension("toml");

    fs::write(&full_sub_path, sub_content.as_slice())?;

    println!(
        "Subscription has successfully been written to {}",
        full_sub_path.display()
    );

    Ok(())
}
