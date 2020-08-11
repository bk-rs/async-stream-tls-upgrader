/*
cargo run -p async-stream-tls-upgrader-demo-async-net --bin smtp_client smtp.gmail.com 465 xxx@gmail.com '123456'

cargo run -p async-stream-tls-upgrader-demo-async-net --bin smtp_client smtp.gmail.com 587 xxx@gmail.com '123456'
*/

use std::env;
use std::io;
use std::str;

use async_net::TcpStream;
use async_stream_tls_upgrader::AsyncTlsClientTlsUpgrader;
use futures_lite::future::block_on;
use futures_lite::{AsyncReadExt, AsyncWriteExt};

use async_stream_packed::SmtpClientInnerStream;

fn main() -> io::Result<()> {
    block_on(run())
}

async fn run() -> io::Result<()> {
    let domain = env::args()
        .nth(1)
        .unwrap_or_else(|| env::var("DOMAIN").unwrap_or("imap.gmail.com".to_owned()));
    let port: u16 = env::args()
        .nth(2)
        .unwrap_or_else(|| env::var("PORT").unwrap_or("993".to_owned()))
        .parse()
        .unwrap();
    let username = env::args()
        .nth(3)
        .unwrap_or_else(|| env::var("USERNAME").unwrap_or_else(|_| "xxx@gmail.com".to_owned()));
    let password = env::args()
        .nth(4)
        .unwrap_or_else(|| env::var("PASSWORD").unwrap_or_else(|_| "123456".to_owned()));

    //
    let addr = format!("{}:{}", domain, port);

    println!("connect");
    let tcp_stream = TcpStream::connect(addr).await?;
    let tls_upgrader = AsyncTlsClientTlsUpgrader::new(Default::default(), domain);

    let mut stream = SmtpClientInnerStream::with_imap_client(tcp_stream, tls_upgrader);

    let mut buf = vec![0u8; 256];

    if format!("{}", port).ends_with("465") {
        println!("upgrade");
        stream.upgrade().await?;

        let n = stream.read(&mut buf).await?;
        println!("greeting: {:?}", str::from_utf8(&buf[..n]));

        stream.write(format!("EHLO RUST\r\n").as_bytes()).await?;
        let n = stream.read(&mut buf).await?;
        println!("EHLO res: {:?}", str::from_utf8(&buf[..n]));
    } else {
        let n = stream.read(&mut buf).await?;
        println!("greeting: {:?}", str::from_utf8(&buf[..n]));

        stream.write(format!("EHLO RUST\r\n").as_bytes()).await?;
        let n = stream.read(&mut buf).await?;
        println!("EHLO res: {:?}", str::from_utf8(&buf[..n]));

        stream.write(format!("STARTTLS\r\n").as_bytes()).await?;
        let n = stream.read(&mut buf).await?;
        println!("STARTTLS res: {:?}", str::from_utf8(&buf[..n]));

        println!("upgrade");
        stream.upgrade().await?;

        stream.write(format!("EHLO RUST\r\n").as_bytes()).await?;
        let n = stream.read(&mut buf).await?;
        println!("EHLO res: {:?}", str::from_utf8(&buf[..n]));
    }

    stream.write(format!("AUTH LOGIN\r\n").as_bytes()).await?;
    let n = stream.read(&mut buf).await?;
    println!("AUTH LOGIN res: {:?}", str::from_utf8(&buf[..n]));

    stream
        .write(format!("{}\r\n", base64::encode(username)).as_bytes())
        .await?;
    let n = stream.read(&mut buf).await?;
    println!("AUTH LOGIN username res: {:?}", str::from_utf8(&buf[..n]));

    stream
        .write(format!("{}\r\n", base64::encode(password)).as_bytes())
        .await?;
    let n = stream.read(&mut buf).await?;
    println!("AUTH LOGIN password res: {:?}", str::from_utf8(&buf[..n]));

    println!("done");

    Ok(())
}
