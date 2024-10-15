use crate::provider_services::providers::{PostMessage, ProviderType};
use crate::service_manager::service_manager::{ChainType, EventData, Service};
use alloy::primitives::{Address, Uint};
use alloy::providers::Provider;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log};
use alloy_pubsub::{PubSubFrontend, Subscription};
use futures::{stream, StreamExt};
use tokio::sync::mpsc::{self, Sender};
#[derive(Clone)]
pub struct SenderService {
    provider_type: ProviderType,
    contract_addr: Address,
    event: String,
    sender: Sender<EventData>,
}

impl SenderService {
    pub fn new(
        provider_type: ProviderType,
        contract_addr: Address,
        event: String,
        sender: Sender<EventData>,
    ) -> Self {
        Self {
            provider_type,
            contract_addr,
            event,
            sender,
        }
    }
}

impl Service for SenderService {
    async fn run(self) {
        println!("yaaaaaaaaaaaaaaaaaaaayyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy");
        let filter = Filter::new()
            .address(self.contract_addr)
            .event(&self.event)
            .from_block(BlockNumberOrTag::Latest);
        match &self.provider_type {
            ProviderType::Http(http_service) => {
                let event = http_service.provider.subscribe_logs(&filter).await.unwrap();
                self.stream_handler(event).await;
            }
            ProviderType::WebSocket(wss_service) => {
                let event = wss_service.provider.subscribe_logs(&filter).await.unwrap();
                self.stream_handler(event).await;
            }
        }
    }
}

impl SenderService {
    async fn stream_handler(&self, stream: Subscription<Log>) {
        let sender_clone = self.sender.clone();
        let x = tokio::spawn(async move {
            let mut stream = stream.into_stream();
            println!("Im hereeeeeeeee");
            while let Some(log) = stream.next().await {
                let data: PostMessage = log.log_decode::<PostMessage>().unwrap().inner.data;
                println!("{:?}", data);
                println!("Im hereeeeeeeee");
                let check = data.toChainId.into_limbs()[0];
                match check {
                    1 => {
                        sender_clone
                            .send(EventData::new(ChainType::Ethereum, data))
                            .await
                            .unwrap();
                        println!("Chain ID is 1: Ethereum Chain");
                    }
                    2 => {
                        // Handle chain ID 2 (e.g., Binance Smart Chain)
                        println!("Chain ID is 2: Binance Smart Chain");
                        sender_clone
                            .send(EventData::new(ChainType::BinanceSmartChain, data))
                            .await
                            .unwrap();
                    }

                    _ => {
                        log::info!("Unregistered chain: {}", check);
                        println!("Unregistered chain: {}", check);
                    }
                }
            }
        });
        x.await.unwrap()
    }
}
