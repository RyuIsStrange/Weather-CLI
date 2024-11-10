use std::io;
use serde::Deserialize;
use colored::*;
use dotenv::dotenv;

// Struct to desierialize the JSON response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse{
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather{
    description: String,
}

// Struct to represent the main weather pramerters
#[derive(Deserialize, Debug)]
struct Main{
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind{
    speed: f64,
}

// Function to get the weather information from API
fn get_weather_info(city: &str, state_code: &str, country_code: &str, units: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{},{}&units={}&appid={}", 
        city, state_code, country_code, units, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather information
fn display_weather_info(response: &WeatherResponse, state_code: &str, units: &str) {
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}, {}: {}
        > Temerature: {:.1}°{},
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > Wind Speed: {:.1} {}",
        response.name,
        state_code,
        description,
        temperature,
        if units == "metric" {"C"} else if units == "imperial" {"F"} else {"K"},
        humidity,
        pressure,
        wind_speed,
        if units == "metric" {"m/s"} else if units == "imperial" {"mi/hr"} else {"m/s"},
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored: ColoredString = match description.as_str(){
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal()
    };
    println!("{}", weather_text_colored);
}
    
fn main() {
    dotenv().ok();
    println!("{}", "Welcome to Weather Station!".bright_yellow());
    
    println!("{}", "Please select your units. (Imperial / Metric / Standard)".bright_green());
    let mut units = String::new();
    io::stdin().read_line(&mut units).expect("Failed to read input!");
    let units = units.trim().to_lowercase();
    if units == "metric" {} else if units == "imperial" {} else if units == "standard" {} else {let _units = String::from("standard"); println!("{}", "You have selected a invaled unit. Defaulting to standard.".bright_red());}

    loop {
        // County Code
        println!("{}", "Please enter the country code (e.g., US for United States):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input!");
        let country_code = country_code.trim();

        // State Code
        println!("{}", "Please enter the state code (e.g., NY for New York):".bright_green());
        let mut state_code = String::new();
        io::stdin().read_line(&mut state_code).expect("Failed to read input!");
        let state_code = state_code.trim();

        // City
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input!");
        let city = city.trim();

        // Get your API key
        let api_key = std::env::var("OpenWeatherMap_API").expect("Weather API Key must be set (keyname: OpenWeatherMap_API)");
        
        // Calling the function to fetch weather information
        match get_weather_info(&city, &state_code, &country_code, &units, &api_key) {
            Ok(response) =>{
                display_weather_info(&response, &state_code, &units);
            }
            Err(err) =>{
                eprintln!("{}: {}", "Error".bright_red(), err)
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();
        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}