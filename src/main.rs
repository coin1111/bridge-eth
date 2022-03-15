use bridge_ethers::bridge_escrow::BridgeEscrow;
use bridge_ethers::util::TransferId;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use std::convert::TryFrom;
use std::env;

#[tokio::main]
async fn main() {
    for argument in env::args() {
        println!("{}", argument);
    }

    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let config = bridge_ethers::config::Config::new(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get_escrow_contract_address().unwrap();
    println!("escrow_addr: {:?}", escrow_addr);

    let signers = bridge_ethers::signers::get_signers().unwrap();
    let sender_wallet = bridge_ethers::signers::get_signer(&signers, &"pete").unwrap();
    let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &"todd").unwrap();
    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice").unwrap();
    let path_abi = "abi/contracts/BridgeEscrow.sol/BridgeEscrow.json";
    let client = validator_wallet.clone().connect(provider.clone());
    println!(
        "caller: {:?}",
        Address::from(validator_wallet.private_key())
    );
    let bridge_escrow = BridgeEscrow::new(escrow_addr, path_abi, &client).unwrap();
    let transfer_id = TransferId::new("eab47fa3a3dc42bc8cbc48c02182667e").unwrap();
    let balance: u64 = 10;
    let data = bridge_escrow.withdraw_from_escrow_this(
        sender_wallet,
        receiver_wallet,
        transfer_id.bytes,
        balance,
        83241151,
    );
    let pending_tx = data
        .unwrap()
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx: {:?}", pending_tx);
}
