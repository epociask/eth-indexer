use ethers::{prelude::{Http, Provider}, providers::Middleware, types::{BlockId, U64}};
use eyre::{eyre, Error};
use dotenv::dotenv;
use traversal::{Reader, Traversal};

struct Config {
    start_height: usize,
    rpc_host: String
}


impl Config {

    // load env vars from process
    fn new() -> Self {
        let start_height = std::env::var("START_HEIGHT").expect("START_HEIGHT must be set.").parse::<usize>().unwrap();
        let rpc_host = std::env::var("RPC_HOST").expect("RPC_HOST must be set.");

        Config {
            start_height,
            rpc_host
        }

    }

    async fn rpc_provider(&self) -> Result<Provider<Http>, Error> {
        Provider::<Http>::try_from(self.rpc_host.clone()).map_err(|e|eyre!(e))
    }
}


#[tokio::main]
async fn main() {
    // load env vars from .env into caller process
    dotenv().ok();

    let cfg = Config::new();
    // load dependencies
    let provider = cfg.rpc_provider().await.unwrap();

    let id = BlockId::Number(ethers::types::BlockNumber::Number(U64::from(cfg.start_height as usize)));
    let starting_block = provider.get_block(id).await.unwrap().unwrap();
    let mut reader = Reader::new(provider, starting_block).await;

    while true {
        let block = reader.next().await;
        println!("{:?}", block);
    }
}
