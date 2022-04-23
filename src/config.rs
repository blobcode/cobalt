use serde::Deserialize;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

// main config struct
pub struct Config {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

// toml representation
#[derive(Deserialize)]
pub struct ConfigToml {
    pub port: u16,
    pub host: Vec<HostToml>,
}

// toml representation
#[derive(Deserialize)]
pub struct HostToml {
    pub from: Vec<String>,
    pub to: String,
}

// parse config file
fn parsehosts(config: ConfigToml) -> HashMap<String, String> {
    // create standin hashmap
    let mut hosts = HashMap::new();

    // unwrap fields into the hashmap
    for host in config.host {
        for from in host.from {
            let to = &host.to;
            hosts.insert(from.to_string(), to.to_string());
        }
    }
    hosts
}

// main function to get config struct
pub fn parse(file: PathBuf) -> Config {
    // load config
    let buf = fs::read_to_string(file).unwrap();

    // parse file contents
    let config: ConfigToml = toml::from_str(&buf).unwrap();

    // create main config struct
    Config {
        port: config.port,
        hosts: parsehosts(config),
    }
}
