use crate::provider_services::providers::{ProviderType, Transfer};
use crate::receiver_service::receive::ReceiveService;
use crate::sender_service::sender::SenderService;
use crate::ChainConfig;
use tokio::sync::mpsc;

pub trait Service {
    async fn run(self); // Added `&self` to make it callable on instances of a struct
}
pub struct ServiceManager {
    sender: SenderService,
    receiver: ReceiveService,
}

impl ServiceManager {
    pub async fn new(config: ChainConfig) -> Self {
        let (sender, receiver) = mpsc::channel::<Transfer>(100);
        let provider_type = ProviderType::new(&config.source_rpc_url).await;
        let sender = SenderService::new(
            provider_type,
            config.source_contarct_addr,
            config.source_event,
            sender,
        );
        let provider_type = ProviderType::new(&config.dest_rpc_url).await;
        let receiver = ReceiveService::new(provider_type, config.dest_contarct_addr, receiver);
        Self { sender, receiver }
    }
}
impl Service for ServiceManager {
    async fn run(self) {
        let mut handles = vec![];

        let handle = tokio::spawn(async move { self.sender.run().await });
        let handle1 = tokio::spawn(async move { self.receiver.run().await });

        handles.push(handle);
        handles.push(handle1);

        for handle in handles {
            let x = handle.await.unwrap();
        }
    }
}
