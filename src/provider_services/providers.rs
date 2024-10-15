use alloy::sol;
use alloy::{
    primitives::Address,
    providers::{Provider, ProviderBuilder, WsConnect},
};
use alloy_provider::RootProvider;
use alloy_pubsub::{PubSubFrontend, Subscription};

use alloy::network::EthereumWallet;
use alloy::signers::local::PrivateKeySigner;
use alloy_transport_http::Http;
use futures::{stream, StreamExt};
use log::info;
use reqwest::Client;
use std::fmt::Debug;
use tokio::sync::mpsc::Sender;
use url::Url;
sol! {
    #[derive(Debug)]
    event PostMessage(
        address indexed contactAddress, // The contact address
        uint256 fromChainId,            // The source chain ID
        uint256 toChainId,              // The destination chain ID
        uint256 unitValue               // The unit value
    );
}

type WssFillers = alloy_provider::fillers::FillProvider<
    alloy_provider::fillers::JoinFill<
        alloy_provider::fillers::JoinFill<
            alloy_provider::Identity,
            alloy_provider::fillers::JoinFill<
                alloy_provider::fillers::GasFiller,
                alloy_provider::fillers::JoinFill<
                    alloy_provider::fillers::BlobGasFiller,
                    alloy_provider::fillers::JoinFill<
                        alloy_provider::fillers::NonceFiller,
                        alloy_provider::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy_provider::fillers::WalletFiller<EthereumWallet>,
    >,
    RootProvider<PubSubFrontend>,
    PubSubFrontend,
    alloy::network::Ethereum,
>;
type HttpFillers = alloy_provider::fillers::FillProvider<
    alloy_provider::fillers::JoinFill<
        alloy_provider::fillers::JoinFill<
            alloy_provider::Identity,
            alloy_provider::fillers::JoinFill<
                alloy_provider::fillers::GasFiller,
                alloy_provider::fillers::JoinFill<
                    alloy_provider::fillers::BlobGasFiller,
                    alloy_provider::fillers::JoinFill<
                        alloy_provider::fillers::NonceFiller,
                        alloy_provider::fillers::ChainIdFiller,
                    >,
                >,
            >,
        >,
        alloy_provider::fillers::WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    alloy::network::Ethereum,
>;

trait ProviderService {
    fn start(&self) {}
}
#[derive(Clone)]
pub struct HttpProviderService {
    pub provider: HttpFillers,
}
impl ProviderService for HttpProviderService {}
#[derive(Clone)]
pub struct WssProviderService {
    pub provider: WssFillers,
}
impl ProviderService for WssProviderService {}
#[derive(Clone)]
pub enum ProviderType {
    Http(HttpProviderService),
    WebSocket(WssProviderService),
}
impl ProviderType {
    pub async fn new(rpc_url: &str) -> Self {
        let signer: PrivateKeySigner =
            "1d0350390ccbec7bb44d6a28bfc7f1a2189717e9627f69236d13a5f555fee176"
                .parse()
                .expect("invalid private Key");
        let wallet = EthereumWallet::from(signer);
        if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
            let provider = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_http(Url::parse(rpc_url).unwrap());
            info!("Using HTTP provider");
            return Self::Http(HttpProviderService { provider });
        } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
            let connect = WsConnect::new(rpc_url);
            let provider: WssFillers = ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_ws(connect)
                .await
                .unwrap();
            return Self::WebSocket(WssProviderService { provider });
        } else {
            println!("Invalid RPC URL");
            panic!("Invalid RPC URL");
        }
    }
}
