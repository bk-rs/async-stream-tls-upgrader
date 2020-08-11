/*
cargo run -p async-stream-tls-upgrader-demo-async-net --bin async_native_tls_client echo.websocket.org 443 /
*/

use std::env;
use std::io;
use std::str;

use async_net::TcpStream;
use async_stream_packed::UpgradableAsyncStream;
use futures_lite::future::block_on;
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use async_stream_tls_upgrader::AsyncNativeTlsClientTlsUpgrader;

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("echo.websocket.org".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("443".to_owned()))
        .parse()
        .unwrap();
    let uri = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("URI").unwrap_or("/".to_owned()));

    println!("async_native_tls_client {} {} {}", domain, port, uri);

    //
    let addr = format!("{}:{}", domain, port);
    let stream = TcpStream::connect(addr).await?;
    let upgrader = AsyncNativeTlsClientTlsUpgrader::new(Default::default(), domain.clone());
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

    let _ = stream.get_ref();
    let _ = stream.get_mut();

    println!("done");

    Ok(())
}
