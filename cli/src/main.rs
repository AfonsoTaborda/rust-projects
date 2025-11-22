use clap::Parser;
mod weather;

#[derive(Parser)]
#[command(name = "weather-cli", about = "A simple CLI to fetch weather data", long_about = None, rename_all = "snake-case")]
struct Cli {
    #[arg(long)]
    country_name: String,
    #[arg(long)]
    country_code: String,
}

fn main() {
    let cli = Cli::parse();

    let geocoding =
        match weather::geocoding::fetch_geocoding_data(&cli.country_name, &cli.country_code) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error fetching geocoding data: {}", e);
                return;
            }
        };
    let weather =
        match weather::weather::fetch_weather_data(geocoding.latitude, geocoding.longitude) {
            Ok(data) => Ok(data),
            Err(e) => Err(e),
        };

    match weather {
        Ok(weather) => weather.print_weather_info(),
        Err(e) => eprintln!("Error fetching weather data: {}", e),
    }
}
