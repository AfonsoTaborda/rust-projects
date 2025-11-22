use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeocodingResponse {
    results: Vec<GeocodingResult>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub latitude: f32,
    pub longitude: f32,
}

pub async fn fetch_geocoding_data(name: &str, country: &str) -> Result<GeocodingResult, Error> {
    let url = format!(
        "https://geocoding-api.open-meteo.com/v1/search?name={}&count=10&language=en&format=json&countryCode={}",
        name, country
    );
    let response = match reqwest::get(&url).await {
        Err(e) => return Err(e),
        Ok(resp) => resp,
    };

    let geocoding: GeocodingResponse = match response.json().await {
        Err(e) => return Err(e),
        Ok(data) => data,
    };

    Ok(geocoding.results[0].clone())
}
