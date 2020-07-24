pub use async_stream_packed::{
    tls::{TlsClientUpgrader, TlsServerUpgrader},
    upgradable::Upgrader,
    UpgradableAsyncStream,
};

#[cfg(feature = "async_tls_client")]
pub mod async_tls_client;
#[cfg(feature = "async_tls_client")]
pub use async_tls_client::AsyncTlsClientTlsUpgrader;

#[cfg(feature = "async_tls_server")]
pub mod async_tls_server;
#[cfg(feature = "async_tls_server")]
pub use async_tls_server::AsyncTlsServerTlsUpgrader;

#[cfg(feature = "async_native_tls_client")]
pub mod async_native_tls_client;
#[cfg(feature = "async_native_tls_client")]
pub use async_native_tls_client::AsyncNativeTlsClientTlsUpgrader;

#[cfg(feature = "async_native_tls_server")]
pub mod async_native_tls_server;
#[cfg(feature = "async_native_tls_server")]
pub use async_native_tls_server::AsyncNativeTlsServerTlsUpgrader;
