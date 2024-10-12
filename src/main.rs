mod ethereum_provider;
use std::str::FromStr;
use alloy::primitives::address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::transports::http::HttpConnect;
use alloy::rpc::types::{BlockNumberOrTag, Filter};
use futures::StreamExt;
use ethereum_provider::providers;


#[tokio::main]
async fn main()  {
    let rpc_url = "";
    let contract_address = address!("0");
    let event = "Transfer";



}
