use clap::{App, Arg, SubCommand};
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
    let matches = App::new("Reviews Service")
        .version("1.0")
        .subcommand(SubCommand::with_name("api").about("Starts the API server"))
        .subcommand(
            SubCommand::with_name("events")
                .about("Starts the Kafka listener")
                .version("1.0")
                .author("Your Name <your.email@example.com>")
                .arg(
                    Arg::with_name("brokers")
                        .short("b")
                        .long("brokers")
                        .value_name("BROKERS")
                        .help("Sets a brokers for the Kafka listener")
                        .takes_value(true),
                ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("api") {
        let addr = matches
            .value_of("address")
            .unwrap_or("127.0.0.1:8000")
            .parse()
            .unwrap();
        presentation::api::start_api_server(addr).await;
    } else if let Some(matches) = matches.subcommand_matches("events") {
        let brokers = matches.value_of("brokers").unwrap_or("localhost:9092");
        let topic = matches.value_of("topic").unwrap_or("reviews");
        presentation::kafka::start_listener(brokers, topic).await;
    }
}
