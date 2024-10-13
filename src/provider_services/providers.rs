use alloy::sol;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use alloy_provider::RootProvider;
use alloy_pubsub::PubSubFrontend;
use alloy_transport_http::Http;
use futures::{stream, StreamExt};
use log::info;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use url::Url;
use alloy::rpc::types::{Filter,BlockNumberOrTag};
sol! {
    #[derive(Debug)]
    event Transfer(address indexed from, address indexed to, uint256 value);
}

trait ProviderService {
    fn start(&self) {}
}

pub struct HttpProviderService {
    provider: RootProvider<Http<Client>>,
}
impl ProviderService for HttpProviderService {}

pub struct WssProviderService {
    provider: RootProvider<PubSubFrontend>,
}
impl ProviderService for WssProviderService {}

pub enum ProviderType {
    Http(HttpProviderService),
    WebSocket(WssProviderService),
}

pub struct ServiceManager {
    provider_type: ProviderType,
    contract_addr: Address,
    event: String,
    sender: Sender<Transfer>,
}

pub async fn initialize_provider(
    rpc_url: &str,
    contract_addr: Address,
    event: &str,
    sender: Sender<Transfer>,
) -> ServiceManager {
    if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
        let provider = ProviderBuilder::new().on_http(Url::parse(rpc_url).unwrap());
        let http_provider_service = HttpProviderService { provider };
        info!("Using HTTP provider");
        ServiceManager {
            provider_type: ProviderType::Http(http_provider_service),
            contract_addr,
            event: event.to_string(),
            sender,
        }
    } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
        let provider = ProviderBuilder::new()
            .on_ws(WsConnect::new(rpc_url))
            .await
            .unwrap();
        let websocket_provider_service = WssProviderService { provider };
        info!("Using WebSocket provider");

        ServiceManager {
            provider_type: ProviderType::WebSocket(websocket_provider_service),
            contract_addr,
            event: event.to_string(),
            sender,
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
            let filter = Filter::new().address(self.contract_addr).event(&self.event).from_block(BlockNumberOrTag::Latest);
            

             let mut event = http_service.provider.subscribe_logs(&filter).await.unwrap();
            //    println!("eventtt {:?}",event.recv().await);
            }
            ProviderType::WebSocket(wss_service) => {
                let filter = Filter::new().address(self.contract_addr).event(&self.event).from_block(BlockNumberOrTag::Latest);
            

                let mut event = wss_service.provider.subscribe_logs(&filter).await.unwrap();
                //   println!("eventtt {:?}",event.recv().await);
                let mut stream = event.into_stream();
                println!("im here");
                while let Some(x) = stream.next().await  {
                    println!("{:?}",x.data())
                    
                }  
            }
        }
    }
}
