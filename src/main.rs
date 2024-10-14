mod provider_services;
mod receiver_service;
mod sender_service;
mod service_manager;
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use service_manager::service_manager::{Service, ServiceManager};
use std::{fs::File, io::BufReader, io::Read};

#[derive(Serialize, Deserialize, Clone)]
struct ChainConfig {
    source_rpc_url: String,
    dest_rpc_url: String,
    source_contarct_addr: Address,
    dest_contarct_addr: Address,
    source_event: String,
}
#[tokio::main]
async fn main() {
    let file = File::open("config.json").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let config: ChainConfig = serde_json::from_str(&contents).unwrap();
    let service = ServiceManager::new(config).await;
    service.run().await
}
