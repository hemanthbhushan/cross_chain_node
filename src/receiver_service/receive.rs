use crate::provider_services::providers::{ProviderType, Transfer};
use crate::service_manager::service_manager::Service;
use alloy::network::{EthereumWallet, NetworkWallet};
use alloy::primitives::Address;
use alloy::signers::local::PrivateKeySigner;
use alloy_sol_types::{sol, SolCall, SolStruct, SolType, SolValue};
use tokio::sync::mpsc::{self, Receiver};
use alloy::providers::{ProviderBuilder};

sol! {
    struct Data{
        address from;
        address to ;
        uint256 amount;

    }
}

pub struct ReceiveService {
    provider_type: ProviderType,
    contract_addr: Address,
    receiver: Receiver<Transfer>,
}

impl ReceiveService {
    pub fn new(
        provider_type: ProviderType,
        contract_addr: Address,
        receiver: Receiver<Transfer>,
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
        println!("im here in the receiverrrr");
        match &mut self.provider_type {
            ProviderType::Http(http_service) => {
                while let Some(x) = self.receiver.recv().await {

                    // let decoded : Result<Data, alloy_sol_types::Error>= SolType::abi_decode(&encoded, true);
                }
            }
            ProviderType::WebSocket(wss_service) => {
                while let Some(x) = self.receiver.recv().await {
                    let signer: PrivateKeySigner =
                        "ae448a0460df6455bcf354a2b8ab9eb6081b689591fd15e0fb6082ecf57bdd58"
                            .parse()
                            .expect("invalid prvt key");
                    let wallet = EthereumWallet::new(signer);
                    // let provider = ProviderBuilder::new().wallet(wallet);
                  
                    let ss = Data {
                        from: x.from,
                        to: x.to,
                        amount: x.value,
                    };

                    let encoded = ss.abi_encode().tokenize().0;
                    

                    // println!("in the receeeee {:?}", encoded);
                }
            }
        }
    }
}

impl ReceiveService {
    async fn generate_signatures() {}
}
