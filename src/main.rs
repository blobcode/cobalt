use std::error::Error;

mod args;
mod config;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // logging setup
    simple_logger::init_with_level(log::Level::Info).unwrap();

    // get args
    let opts = args::parse();

    // get config
    let config = config::parse(opts.path);

    // server setup
    let server = server::Server {
        port: config.port,
        hosts: config.hosts,
    };

    // run server
    server.run().await?;

    Ok(())
}
