use std::io;

use async_native_tls::{TlsAcceptor, TlsStream};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::tls::TlsServerUpgrader;
use async_stream_packed::upgradable::Upgrader;

pub struct AsyncTlsServerTlsUpgrader {
    acceptor: TlsAcceptor,
}

impl AsyncTlsServerTlsUpgrader {
    pub fn new(acceptor: TlsAcceptor) -> Self {
        Self { acceptor }
    }
}

#[async_trait]
impl<S> TlsServerUpgrader<S> for AsyncTlsServerTlsUpgrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

#[async_trait]
impl<S> Upgrader<S> for AsyncTlsServerTlsUpgrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = TlsStream<S>;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        let stream = self
            .acceptor
            .accept(stream)
            .await
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        Ok(stream)
    }
}
