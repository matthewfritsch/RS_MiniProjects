use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;

//TODO test untyped JSON values to eliminate huge struct definitions
#[derive(Serialize, Deserialize)]
struct Condition {
    text : String,
    icon : String,
    code : i16,
}

#[derive(Serialize, Deserialize)]
struct Current {
    last_updated_epoch: i64,
    last_updated: String,
    temp_c: f32,
    temp_f: f32,
    is_day: i8,
    condition: Condition,
    wind_mph: f32,
    wind_kph: f32,
    wind_degree: i16,
    wind_dir: String,
    pressure_mb: f32,
    pressure_in: f32,
    precip_mm: f32,
    precip_in: f32,
    humidity: i8,
    cloud: i8,
    feelslike_c: f32,
    feelslike_f: f32,
    vis_km: f32,
    vis_miles: f32,
    uv: f32,
    gust_mph: f32,
    gust_kph: f32,
}

#[derive(Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f32,
    lon: f32,
    localtime_epoch: i64,
    localtime: String,
}

#[derive(Serialize, Deserialize)]
struct Weatherdata {
    location : Location,
    current : Current,
}

fn main() {

    //TODO -swap-in the below lines (3) for release-
    // let request_url = _get_request_url();
    // let result = _fetch_weather_data(request_url);
    // let result_string = result.unwrap();

    //TODO -swap-out the below line to end debug-
    let result_string = _read_last_output();

    let _json_conversion = format_json(&result_string);
    println!("{}",&result_string);
}

//TODO remove underscore in function name for release
fn _read_last_output() -> String {
    let api_file = "last_output.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}

//TODO remove underscore in function name for release
fn _get_request_url() -> String {
    let req_prefix = "https://api.weatherapi.com/v1/current.json?key=";
    let api_key = _fetch_api_key();
    let req_option = "&q=";
    let user_loc = "95060";
    let req_suffix = "&aqi=no";
    let mut request_url = String::new();
    request_url.push_str(req_prefix);
    request_url += &api_key; 
    request_url.push_str(req_option); 
    request_url.push_str(user_loc);
    request_url.push_str(req_suffix);
    request_url
}

//TODO remove underscore in function name for release
fn _fetch_weather_data(req_url : String) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(req_url)?.text()?;
    Ok(response)
}

//TODO remove underscore in function name for release
fn _fetch_api_key() -> String {
    let api_file = "weatherapi_key.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}

fn format_json(json : &String) -> Weatherdata {
    let weather : Weatherdata = serde_json::from_str(json.as_str()).unwrap();
    weather
}

fn get_fetch_text(ascii:&String, weather_text:&String) -> String {
    let mut to_return = String::new();
    let mut ascii_vec : Vec<String> = Vec::new();
    let mut weather_vec : Vec<String> = Vec::new();

    for line in ascii.lines() {
        ascii_vec.push(line.to_string());
    }
    for line in weather_text.lines() {
        weather_vec.push(line.to_string());
    }

    let mut ascii_iter = ascii_vec.iter();
    let mut weather_iter = weather_vec.iter();

    loop {
        match (ascii_iter.next(), weather_iter.next()) {
            (Some(x), Some(y)) => to_return.push_str(format!("{}{}\n", x, y).as_str()),
            (Some(x), None) => to_return.push_str(format!("{}\n", x).as_str()),
            (None, Some(y)) => to_return.push_str(format!("{}\n", y).as_str()),
            (None, None) => break,
        }
    }

    println!("{}", to_return);
    to_return
}

/*
fn initial_setup() {}
fn choose_ascii() {}
*/