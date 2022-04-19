use simple_logger::SimpleLogger;
use std::error::Error;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // logging setup
    SimpleLogger::new().init().unwrap();

    // server setup
    let server = server::Server { port: 80 };
    server.run().await?;
    Ok(())
}
