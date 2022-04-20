use tokio::io::AsyncWriteExt;
use tokio::io::{self, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

pub struct Server {
    pub port: u16,
}

impl Server {
    // start server
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listen_addr = "127.0.0.1:".to_string() + &self.port.to_string();

        log::info!("Listening on: {}", listen_addr);

        let listener = TcpListener::bind(listen_addr).await?;

        // listener loop that passes off to handler
        while let Ok((inbound, _)) = listener.accept().await {
            let handler = handle(inbound).map(|r| {
                if let Err(e) = r {
                    log::error!("Failed to handle request; error={}", e);
                }
            });

            tokio::spawn(handler);
        }
        Ok(())
    }
}

// request handler
async fn handle(mut inbound: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = vec![0; 1024];
    inbound.read(&mut buf).await.unwrap();
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut r = httparse::Request::new(&mut headers);

    r.parse(&buf).unwrap();

    let p = headers.iter().position(|&h| h.name == "Host").unwrap();
    let host = String::from_utf8_lossy(headers[p].value);

    log::info!("{}", host);

    let proxy = proxy(inbound, host.to_string()).map(|r| {
        if let Err(e) = r {
            log::error!("Failed to proxy; error={}", e);
        }
    });

    tokio::spawn(proxy);

    Ok(())
}

// proxy a tcp stream
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
