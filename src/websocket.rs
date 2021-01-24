use actix::io::SinkWrite;
use actix_codec::Framed;
use awc::{
    ws::{Codec, Message},
    BoxedSocket,
};
use futures::stream::SplitSink;
use std::sync::Arc;
use std::time::Duration;

pub fn new_websocket_client() -> awc::Client {
    let mut cfg = rustls::ClientConfig::new();
    cfg.alpn_protocols = vec![b"http/1.1".to_vec()];
    cfg.root_store
        .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);
    let connector = awc::Connector::new()
        .rustls(Arc::new(cfg))
        // cf. https://docs.rs/awc/2.0.0-beta.1/awc/struct.Connector.html#method.ssl
        .timeout(Duration::from_secs(5))
        .finish();
    awc::ClientBuilder::new().connector(connector).finish()
}

pub type Sender = SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>;
