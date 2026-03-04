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
    let time = time.trim();

    let date = &time[..time.len() - 6];
    let time_str = &time[10..];

    let get_weather_url = format!(
        "https://api.open-meteo.com/v1/forecast?{}&hourly=temperature_2m,precipitation,precipitation_probability&timezone=auto&start_date={}&end_date={}",
        location, date, date
    );

    let raw_weather_string = reqwest::blocking::get(&get_weather_url)
        .unwrap()
        .text()
        .unwrap()
        .trim()
        .to_string();

    let v: Value = serde_json::from_str(&raw_weather_string).unwrap();

    let vec_number = match time_str {
        "T00:00" => 0,
        "T01:00" => 1,
        "T02:00" => 2,
        "T03:00" => 3,
        "T04:00" => 4,
        "T05:00" => 5,
        "T06:00" => 6,
        "T07:00" => 7,
        "T08:00" => 8,
        "T09:00" => 9,
        "T10:00" => 10,
        "T11:00" => 11,
        "T12:00" => 12,
        "T13:00" => 13,
        "T14:00" => 14,
        "T15:00" => 15,
        "T16:00" => 16,
        "T17:00" => 17,
        "T18:00" => 18,
        "T19:00" => 19,
        "T20:00" => 20,
        "T21:00" => 21,
        "T22:00" => 22,
        "T23:00" => 23,
        _ => {
            println!("Invalid time format.");
            return;
        }
    };
    println!("{}:", time);
    if let Some(temp_array) = v["hourly"]["temperature_2m"].as_array() {
        if let Some(first_temp) = temp_array.get(vec_number).and_then(Value::as_f64) {
            println!("Temperature: {:.1}°C", first_temp);
        } else {
            println!("Temperature data is not available for the specified time.");
        }
    } else {
        println!("Temperature array is not available.");
    }

    if let Some(prec_array) = v["hourly"]["precipitation"].as_array() {
        if let Some(first_prec) = prec_array.get(vec_number).and_then(Value::as_f64) {
            println!("Precipitation: {:.1}mm", first_prec);
        } else {
            println!("Precipitation data is not available for the specified time.");
        }
    } else {
        println!("Precipitation array is not available.");
    }

    if let Some(prec_prob_array) = v["hourly"]["precipitation_probability"].as_array() {
        if let Some(first_prec_prob) = prec_prob_array.get(vec_number).and_then(Value::as_f64) {
            println!("Precipitation probability: {:.1}%", first_prec_prob);
        } else {
            println!("Precipitation data is not available for the specified time.");
        }
    } else {
        println!("Precipitation array is not available.");
    }
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
