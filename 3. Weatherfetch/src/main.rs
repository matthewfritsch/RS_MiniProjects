use serde_json;
use serde::{Deserialize, Serialize};
use std::fs;

fn main() {
    //TODO -release-
    // let request_url = _get_request_url();
    // let result = _fetch_weather_data(request_url);
    // let result_string = result.unwrap();
    //TODO -release-
    //TODO -debug-
    let result_string = _read_last_output();
    //TODO -debug-
    let _json_conversion = format_json(&result_string);
    println!("{}",&result_string);
}

fn _read_last_output() -> String {
    let api_file = "last_output.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}

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

fn _fetch_weather_data(req_url : String) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(req_url)?.text()?;
    Ok(response)
}

fn _fetch_api_key() -> String {
    let api_file = "weatherapi_key.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}

/*
fn determine_user_location() {}
fn manage_user_settings() {}
fn print_ascii() {}
*/