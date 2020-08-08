use std::io;
use std::pin::Pin;
use std::task::{Context, Poll};

use async_trait::async_trait;
use futures_io::{AsyncRead, AsyncWrite};

use async_stream_packed::{TlsClientUpgrader, Upgrader, UpgraderExtRefer};

pub enum UnionableTlsClientUpgrader {
    #[cfg(feature = "async_tls_client")]
    AsyncTls(crate::async_tls_client::AsyncTlsClientTlsUpgrader),
    #[cfg(feature = "async_native_tls_client")]
    AsyncNativeTls(crate::async_native_tls_client::AsyncNativeTlsClientTlsUpgrader),
}

#[async_trait]
impl<S> TlsClientUpgrader<S> for UnionableTlsClientUpgrader where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static
{
}

#[async_trait]
impl<S> Upgrader<S> for UnionableTlsClientUpgrader
where
    S: AsyncRead + AsyncWrite + Unpin + Send + 'static,
{
    type Output = UnionableTlsStream<S>;
    async fn upgrade(&mut self, stream: S) -> io::Result<Self::Output> {
        match self {
            #[cfg(feature = "async_tls_client")]
            UnionableTlsClientUpgrader::AsyncTls(upgrader) => {
                let stream = upgrader.upgrade(stream).await?;
                Ok(UnionableTlsStream::AsyncTls(stream))
            }
            #[cfg(feature = "async_native_tls_client")]
            UnionableTlsClientUpgrader::AsyncNativeTls(upgrader) => {
                let stream = upgrader.upgrade(stream).await?;
                Ok(UnionableTlsStream::AsyncNativeTls(stream))
            }
        }
    }
}

impl<S> UpgraderExtRefer<S> for UnionableTlsClientUpgrader
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

//
//
//
pub enum UnionableTlsStream<S> {
    #[cfg(feature = "async_tls_client")]
    AsyncTls(crate::async_tls_client::TlsStream<S>),
    #[cfg(feature = "async_native_tls_client")]
    AsyncNativeTls(crate::async_native_tls_client::TlsStream<S>),
}

macro_rules! unionable_tls_stream {
    ($value:expr, $pattern:pat => $result:expr) => {
        match $value {
            #[cfg(feature = "async_tls_client")]
            UnionableTlsStream::AsyncTls($pattern) => $result,
            #[cfg(feature = "async_native_tls_client")]
            UnionableTlsStream::AsyncNativeTls($pattern) => $result,
        }
    };
}

impl<S> UnionableTlsStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn get_ref(&self) -> &S {
        unionable_tls_stream!(self, s => s.get_ref())
    }
    fn get_mut(&mut self) -> &mut S {
        unionable_tls_stream!(self, s => s.get_mut())
    }
}

impl<S> AsyncRead for UnionableTlsStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        unionable_tls_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_write(cx, buf))
    }
}

impl<S> AsyncWrite for UnionableTlsStream<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context, buf: &[u8]) -> Poll<io::Result<usize>> {
        unionable_tls_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_write(cx, buf))
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        unionable_tls_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_flush(cx))
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<io::Result<()>> {
        unionable_tls_stream!(self.get_mut(), ref mut s => Pin::new(s).poll_close(cx))
    }
}
