use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use bridge_ethers::config;
use bridge_ethers::signers;
use std::convert::TryFrom;

fn main() {
    let provider = Provider::<Http>::try_from(
        "http://localhost:8545",
    )
        .unwrap();
    let config = bridge_ethers::config::Config::read_config(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get("escrowContract").unwrap();
    println!("escrow_addr: {:?}",escrow_addr);

    let signers = bridge_ethers::signers::get_signers().unwrap();

    let sender_wallet = bridge_ethers::signers::get_signer(&signers, &"pete");
    let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &"todd");
    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice");

}




