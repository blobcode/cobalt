use simple_logger::SimpleLogger;
use std::error::Error;

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

    // server setup
    let server = server::Server { port: config.port };
    server.run().await?;
    Ok(())
}
