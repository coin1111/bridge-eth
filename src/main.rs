use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use bridge_ethers::config;
use bridge_ethers::signers;
use std::convert::TryFrom;
use ethers::abi::Abi;
use ethers::contract::Contract;
use ethers::types::Address;
use serde_json::Error;
use std::fs;

fn main() {
    let provider = Provider::<Http>::try_from(
        "http://localhost:8545",
    )
        .unwrap();
    let config = bridge_ethers::config::Config::read_config(".bridge_escrow.config").unwrap();
    let escrow_addr_str = String::from(config.get("escrowContract").unwrap().
        to_string().replace("\"",""));
    let escrow_addr = (escrow_addr_str)[2..].parse::<Address>().unwrap();
    println!("escrow_addr: {:?}",escrow_addr);

    let signers = bridge_ethers::signers::get_signers().unwrap();

    let sender_wallet = bridge_ethers::signers::get_signer(&signers, &"pete").unwrap();
    let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &"todd").unwrap();
    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice").unwrap();
    let bridge_escrow_json = fs::read_to_string("abi/contracts/BridgeEscrow.sol/BridgeEscrow.json").unwrap();
    let abi: Abi = serde_json::from_str(&bridge_escrow_json).unwrap();
    let client = validator_wallet.connect(provider);
    let bridge_escrow_contract = Contract::new(escrow_addr, abi, &client);
    println!("bridge_escrow_contract: {:?}",bridge_escrow_contract);

}




