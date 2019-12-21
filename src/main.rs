use reqwest::Client;
use reqwest::header::{HeaderMap, CONTENT_TYPE, HeaderValue};
use serde::Serialize;
use serde_json::Value;
use std::{env, process};

#[derive(Serialize)]
struct TelegramParams {
    chat_id: String,
    text: String
}

fn send_request(params: &TelegramParams, token: &String) -> Result<(), String> {
    let cli = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE, 
        HeaderValue::from_static("application/x-www-form-urlencoded"));

    let res = cli
        .post(format!("https://api.telegram.org/bot{}/sendMessage", token).as_str())
        .headers(headers)
        .form(params)
        .send();
    
    let res = match res.and_then(|mut res| res.text()){
        Ok(res) => res,
        Err(e) => return Err(format!("{:?}", e))
    };

    let res: Value = match serde_json::from_str(res.as_str()) {
        Ok(value) => value,
        Err(e) => return Err(format!("{:?}", e))
    };

    if let Some(v) = res.get("ok").and_then(|val| val.as_bool()) {
        if !v {
            return Err(match res.get("description").and_then(|val| val.as_str()) {
                Some(e) => e.to_string(),
                None => format!("could not get error description")
            })
        };
    };
    
    Ok(())
}

fn main() {
    let telegram_token = match env::var("TELEGRAM_TOKEN") {
        Ok(token) => token,
        Err(_) => {
            eprintln!("environment variable TELEGRAM_TOKEN not defined");
            process::exit(1);
        }
    };

    let telegram_channel = match env::var("TELEGRAM_CHANNEL") {
        Ok(channel) => channel,
        Err(_) => {
            eprintln!("environment variable TELEGRAM_CHANNEL not defined");
            process::exit(1);
        }
    };

    let args: Vec<String> = env::args().skip(1).collect();
    let params = TelegramParams {
        chat_id: telegram_channel,
        text: args.join(" ")
    };

    if let Err(e) = send_request(&params, &telegram_token) {
        eprintln!("failed to send message: {}", e);
        process::exit(1);
    };
}
