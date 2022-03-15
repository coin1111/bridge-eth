use bridge_ethers::bridge_escrow::BridgeEscrow;
use bridge_ethers::util::TransferId;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use std::convert::{TryFrom};
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: bridge-eth <action> <args>");
        println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
        exit(0);
    }
    let gas_price = 83241151;
    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();
    let config = bridge_ethers::config::Config::new(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get_escrow_contract_address().unwrap();
    println!("escrow_addr: {:?}", escrow_addr);
    let signers = bridge_ethers::signers::get_signers().unwrap();

    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice").unwrap();
    let path_abi = "abi/contracts/BridgeEscrow.sol/BridgeEscrow.json";
    let client = validator_wallet.clone().connect(provider.clone());
    println!(
        "caller: {:?}",
        Address::from(validator_wallet.private_key())
    );
    let bridge_escrow = BridgeEscrow::new(escrow_addr, path_abi, &client).unwrap();


    let data = if args[1] == "withdraw" {
        if args.len() < 6 {
            println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
            exit(1);
        }

        let sender_name = args[2].clone();
        let receiver_name = args[3].clone();
        let balance = args[4].parse::<u64>().unwrap();
        let transfer_id_str = args[5].clone();
        let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
        let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &receiver_name).unwrap();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.withdraw_from_escrow_this(
            sender_wallet,
            receiver_wallet,
            transfer_id.bytes,
            balance,
            gas_price,
        );
        data
    } else if args[1] == "close-transfer-account" {
        let transfer_id_str = args[2].clone();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.close_transfer_account(
            transfer_id.bytes,
            gas_price,
        );
        data
    } else {
        println!("{} is not supported",args[1]);
        exit(1);
    };
    let pending_tx = data
        .unwrap()
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx: {:?}", pending_tx);

}
