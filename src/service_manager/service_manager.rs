use crate::provider_services::providers::{PostMessage, ProviderType};
use crate::receiver_service::receive::{self, DestinationInfo, ReceiveService};
use crate::sender_service::sender::{self, SenderService};
use crate::ChainConfig;
use alloy::primitives::{address, Address};
use std::collections::HashMap;
use tokio::sync::mpsc::{self, Sender};

#[derive(Debug)]
pub enum ChainType {
    Ethereum,
    BinanceSmartChain,
    Amoy,
}
pub struct EventData {
    pub chain_type: ChainType,
    pub post_message: PostMessage,
}
impl EventData {
    pub fn new(chain_type: ChainType, post_message: PostMessage) -> Self {
        Self {
            chain_type,
            post_message,
        }
    }
}
pub trait Service {
    async fn run(self); // Added `&self` to make it callable on instances of a struct
}
pub struct ServiceManager {
    senders: Vec<SenderService>,
    receivers: Vec<ReceiveService>,
}

impl ServiceManager {
    pub async fn new(config: ChainConfig) -> Self {
        // let (sender, receiver) = mpsc::channel::<PostMessage>(100);

        let (senders, receivers) = Self::create_multiple_sender_receiver_services(
            vec![&config.source_rpc_url, &config.dest_rpc_url],
            vec![config.source_contarct_addr, config.dest_contarct_addr],
            vec![config.source_event, config.dest_event],
        )
        .await;
        Self { senders, receivers }
    }
    //first one is source , second one is destination
    async fn create_multiple_sender_receiver_services(
        rpc_url: Vec<&str>,
        contarct_addr: Vec<Address>,
        event: Vec<String>,
    ) -> (Vec<SenderService>, Vec<ReceiveService>) {
        let mut senders: Vec<SenderService> = vec![];
        let mut receivers: Vec<ReceiveService> = vec![];
        let (source_sender, source_receiver) = mpsc::channel::<EventData>(100);
        let (dest_sender, dest_receiver) = mpsc::channel::<EventData>(100);
        let source_provider_type = ProviderType::new(rpc_url[0]).await;
        let dest_provider_type = ProviderType::new(rpc_url[1]).await;
        let sender_source = SenderService::new(
            source_provider_type.clone(),
            contarct_addr[0],
            event[0].clone(),
            source_sender,
        );
        senders.push(sender_source);
        let dest_source = SenderService::new(
            dest_provider_type.clone(),
            contarct_addr[1],
            event[1].clone(),
            dest_sender,
        );
        let mut cain_to_dest: HashMap<u64, DestinationInfo> = HashMap::new();
        let mut cain_to_dest2: HashMap<u64, DestinationInfo> = HashMap::new();

        cain_to_dest.insert(
        80002,
            DestinationInfo {
                provider_type: dest_provider_type,
                contract_addr: contarct_addr[1],
            },
        );
        cain_to_dest2.insert(
            1,
            DestinationInfo {
                provider_type: source_provider_type,
                contract_addr: contarct_addr[0],
            },
        );
        senders.push(dest_source);
        let receiver_source = ReceiveService::new(cain_to_dest, source_receiver);
        receivers.push(receiver_source);
        let receiver_dest = ReceiveService::new(cain_to_dest2, dest_receiver);
        receivers.push(receiver_dest);

        return (senders, receivers);
    }
}
impl Service for ServiceManager {
    async fn run(self) {
        let mut handles = vec![];
        for sender in self.senders {
            handles.push(tokio::spawn(async move { sender.run().await }));
        }
        for receiver in self.receivers {
            handles.push(tokio::spawn(async move { receiver.run().await }));
        }

        for handle in handles {
            let x = handle.await.unwrap();
        }
    }
}
