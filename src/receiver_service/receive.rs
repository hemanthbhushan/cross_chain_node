use crate::{
    abi::IWETH9,
    provider_services::providers::ProviderType,
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

pub struct ReceiveService {
    provider_type: ProviderType,
    contract_addr: Address,
    receiver: Receiver<EventData>,
}

impl ReceiveService {
    pub fn new(
        provider_type: ProviderType,
        contract_addr: Address,
        receiver: Receiver<EventData>,
    ) -> Self {
        Self {
            provider_type,
            contract_addr,
            receiver,
        }
    }
}

impl Service for ReceiveService {
    async fn run(mut self) {
        println!("herrrrr we goooo ============================");

        match &mut self.provider_type {
            ProviderType::Http(http_service) => {
                while let Some(x) = self.receiver.recv().await {
                    // let contract = IWETH9::new(self.contract_addr, http_service.provider.clone());

                    // let decoded : Result<Data, alloy_sol_types::Error>= SolType::abi_decode(&encoded, true);
                }
            }
            ProviderType::WebSocket(wss_service) => {
                while let Some(x) = self.receiver.recv().await {
                    match x.chain_type {
                        ChainType::Ethereum => {
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
                            println!("in the less than");
                            println!("in the Greater than");
                            let contract =
                                IWETH9::new(self.contract_addr, wss_service.provider.clone());
                            let tx = TransactionRequest::default();

                            let builder = contract
                                .provider()
                                .send_transaction(tx)
                                .await
                                .unwrap()
                                .with_required_confirmations(2);
                            let tx_hash = builder.watch().await.unwrap();
                            println!("transacrtion  hash {}", tx_hash);

                            let total_supply = contract
                                .mapp(x.post_message.unitValue)
                                .call()
                                .await
                                .unwrap()
                                ._0;
                            println!("total_supply -----{}", total_supply)
                        }
                    }
                }
            }
        }
    }
}

impl ReceiveService {
    // async fn create_contarct_instance(contract_addr: Address, provider: ProviderType) {
    //     let contract = IWETH9::new(contract_addr, provider.);
    // }
    async fn generate_signatures() {}
}
