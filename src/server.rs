use std::collections::HashMap;
use tokio::io::AsyncWriteExt;
use tokio::io::{self};
use tokio::net::{TcpListener, TcpStream};

use futures::FutureExt;
use std::error::Error;

pub struct Server {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

impl Server {
    // start server
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        let listen_addr = "127.0.0.1:".to_string() + &self.port.to_string();

        log::info!("Listening on: {}", listen_addr);

        let listener = TcpListener::bind(listen_addr).await?;

        // listener loop that passes off to handler
        while let Ok((inbound, _)) = listener.accept().await {
            let handler = handle(inbound, self.hosts.clone()).map(|r| {
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
async fn handle(inbound: TcpStream, hosts: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    // buffer init
    let mut buf = vec![0; 512];
    let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut r = httparse::Request::new(&mut headers);

    // parse request
    inbound.peek(&mut buf).await?;
    r.parse(&buf)?;

    // parse headers
    let p = headers.iter().position(|&h| h.name == "Host").unwrap();
    let host = String::from_utf8_lossy(headers[p].value).to_string();
    let to = hosts.get(&host).unwrap();

    // proxy
    let proxy = proxy(inbound, to.to_string()).map(|r| {
        if let Err(e) = r {
            log::error!("Failed to proxy; error={}", e);
        }
    });

    tokio::spawn(proxy);

    Ok(())
}

// proxy tcpstream
async fn proxy(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    // connect to server
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    // split and merge streams
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
