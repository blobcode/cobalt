use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

pub struct Server {
    pub port: u16,
}

impl Server {
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listen_addr = "127.0.0.1:".to_string() + &self.port.to_string();

        log::info!("Listening on: {}", listen_addr);

        let listener = TcpListener::bind(listen_addr).await?;

        while let Ok((inbound, _)) = listener.accept().await {
            let handler = handle(inbound).map(|r| {
                if let Err(e) = r {
                    log::error!("Failed to proxy; error={}", e);
                }
            });

            tokio::spawn(handler);
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

async fn handle(inbound: TcpStream) -> Result<(), Box<dyn Error>> {
    let server_addr = "127.0.0.1:8080".to_string();

    let transfer = proxy(inbound, server_addr.clone()).map(|r| {
        if let Err(e) = r {
            log::error!("Failed to proxy; error={}", e);
        }
    });

    tokio::spawn(transfer);

    Ok(())
}

fn gethost() -> Result<(), Box<dyn Error>> {
    let mut headers = [httparse::EMPTY_HEADER; 64];
    let mut req = httparse::Request::new(&mut headers);

    let buf = b"GET /index.html HTTP/1.1\r\nHost";
    assert!(req.parse(buf)?.is_partial());

    // a partial request, so we try again once we have more data

    let buf = b"GET /index.html HTTP/1.1\r\nHost: example.domain\r\n\r\n";
    assert!(req.parse(buf)?.is_complete());
    Ok(())
}
