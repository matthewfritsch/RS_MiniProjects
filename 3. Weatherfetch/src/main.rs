// use std::io;
use std::fs;
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
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
    let response = reqwest::get(request_url).await?;

    println!("{:?}", response.text().await?);
    Ok(())
}

fn fetch_api_key() -> String {
    let api_file = "weatherapi_key.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}
/*
fn format_json() {}
fn determine_user_location() {}
fn manage_user_settings() {}
fn print_ascii() {}
*/