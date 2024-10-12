mod ethereum_provider;
use alloy::primitives::address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{BlockNumberOrTag, Filter};
use alloy::transports::http::HttpConnect;
use ethereum_provider::providers;
use futures::StreamExt;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let rpc_url = "https://eth.llamarpc.com";
    let contract_address = address!("62d4d3785f8117Be8d2eE8e1e81C9147098bC3fF");
    let event = "Transfer";
    println!("im here ");
    let x = providers::build_provider(rpc_url, contract_address, event).await;
    x.build_server().await;
}
