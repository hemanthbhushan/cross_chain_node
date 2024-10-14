use crate::provider_services::providers::{ProviderType, Transfer};
use crate::service_manager::service_manager::Service;
use alloy::primitives::Address;
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
    sender: Sender<Transfer>,
}

impl SenderService {
    pub fn new(
        provider_type: ProviderType,
        contract_addr: Address,
        event: String,
        sender: Sender<Transfer>,
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
        let filter = Filter::new()
            .address(self.contract_addr)
            .event(&self.event)
            .from_block(BlockNumberOrTag::Number(20963632));
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
        println!("Im here");

        let x = tokio::spawn(async move {
            let mut stream = stream.into_stream();

            while let Some(log) = stream.next().await {
                let data = log.log_decode::<Transfer>().unwrap().inner.data;
                println!("{:?}", data);
                sender_clone.send(data).await.unwrap();
            }
        });
        x.await.unwrap()
    }
}
