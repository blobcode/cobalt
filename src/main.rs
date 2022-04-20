use simple_logger::SimpleLogger;
use std::{collections::HashMap, error::Error};

mod args;
mod config;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // logging setup
    SimpleLogger::new().init().unwrap();

    // get args
    let opts = args::parse();

    // get config
    let config = config::parse(opts.path);

    let mut hosts = HashMap::new();
    hosts.insert("localhost:8000".to_string(), "localhost:8080".to_string());

    // server setup
    let server = server::Server {
        port: config.port,
        hosts,
    };
    server.run().await?;
    Ok(())
}
