use std::io;
use serde::Deserialize;
use colored::*;
use dotenv::dotenv;

// Struct to deserialize the JSON response from
// OpenWeatherMap API

#[derive(Deserialize, Debug)]
struct WeatherAPIResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent the weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind parameters
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherAPIResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}, {}&units=metric&appid={}",
        city, country_code, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let json_data: WeatherAPIResponse = response.json::<WeatherAPIResponse>()?;
    Ok(json_data)
}

fn get_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤"
    } else {
        "☀"
    }
}

fn display_weather_info(response: &WeatherAPIResponse) {
    let description = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;
    let weather_text = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}
        > Humidity: {:.1}
        > Pressure: {:.1} hPa
        > Wind Speed: {:.1}m/s
        ", response.name, description, temperature, get_emoji(temperature), humidity, pressure, wind_speed
    );

    let text_colour = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    println!("{}", text_colour);
}


fn main() {
    dotenv().ok();
    println!("{}", "Welcome to the Weather Station!".bright_yellow());
    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city: &str = city.trim();

        println!("{}", "Please enter the country code:".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code: &str = country_code.trim();

        let api_key = std::env::var("OPEN_WEATHER_API_KEY")
            .expect("OPEN_WEATHER_API_KEY must be set.");
        match get_weather_info(&city, &country_code, &*api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error: {}", err);
            }
        }

        println!("{}", "Do you want to check the weather in another city? (yes/no)".bright_green());
        let mut check_again = String::new();
        io::stdin().read_line(&mut check_again).expect("Failed to read input!");
        let check_again: &str = check_again.trim();
        if check_again != "yes" {
            println!("Thank you for using the weather app!");
            break;
        }
    }
}
