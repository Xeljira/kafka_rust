use rdkafka::config::ClientConfig;
use std::boxed::Box;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn get_config(file: &str) -> Result<ClientConfig, Box<dyn std::error::Error>> {

    let mut kafka_config = ClientConfig::new();

    let file = File::open(file)?;
    for line in BufReader::new(&file).lines() {
        let cur_line: String = line?.trim().to_string();
        if ! (cur_line.starts_with('#') || cur_line.len() < 1) {
            let key_value: Vec<&str> = cur_line.split("=").collect();
            kafka_config.set(
               key_value.get(0).ok_or("malformed key")?.to_string(),
                key_value.get(1).ok_or("malformed value")?.to_string(),
            );
        }
    }

    Ok(kafka_config)
}

pub fn config_access(filename: &str) -> ClientConfig {

        let config = get_config(filename).ok().expect("Failed to open your config file");

        config
}
