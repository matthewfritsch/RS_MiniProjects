use std::{fs, net::ToSocketAddrs};
use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let req_prefix = "https://api.weatherapi.com/v1/current.json?key=";
    let api_key = fetch_api_key();
    let req_option = "&q=";
    let user_loc = "San Jose";
    let req_suffix = "&aqi=no";
    let request_url = req_prefix
    + api_key.to_str() 
    + req_option 
    + user_loc
    + req_suffix;
    let response = reqwest::get(request_url).await?;

    println!("{:?}", response.text().await?);
    Ok(())
}

fn fetch_api_key() -> string {
    let api_file = "weatherapi_key.txt";
    fs::read_to_string(api_file).expect("File could not be accessed.")
}
/*
fn format_json() {}
fn determine_user_location() {}
fn manage_user_settings() {}
fn print_ascii() {}
*/