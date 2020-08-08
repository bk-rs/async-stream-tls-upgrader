use std::io;

pub(crate) use async_native_tls::TlsStream;
pub use async_native_tls::{Certificate, Identity, Protocol, TlsConnector};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::{TlsClientUpgrader, Upgrader, UpgraderExtRefer};

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

impl<S> UpgraderExtRefer<S> for AsyncNativeTlsClientTlsUpgrader
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
