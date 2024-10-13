mod provider_services;
use alloy::primitives::address;
use futures::channel::oneshot::channel;
use provider_services::providers::{self,Transfer};
use tokio::sync::mpsc::{self, Sender};



#[tokio::main]
async fn main() {
    let rpc_url = "wss://ethereum-rpc.publicnode.com";
    let contract_address = address!("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    let event = "Transfer";
    let (sender , receiver ) = mpsc::channel::<Transfer>(100);
    let x = providers::initialize_provider(rpc_url, contract_address, event ,sender ).await;
    x.start_service().await;
}
