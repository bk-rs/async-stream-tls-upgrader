/*
cargo run -p async-stream-tls-upgrader-demo-smol --bin async_tls_client httpbin.org 443 /ip
*/

use std::env;
use std::io;
use std::str;

use async_net::TcpStream;
use async_stream_tls_upgrader::{AsyncTlsClientTlsUpgrader, UpgradableAsyncStream};
use futures_lite::future::block_on;
use futures_lite::{AsyncReadExt, AsyncWriteExt};

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("httpbin.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("443".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/ip".to_owned()));

    println!("{} {} {}", domain, port, uri);

    //
    let addr = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(addr).await?;
    let upgrader = AsyncTlsClientTlsUpgrader::new(Default::default(), domain.clone());
    let mut stream = UpgradableAsyncStream::new(stream, upgrader);

    stream.upgrade().await?;

    let req_string = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n",
        uri, domain
    );
    println!("{}", req_string);

    stream.write(&req_string.as_bytes()).await?;

    let mut buf = vec![0u8; 288];
    stream.read(&mut buf).await?;

    println!("{:?}", str::from_utf8(&buf));

    println!("done");

    Ok(())
}
