#[cfg(feature = "unionable_client")]
#[cfg(test)]
mod unionable_client_tests {
    #[allow(unused_imports)]
    use async_stream_tls_upgrader::UnionableTlsClientUpgrader;
}

#[cfg(feature = "async_tls_client")]
#[cfg(test)]
mod unionable_client_with_async_tls_client_tests {
    use std::io;

    use async_stream_packed::Upgrader;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;

    use async_stream_tls_upgrader::{AsyncTlsClientTlsUpgrader, UnionableTlsClientUpgrader};

    #[test]
    fn upgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"".to_vec());

            let mut upgrader = UnionableTlsClientUpgrader::AsyncTls(
                AsyncTlsClientTlsUpgrader::new(Default::default(), "foo.example.com".to_owned()),
            );

            let err = upgrader.upgrade(cursor).await.err().unwrap();

            assert_eq!(err.kind(), io::ErrorKind::UnexpectedEof);
            assert_eq!(err.to_string(), "tls handshake eof");

            Ok(())
        })
    }
}

#[cfg(feature = "async_native_tls_client")]
#[cfg(test)]
mod unionable_client_with_async_native_tls_client_tests {
    use std::io;

    use async_stream_packed::Upgrader;
    use futures_lite::future::block_on;
    use futures_lite::io::Cursor;

    use async_stream_tls_upgrader::{AsyncNativeTlsClientTlsUpgrader, UnionableTlsClientUpgrader};

    #[test]
    fn upgrade() -> io::Result<()> {
        block_on(async {
            let cursor = Cursor::new(b"".to_vec());

            let mut upgrader =
                UnionableTlsClientUpgrader::AsyncNativeTls(AsyncNativeTlsClientTlsUpgrader::new(
                    Default::default(),
                    "foo.example.com".to_owned(),
                ));

            let err = upgrader.upgrade(cursor).await.err().unwrap();

            assert_eq!(err.kind(), io::ErrorKind::Other);
            assert_eq!(err.to_string(), "unexpected EOF");

            Ok(())
        })
    }
}
