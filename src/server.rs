use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

pub struct Server {
    pub port: u32,
}

impl Server {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listen_addr = "127.0.0.1:8081".to_string();
        let server_addr = "127.0.0.1:8080".to_string();

        log::info!("Listening on: {}", listen_addr);
        log::info!("Proxying to: {}", server_addr);

        let listener = TcpListener::bind(listen_addr).await?;

        while let Ok((inbound, _)) = listener.accept().await {
            let transfer = proxy(inbound, server_addr.clone()).map(|r| {
                if let Err(e) = r {
                    log::error!("Failed to proxy; error={}", e);
                }
            });

            tokio::spawn(transfer);
        }
        Ok(())
    }
}

async fn proxy(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    let (mut ri, mut wi) = inbound.split();
    let (mut ro, mut wo) = outbound.split();

    let client_to_server = async {
        io::copy(&mut ri, &mut wo).await?;
        wo.shutdown().await
    };

    let server_to_client = async {
        io::copy(&mut ro, &mut wi).await?;
        wi.shutdown().await
    };

    tokio::try_join!(client_to_server, server_to_client)?;

    Ok(())
}
