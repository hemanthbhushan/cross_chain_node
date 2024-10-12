mod provider_services;
use alloy::primitives::address;
use provider_services::providers;

#[tokio::main]
async fn main() {
    let rpc_url = "https://eth.llamarpc.com";
    let contract_address = address!("62d4d3785f8117Be8d2eE8e1e81C9147098bC3fF");
    let event = "Transfer";
    let x = providers::initialize_provider(rpc_url, contract_address, event).await;
    x.start_service().await;
}
