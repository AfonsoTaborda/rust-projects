use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentWeather {
    time: String,
    interval: i32,
    temperature_2m: f32,
    wind_speed_10m: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub current: CurrentWeather,
}

impl Weather {
    pub fn print_weather_info(&self) {
        println!("Current Temperature: {}°C", self.current.temperature_2m);
        println!("Wind Speed: {} m/s", self.current.wind_speed_10m);
    }
}

pub fn fetch_weather_data(latitude: f32, longitude: f32) -> Result<Weather, Error> {
    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,wind_speed_10m&hourly=temperature_2m,relative_humidity_2m,wind_speed_10m",
        latitude, longitude
    );
    let response = match reqwest::blocking::get(&url) {
        Err(e) => return Err(e),
        Ok(data) => data,
    };

    let weather: Weather = match response.json() {
        Err(e) => return Err(e),
        Ok(data) => data,
    };

    Ok(weather)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_weather_data() {
        let result = fetch_weather_data(52.52, 13.405);
        assert!(result.is_ok());
    }
}
