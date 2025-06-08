use serde::{Deserialize, Serialize};
use polars::prelude::*;
use std::io::{Cursor};

#[derive(Debug, Deserialize, Serialize)]
struct Driver {
    broadcast_name: Option<String>,
    country_code: Option<String>,
    driver_number: i32,
    first_name: Option<String>,
    full_name: Option<String>,
    headshot_url: Option<String>,
    last_name: Option<String>,
    meeting_key: u32,
    name_acronym: Option<String>,
    session_key: u32,
    team_colour: Option<String>,
    team_name: Option<String>,
}

const DRIVERS_URL: &str = "https://api.openf1.org/v1/drivers";

async fn fetch_drivers() -> Result<Vec<Driver>, reqwest::Error> {
    let response = reqwest::get(DRIVERS_URL).await?;
    let drivers: Vec<Driver> = response.json().await?;
    Ok(drivers)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let drivers = fetch_drivers().await.unwrap_or_else( |_| {
        println!("Failed to fetch drivers, using empty vector.");
        Vec::new()
    });

    let drivers_json = serde_json::to_string(&drivers).unwrap_or_else(|_| "[]".to_string());
    let cursor = Cursor::new(drivers_json);

    let df = match JsonReader::new(cursor).finish()
    {
        Ok(df) => df,
        Err(e) => {
            eprintln!("Error creating DataFrame: {}", e);
            DataFrame::default()
        }
    };

    print!("DataFrame:\n{}", df);

    Ok(())
}
