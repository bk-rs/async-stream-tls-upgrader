use std::io;

pub(crate) use async_tls::client::TlsStream;
pub use async_tls::TlsConnector;
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};
pub use rustls::ClientConfig;
pub use webpki_roots::TLS_SERVER_ROOTS;

use async_stream_packed::{TlsClientUpgrader, Upgrader, UpgraderExtRefer};

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

impl<S> UpgraderExtRefer<S> for AsyncTlsClientTlsUpgrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    fn get_ref(output: &Self::Output) -> &S {
        output.get_ref()
    }

    fn get_mut(output: &mut Self::Output) -> &mut S {
        output.get_mut()
    }
}
