// mod
mod utils;

// std lib
use std::fs::File;
use std::io::Read;

// libs
use clap::{arg, Command};
use colored::Colorize;
use serde_json::{from_str, Value};

#[tokio::main]
async fn main() {

    utils::banner();
    utils::copyright();

    let matches = Command::new("Darkeye")
        .version("1.0")
        .author("v7sr14ul2x9")
        .arg(arg!(
            -u --username <username> "Search for an username"
        )
            .required(true)
        )
        .get_matches();

    if let Some(username) = matches.get_one::<String>("username") {

        println!("Searching for {username}\n");

        let mut file = File::open(r"D:\Project\Darkeye\src\data.json").expect("Can't open file");
        let mut ctx = String::new();
        file.read_to_string(&mut ctx).expect("Can't read file");
        let data: Value = from_str(&ctx).expect("Can't parse file");

        if let Value::Object(data) = data {
            if let Some(site) = data.get("site") {
                if let Value::Array(sites) = site {
                    for site in sites {
                        if let Value::Object(site) = site {
                            if let Some(app) = site.get("app") {
                                if let Value::String(app) = app {

                                    let url = app.replace("<username>", username);

                                    let client = reqwest::Client::new();
                                    let res = client.get(url.clone())
                                        .header("User-Agent", "Darkeye")
                                        .send()
                                        .await;

                                    let res = res.expect("Can't get response");

                                    // with http code
                                    if let Some(code) = site.get("code") {
                                        let status = res.status();
                                        let response_code = status.as_str();

                                        if code == response_code {
                                            println!("{}{}", url, " Found".green())
                                        }
                                        else {
                                            println!("{}{}", url, " Not Found".red())
                                        }

                                    }
                                    // to finish
                                    // with response text
                                    if let Some(response) = site.get("response") {
                                        let body = res.text().await.expect("");

                                        if !body.contains(&Value::to_string(response)) {
                                            println!("{}{}", url, " Found".green())
                                        }
                                        else {
                                            println!("{}{}", url, " Not Found".red())
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}