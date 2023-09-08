extern crate open_meteo_rs;
use chrono::{Local, Timelike, Datelike};
use config::Config;
use toml::Table;
use home;

#[tokio::main]
async fn main() {
    // Get default settings
    let mut st_path = home::home_dir().unwrap().to_owned();
    st_path.push(".config/meteocli.toml");

    let settings = Config::builder()
        .add_source(config::File::with_name(st_path.to_str().unwrap()))
        .build()
        .unwrap_or_default();

    // Initialize forecast report
    let client = open_meteo_rs::Client::new();
    let mut opts = open_meteo_rs::forecast::Options::default();

    // Receive location from config
    let config = settings.try_deserialize::<Table>().unwrap();
    // Location
    opts.location = open_meteo_rs::Location {
        lat: config["latitude"].as_float().unwrap_or_default(),
        lng: config["longitude"].as_float().unwrap_or_default(),
    };

    // Elevation
    opts.elevation = Some(open_meteo_rs::forecast::Elevation::Value(config["elevation"]
        .as_float()
        .unwrap_or_default() as f32
    ));

    // Current weather
    opts.current_weather = Some(true);

    // Temperature unit
    opts.temperature_unit = Some(open_meteo_rs::forecast::TemperatureUnit::Celsius);

    // Wind speed unit
    opts.wind_speed_unit = Some(open_meteo_rs::forecast::WindSpeedUnit::Kmh);

    // Precipitation unit
    opts.precipitation_unit = Some(open_meteo_rs::forecast::PrecipitationUnit::Millimeters);

    // Time zone (default to UTC)
    // opts.time_zone = Some(Local::now().offset().to_string());
    // println!("TZ = {}", Local::now().offset().to_string());

    // Forecast days (0-16)
    opts.forecast_days = Some(2);

    // Houly parameters
    opts.hourly.push("temperature_2m".into());
    opts.hourly.push("relativehumidity_2m".into());
    opts.hourly.push("precipitation_probability".into());
    opts.hourly.push("precipitation".into());
    opts.hourly.push("weathercode".into());

    // Get forecast
    let res = client.forecast(opts).await.unwrap();
    
    if let Some(hour_forecast) = res.hourly {
        let dt = Local::now();

        println!("{}", format!("{:<11}   {:<8}   {:<5}   {:<5}   {:<7}   {}", "Date", "Temperat", "Humi%", "Prec%", "Precipi", "Description"));

        for forecast in hour_forecast {
            let is_future_forecast = forecast.datetime.day() > dt.day() || forecast.datetime.hour() >= dt.hour();

            if is_future_forecast {
                print!("{}   ", forecast.datetime.format("%b %d, %Hh"));
                
                let temperature_2m = forecast.values.get("temperature_2m").unwrap();
                let relativehumidity_2m = forecast.values.get("relativehumidity_2m").unwrap();
                let precipitation_probability = forecast.values.get("precipitation_probability").unwrap();
                let precipitation = forecast.values.get("precipitation").unwrap();
                let weathercode = forecast.values.get("weathercode").unwrap();
                
                print!("{} {}   ", format!("{:>5}", temperature_2m.value.to_string()), temperature_2m.unit.as_ref().unwrap());
                print!("{} {}   ", format!("{:>3}", relativehumidity_2m.value.to_string()), relativehumidity_2m.unit.as_ref().unwrap());
                print!("{} {}   ", format!("{:>3}", precipitation_probability.value.to_string()), precipitation_probability.unit.as_ref().unwrap());
                print!("{} {}   ", format!("{:>4}", precipitation.value.to_string()), precipitation.unit.as_ref().unwrap());
                println!(" {}", get_description(weathercode.value.as_u64().unwrap()));
            }
        }
    }
}

fn get_description(weathercode: u64) -> &'static str {
    return match weathercode {
        0 => "Clear sky",
        1 => "Mainly clear",
        2 => "Partly cloudy",
        3 => "Overcast",
        45 => "Fog",
        48 => "Deposition rime fog",
        51 => "Drizzle: Light intensity",
        53 => "Drizzle: Moderate intensity",
        55 => "Drizzle: Dense intensity",
        56 => "Freezing Drizzle: Light intensity",
        57 => "Freezing Drizzle: Dense intensity",
        61 => "Rain: Slight intensity",
        63 => "Rain: Moderate intensity",
        65 => "Rain: Heavy intensity",
        66 => "Freezing Rain: Light intensity",
        67 => "Freezing Rain: Heavy intensity",
        71 => "Snow fall: Slight intensity",
        73 => "Snow fall: Moderate intensity",
        75 => "Snow fall: Heavy intensity",
        77 => "Snow grains",
        80 => "Rain showers: Slight",
        81 => "Rain showers: Moderate",
        82 => "Rain showers: Violent",
        85 => "Snow showers slight",
        86 => "Snow showers heavy",
        95 => "Thunderstorms: Slight or heavy",
        96 => "Thunderstorms with slight hail",
        99 => "Thunderstorms with heavy hail",
        _ => "Uknown weather code...",
    };
}