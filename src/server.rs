use std::collections::HashMap;
use tokio::io;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::task;

use futures::FutureExt;
use std::error::Error;

// main server state
pub struct Server {
    pub port: u16,
    pub hosts: HashMap<String, String>,
}

impl Server {
    // start server
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
        // setup address
        let listen_addr = "localhost:".to_string() + &self.port.to_string();

        log::info!("cobalt started");
        let listener = TcpListener::bind(&listen_addr).await?;
        log::info!("listening on: http://{}", listen_addr);

        // listener loop that passes off to handler
        while let Ok((inbound, _)) = listener.accept().await {
            let handler = handle(inbound, self.hosts.clone()).map(|r| {
                if let Err(e) = r {
                    log::error!("failed to handle request; {}", e);
                }
            });

            // create thread for handler
            tokio::spawn(handler);
        }
        Ok(())
    }
}

// request handler
async fn handle(inbound: TcpStream, hosts: HashMap<String, String>) -> Result<(), Box<dyn Error>> {
    // buffer init
    let mut buf = vec![0; 256];
    let mut headers = [httparse::EMPTY_HEADER; 16];

    // peek into buffer and parse request
    inbound.peek(&mut buf).await?;

    let b = buf.leak();

    // do buffer parsing
    let host = task::spawn_blocking(move || {
        let mut r = httparse::Request::new(&mut headers);
        r.parse(b).unwrap();
        // try to parse headers
        let p = headers.iter().position(|&h| h.name == "Host").unwrap();
        let host = String::from_utf8_lossy(headers[p].value).to_string();
        host
    })
    .await?;

    // lookup
    let to = &hosts[&host];

    // spawn proxy
    let proxy = proxy(inbound, to.to_string()).map(|r| {
        if let Err(e) = r {
            log::error!("failed to proxy; {}", e);
        }
    });

    // spawn thread for proxy
    tokio::spawn(proxy);

    Ok(())
}

// proxy tcpstreams
async fn proxy(mut inbound: TcpStream, proxy_addr: String) -> Result<(), Box<dyn Error>> {
    // open stream
    let mut outbound = TcpStream::connect(proxy_addr).await?;

    // split, swap and merge streams
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
