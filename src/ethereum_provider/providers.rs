use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use alloy_provider::RootProvider;
use alloy_pubsub::PubSubFrontend;
use alloy_transport_http::Http;
use log::info;
use reqwest::Client;
use url::Url;

trait Service {
    fn run(&self) {}
}
struct HttpProvider {
    provider: RootProvider<Http<Client>>,
}
impl Service for HttpProvider {}

struct WssProvider {
    provider: RootProvider<PubSubFrontend>,
}
impl Service for WssProvider {}

pub enum Providers {
    HttpProvider(HttpProvider),
    WssProvider(WssProvider),
}
pub struct Services {
    provider: Providers,
}

pub async fn build_provider(rpc_url: &str, contract_address: Address, event: &str) -> Services {
    if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
        let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url).unwrap());
        let http_provider = HttpProvider { provider };
        info!("in the Http provider");
        Services {
            provider: Providers::HttpProvider(http_provider),
        }
    } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
        let provider = ProviderBuilder::new()
            .on_ws(WsConnect::new(rpc_url))
            .await
            .unwrap();
        let wss_provider = WssProvider { provider };
        info!("in the Wss provider");

        Services {
            provider: Providers::WssProvider(wss_provider),
        }
    } else {
        println!("invalid Rpc Url");
        panic!("Invalid Rpc Url");
    }
}

impl Services {
    pub async fn build_server(&self) {
        match &self.provider {
            Providers::HttpProvider(x) => {
                println!(
                    "in the Http {}",
                    x.provider.get_block_number().await.unwrap()
                )
            }
            Providers::WssProvider(x) => {
                println!(
                    "in the wss {}",
                    x.provider.get_block_number().await.unwrap()
                )
            }
        }
    }
}
