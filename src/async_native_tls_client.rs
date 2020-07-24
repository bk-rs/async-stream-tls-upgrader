use std::io;

use async_native_tls::{TlsConnector, TlsStream};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::tls::TlsClientUpgrader;
use async_stream_packed::upgradable::Upgrader;

pub struct AsyncNativeTlsClientTlsUpgrader {
    connector: TlsConnector,
    domain: String,
}

impl AsyncNativeTlsClientTlsUpgrader {
    pub fn new(connector: TlsConnector, domain: String) -> Self {
        Self { connector, domain }
    }
}

#[async_trait]
impl<S> TlsClientUpgrader<S> for AsyncNativeTlsClientTlsUpgrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

#[async_trait]
impl<S> Upgrader<S> for AsyncNativeTlsClientTlsUpgrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = TlsStream<S>;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        let stream = self
            .connector
            .connect(&self.domain, stream)
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(stream)
    }
}
