use std::error::Error;

mod args;
mod config;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // logging setup - log at info level
    simple_logger::init_with_level(log::Level::Info)?;

    // get args
    let opts = args::parse();

    // get config
    let config = config::parse(opts.path)?;

    // assert valid config
    log::info!("config valid");

    // server setup
    let server = server::Server {
        port: config.port,
        hosts: config.hosts,
    };

    // run server
    server.run().await?;

    Ok(())
}
