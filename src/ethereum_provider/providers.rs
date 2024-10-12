use alloy::{
    primitives::Address,
    providers::{self, Provider, ProviderBuilder,WsConnect},
};
use alloy_provider::RootProvider;
use alloy_transport_http::Http;
use reqwest::Client;
use url::Url;
use alloy_pubsub::PubSubFrontend;

struct HttpProvider {
    provider: RootProvider<Http<Client>>,
}

struct WssProvider {
    provider : RootProvider<PubSubFrontend>
}
struct Services{
    provider : Box<Self> 

}

pub async fn build_provider(rpc_url: &str, contract_address: Address, event: &str) {
    if rpc_url.starts_with("https") || rpc_url.starts_with("http") {
        let url = Url::parse(rpc_url).unwrap();
        let provider = ProviderBuilder::new().on_http(url);
        let http_provider = HttpProvider { provider };
    } else if rpc_url.starts_with("wss") || rpc_url.starts_with("ws") {
        let connect = WsConnect::new(rpc_url);
        let provider = ProviderBuilder::new().on_ws(connect).await.unwrap();
       let wss_provider =  WssProvider{
            provider
        };

    } else {
        println!("invalid Rpc Url")
    }
}
