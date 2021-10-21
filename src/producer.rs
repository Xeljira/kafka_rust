use std::time::Duration;

use clap::{App, Arg};
use log::info;

use std::io;

use rdkafka::config::ClientConfig;
use rdkafka::message::OwnedHeaders;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

async fn produce(brokers: &str, topic_name: &str) {

    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    println!("Enter your message");
    let mut message = String::new();

    io::stdin()
        .read_line(&mut message)
        .expect("Failed to read input");

     let _delivery_status = producer
        .send(
            FutureRecord::to(topic_name)
                .payload(&format!("Message {}", message))
                .key(&format!("Key {}", message))
                .headers(OwnedHeaders::new().add("header_key", "header_value")),
                Duration::from_secs(0),
            )
        .await;

            // This will be executed when the result is received.
            info!("Delivery status for message {} received", message);

}

#[tokio::main]
async fn main() {
    let matches = App::new("producer example")
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("Simple command line producer")
        .arg(
            Arg::with_name("brokers")
                .short("b")
                .long("brokers")
                .help("Broker list in kafka format")
                .takes_value(true)
                .default_value("localhost:9092"),
        )
        .arg(
            Arg::with_name("topic")
                .short("t")
                .long("topic")
                .help("Destination topic")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let (version_n, version_s) = get_rdkafka_version();
    info!("rd_kafka_version: 0x{:08x}, {}", version_n, version_s);

    let topic = matches.value_of("topic").unwrap();
    let brokers = matches.value_of("brokers").unwrap();

    produce(brokers, topic).await;
}
