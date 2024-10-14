pub mod ethereum_provider;
pub mod services;

#[derive(Serialize, Deserialize, Clone)]
struct ChainConfig {
    source_rpc_url: String,
    dest_rpc_url: String,
    source_contarct_addr: Address,
    dest_contarct_addr: Address,
    source_event: String,
}