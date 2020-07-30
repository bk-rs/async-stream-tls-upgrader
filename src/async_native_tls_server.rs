use std::io;

use async_native_tls::{TlsAcceptor, TlsStream};
use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use crate::{TlsServerUpgrader, Upgrader, UpgraderExtRefer};

pub struct AsyncNativeTlsServerTlsUpgrader {
    acceptor: TlsAcceptor,
}

impl AsyncNativeTlsServerTlsUpgrader {
    pub fn new(acceptor: TlsAcceptor) -> Self {
        Self { acceptor }
    }
}

#[async_trait]
impl<S> TlsServerUpgrader<S> for AsyncNativeTlsServerTlsUpgrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

#[async_trait]
impl<S> Upgrader<S> for AsyncNativeTlsServerTlsUpgrader
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

impl<S> UpgraderExtRefer<S> for AsyncNativeTlsServerTlsUpgrader
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
