use alloy::sol;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use alloy_provider::RootProvider;
use alloy_pubsub::{PubSubFrontend, Subscription};

use alloy_transport_http::Http;
use futures::{stream, StreamExt};
use log::info;
use reqwest::Client;
use std::fmt::Debug;
use tokio::sync::mpsc::Sender;
use url::Url;
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

trait ProviderService {
    fn start(&self) {}
}
#[derive(Clone)]
pub struct HttpProviderService {
    pub provider: RootProvider<Http<Client>>,
}
impl ProviderService for HttpProviderService {}
#[derive(Clone)]
pub struct WssProviderService {
    pub provider: RootProvider<PubSubFrontend>,
}
impl ProviderService for WssProviderService {}
#[derive(Clone)]
pub enum ProviderType {
    Http(HttpProviderService),
    WebSocket(WssProviderService),
}
impl ProviderType {
    pub async fn new(rpc_url: &str) -> Self {
        if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
            let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url).unwrap());
            info!("Using HTTP provider");
            return Self::Http(HttpProviderService { provider });
        } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
            let provider = ProviderBuilder::new()
                .on_ws(WsConnect::new(rpc_url))
                .await
                .unwrap();

            info!("Using WebSocket provider");
            return Self::WebSocket(WssProviderService { provider });
        } else {
            println!("Invalid RPC URL");
            panic!("Invalid RPC URL");
        }
    }
}
