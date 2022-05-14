use serde::Deserialize;
use std::error::Error;
use std::path::PathBuf;
use std::{collections::HashMap, fs};

// main config struct
pub struct Config {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

// toml representation (main)
#[derive(Deserialize)]
pub struct ConfigToml {
    pub port: u16,
    pub host: Vec<HostToml>,
}

// toml representation (for hosts)
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
            hosts.insert(from, to.to_string());
        }
    }
    hosts
}

// get config struct
pub fn parse(file: PathBuf) -> Result<Config, Box<dyn Error>> {
    // load config from file
    let buf = fs::read_to_string(file)?;

    // parse file contents
    let config: ConfigToml = toml::from_str(&buf)?;

    // create and return main config struct
    Ok(Config {
        port: config.port,
        hosts: parsehosts(config),
    })
}
