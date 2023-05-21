use clap::{App, Arg, Command, SubCommand};
use std::collections::HashMap;

mod domain;
mod presentation;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;

    println!("{:#?}", resp);
    Ok(())
}

#[tokio::main]
async fn clap_main() {
    let matches = Command::new("Reviews Service")
        .version("1.0")
        .subcommand(SubCommand::with_name("api").about("Starts the API server"))
        .subcommand(
            SubCommand::with_name("events")
                .about("Starts the Kafka listener")
                .version("1.0")
                .author("Your Name <your.email@example.com>")
                .arg(
                    Arg::new("brokers")
                        .short('n')
                        .long("brokers")
                        .help("Sets a brokers for the Kafka listener")
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("api", ) => {
            presentation::api::start_api_server("127.0.0.1:8000".into()).await;
        },
        Some("events") => {
            let brokers = matches.get_one::<String>("brokers")
                .unwrap_or(&"localhost:9092".to_string());

            presentation::kafka::start_listener(brokers, "reviews").await;
        }
    }
}
