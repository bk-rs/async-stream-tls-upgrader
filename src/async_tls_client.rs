use std::io;

use async_tls::{client::TlsStream, TlsConnector};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::tls::TlsClientUpgrader;
use async_stream_packed::upgradable::Upgrader;

pub struct AsyncTlsClientTlsUpgrader {
    connector: TlsConnector,
    domain: String,
}

impl AsyncTlsClientTlsUpgrader {
    pub fn new(connector: TlsConnector, domain: String) -> Self {
        Self { connector, domain }
    }
}

#[async_trait]
impl<S> TlsClientUpgrader<S> for AsyncTlsClientTlsUpgrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

#[async_trait]
impl<S> Upgrader<S> for AsyncTlsClientTlsUpgrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = TlsStream<S>;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        let stream = self.connector.connect(&self.domain, stream).await?;
        Ok(stream)
    }
}
