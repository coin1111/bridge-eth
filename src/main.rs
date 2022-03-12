use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use bridge_ethers::config;
use std::convert::TryFrom;

fn main() {
    let provider = Provider::<Http>::try_from(
        "http://localhost:8545",
    )
        .unwrap();
    let config = bridge_ethers::config::Config::read_config(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get("escrowContract").unwrap();
    println!("Hello, world!: {:?}",escrow_addr);
}




