use crate::{
    abi::IWETH9,
    provider_services::providers::{HttpFillers, ProviderType, WssFillers},
    service_manager::service_manager::{ChainType, EventData, Service},
};

use alloy::{
    network::NetworkWallet,
    primitives::{Address, TxHash, Uint},
    rpc::types::TransactionRequest,
};

use alloy_provider::{Provider, WalletProvider};
use alloy_sol_types::{sol, SolEvent, SolValue};
use std::cmp::Ordering;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

// use alloy::signers::
sol! {
    struct Data{
        address contactAddress; // The contact address
        uint256 fromChainId;          // The source chain ID
        uint256 toChainId;             // The destination chain ID
        uint256 unitValue;

    }
}
pub struct DestinationInfo {
    pub provider_type: ProviderType, //Destination
    pub contract_addr: Address,      //Destination
}

pub struct ReceiveService {
    chain_to_destination_info: HashMap<u64, DestinationInfo>,

    receiver: Receiver<EventData>, //source
}

impl ReceiveService {
    pub fn new(
        chain_to_destination_info: HashMap<u64, DestinationInfo>,
        receiver: Receiver<EventData>,
    ) -> Self {
        Self {
            chain_to_destination_info,
            receiver,
        }
    }
}

impl Service for ReceiveService {
    async fn run(mut self) {
        println!("herrrrr we goooo ============================");

        while let Some(x) = self.receiver.recv().await {
            match x.chain_type {
                ChainType::Ethereum => {
                    Self::destination_call(self.chain_to_destination_info.get(&1).unwrap()).await;
                    println!("in the Equal");
                    let data = Data {
                        contactAddress: x.post_message.contactAddress,
                        fromChainId: x.post_message.fromChainId,
                        toChainId: x.post_message.toChainId,
                        unitValue: x.post_message.unitValue,
                    };
                    let x = data.abi_encode().tokenize().0;
                }
                ChainType::BinanceSmartChain => {
                    Self::destination_call(self.chain_to_destination_info.get(&2).unwrap()).await;
                }
                ChainType::Amoy =>{
                    println!("Im in the AMoy");
                    Self::destination_call(self.chain_to_destination_info.get(&80002).unwrap()).await;

                }
            }
        }
    }
}

impl ReceiveService {
    // async fn create_contarct_instance(contract_addr: Address, provider: ProviderType) {
    //     let contract = IWETH9::new(contract_addr, provider.);
    // }
    async fn destination_call(dest_info: &DestinationInfo) {
        match &dest_info.provider_type {
            ProviderType::Http(http_service) => {
                Self::http_contract_handler(dest_info.contract_addr, http_service.provider.clone())
                    .await;
            }
            ProviderType::WebSocket(wss_provider) => {
                Self::wss_contract_handler(dest_info.contract_addr, wss_provider.provider.clone())
                    .await;
            }
        }
    }
    async fn http_contract_handler(contract_addr: Address, provider_http: HttpFillers) {
        let contract = IWETH9::new(contract_addr, provider_http);

        let tx_hash = contract
            .mapp(Uint::from(2))
            .send()
            .await
            .unwrap()
            .with_required_confirmations(2)
            .get_receipt()
            .await
            .unwrap();
        println!("transacrtion  hash {:?}", tx_hash.transaction_hash);
    }
    async fn wss_contract_handler(contract_addr: Address, provider_http: WssFillers) {
        let contract = IWETH9::new(contract_addr, provider_http);

        let tx_hash = contract
            .mapp(Uint::from(2))
            .send()
            .await
            .unwrap()
            .with_required_confirmations(2)
            .get_receipt()
            .await
            .unwrap();
        println!("transacrtion  hash {:?}", tx_hash.transaction_hash);
    }
    async fn generate_signatures() {}
}
