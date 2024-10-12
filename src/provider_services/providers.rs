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

trait ProviderService {
    fn start(&self) {}
}

struct HttpProviderService {
    provider: RootProvider<Http<Client>>,
}
impl ProviderService for HttpProviderService {}

struct WebSocketProviderService {
    provider: RootProvider<PubSubFrontend>,
}
impl ProviderService for WebSocketProviderService {}

pub enum ProviderType {
    Http(HttpProviderService),
    WebSocket(WebSocketProviderService),
}

pub struct ServiceManager {
    provider_type: ProviderType,
}

pub async fn initialize_provider(rpc_url: &str, contract_address: Address, event: &str) -> ServiceManager {
    if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
        let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url).unwrap());
        let http_provider_service = HttpProviderService { provider };
        info!("Using HTTP provider");
        ServiceManager {
            provider_type: ProviderType::Http(http_provider_service),
        }
    } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
        let provider = ProviderBuilder::new()
            .on_ws(WsConnect::new(rpc_url))
            .await
            .unwrap();
        let websocket_provider_service = WebSocketProviderService { provider };
        info!("Using WebSocket provider");

        ServiceManager {
            provider_type: ProviderType::WebSocket(websocket_provider_service),
        }
    } else {
        println!("Invalid RPC URL");
        panic!("Invalid RPC URL");
    }
}

impl ServiceManager {
    pub async fn start_service(&self) {
        match &self.provider_type {
            ProviderType::Http(http_service) => {
                println!(
                    "Using HTTP: Block number is {}",
                    http_service.provider.get_block_number().await.unwrap()
                )
            }
            ProviderType::WebSocket(websocket_service) => {
                println!(
                    "Using WebSocket: Block number is {}",
                    websocket_service.provider.get_block_number().await.unwrap()
                )
            }
        }
    }
}