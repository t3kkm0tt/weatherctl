use serde_json::{Result, Value, json};
use std::io::Write;

use geolocation;
use reqwest;
fn getlocation() -> String {
    let ip = reqwest::blocking::get("https://icanhazip.com")
        .unwrap()
        .text()
        .unwrap()
        .trim()
        .to_string();
    let info = geolocation::find(&ip).unwrap();

    println!("{}", info.latitude);
    println!("{}", info.longitude);
    let location_string = format!(
        "latitude={}&longitude={}",
        info.latitude.to_string(),
        info.longitude.to_string()
    );
    println!("{}", location_string);
    return location_string;
}

fn getforecastweather(location: &String) {
    println!("Enter the Time you want to get the forecast for. Format: YYYY-MM-DDThh:mm");
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut time = String::new();
    std::io::stdin().read_line(&mut time).unwrap();
    println!("{}", time);
    let get_weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?{}&hourly=temperature_2m&timezone=auto",
        location
    );
    let raw_weather_string = reqwest::blocking::get(get_weather_url)
        .unwrap()
        .text()
        .unwrap()
        .trim()
        .to_string();
    let v: Value = serde_json::from_str(&raw_weather_string).unwrap();
    let weather_reformatted = reformat_weather(v);
    let weather_json_time = &weather_reformatted[&time.trim()];
    println!(
        "{}:\nTemperature: {}",
        time.trim(),
        weather_json_time["temperature"]
    );
    println!("{}", weather_json_time)
}

fn reformat_weather(data: Value) -> Value {
    let times = data["hourly"]["time"].as_array().unwrap();
    let temps = data["hourly"]["temperature_2m"].as_array().unwrap();

    let mut map = serde_json::Map::new();

    for (i, time) in times.iter().enumerate() {
        let time_str = time.as_str().unwrap().to_string();
        let temp = temps[i].clone();

        map.insert(time_str, json!({ "temperature": temp }));
    }

    Value::Object(map)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let flags = &args[1..];
    let location = getlocation();
    for arg in flags {
        match arg.as_str() {
            "-f" | "--forecast" => getforecastweather(&location),
            "-h" | "--help" => println!(
                "Usage: ./weatherctl [-f, -h] [Options]\n-f, --forecast: Get the forecast for a specific time TODAY or up to 7 days in the future. "
            ),
            _ => println!("Unknown argument: {}", arg),
        }
    }
}
