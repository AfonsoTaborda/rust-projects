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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let geocoding =
        weather::geocoding::fetch_geocoding_data(&cli.country_name, &cli.country_code).await;
    let geocoding = match geocoding {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error fetching geocoding data: {}", e);
            return;
        }
    };
    let weather =
        weather::weather::fetch_weather_data(geocoding.latitude, geocoding.longitude).await;

    match weather {
        Ok(data) => data.print_weather_info(),
        Err(e) => eprintln!("Error fetching weather data: {}", e),
    }
}
