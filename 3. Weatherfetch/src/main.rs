// use serde_json::{Result, Value};
use std::fs;


fn main() {
    let request_url = get_request_url();
    let result = fetch_weather_data(request_url);
    // let json_conversion = format_json(result.unwrap());
    println!("{}",result.unwrap());
}

fn get_request_url() -> String {
    let req_prefix = "https://api.weatherapi.com/v1/current.json?key=";
    let api_key = fetch_api_key();
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

fn fetch_weather_data(req_url : String) -> Result<String, reqwest::Error> {
    let response = reqwest::blocking::get(req_url)?.text()?;

    // println!("{:?}", response.text().await?);
    Ok(response)
}

fn fetch_api_key() -> String {
    let api_file = "weatherapi_key.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}
// fn format_json(json : String) ->  {
//     let fixed_json = serde_json::from_str(json);
//     fixed_json
// }

/*
fn determine_user_location() {}
fn manage_user_settings() {}
fn print_ascii() {}
*/