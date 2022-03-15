use bridge_ethers::bridge_escrow::BridgeEscrow;
use bridge_ethers::ol_token::OLToken;
use bridge_ethers::util::TransferId;
use ethers::providers::{Http, Provider};
use ethers::types::Address;
use std::convert::TryFrom;
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: bridge-eth <action> <args>");
        println!("Usage: bridge-eth deposit <sender> <receiver> <amount> <transfer_id>");
        println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
        println!("Usage: bridge-eth close-transfer-account <transfer_id>");
        exit(0);
    }

    let config = bridge_ethers::config::Config::new(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get_escrow_contract_address().unwrap();
    println!("escrow_addr: {:?}", escrow_addr);
    let url = config.get_provider_url().unwrap();
    let gas_price = config.get_gas_price().unwrap();
    let provider = Provider::<Http>::try_from(url.as_str()).unwrap();
    let signers = bridge_ethers::signers::get_signers().unwrap();

    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice").unwrap();
    println!(
        "caller: {:?}",
        Address::from(validator_wallet.private_key())
    );

    let path_abi_escrow = "abi/contracts/BridgeEscrow.sol/BridgeEscrow.json";
    let path_abi_ol = "abi/contracts/OLToken.sol/OLToken.json";
    if args[1] == "deposit" {
        if args.len() < 6 {
            println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
            exit(1);
        }

        let sender_name = args[2].clone();
        let receiver_name = args[3].clone();
        let amount = args[4].parse::<u64>().unwrap();
        let transfer_id_str = args[5].clone();
        let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
        let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &receiver_name).unwrap();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();

        let ol_addr = config.get_ol_contract_address().unwrap();
        let client_ol = sender_wallet.clone().connect(provider.clone());
        let ol_token = OLToken::new(ol_addr, path_abi_ol, &client_ol).unwrap();

        let data_approve = ol_token.approve(escrow_addr, amount, gas_price);
        let pending_tx_approve = data_approve
            .unwrap()
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx_approve: {:?}", pending_tx_approve);

        let client = sender_wallet.clone().connect(provider.clone());
        let bridge_escrow = BridgeEscrow::new(escrow_addr, path_abi_escrow, &client).unwrap();

        let data = bridge_escrow.create_transfer_account(
            Address::from(receiver_wallet.private_key()),
            transfer_id.bytes,
            amount,
            gas_price,
        );
        let pending_tx = data
            .unwrap()
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx: {:?}", pending_tx);
    } else if args[1] == "withdraw" {
        if args.len() < 6 {
            println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
            exit(1);
        }

        let client = validator_wallet.clone().connect(provider.clone());
        let bridge_escrow = BridgeEscrow::new(escrow_addr, path_abi_escrow, &client).unwrap();

        let sender_name = args[2].clone();
        let receiver_name = args[3].clone();
        let balance = args[4].parse::<u64>().unwrap();
        let transfer_id_str = args[5].clone();
        let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
        let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &receiver_name).unwrap();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.withdraw_from_escrow_this(
            Address::from(sender_wallet.private_key()),
            Address::from(receiver_wallet.private_key()),
            transfer_id.bytes,
            balance,
            gas_price,
        );
        let pending_tx = data
            .unwrap()
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx: {:?}", pending_tx);
    } else if args[1] == "close-transfer-account" {
        let client = validator_wallet.clone().connect(provider.clone());
        let bridge_escrow = BridgeEscrow::new(escrow_addr, path_abi_escrow, &client).unwrap();

        let transfer_id_str = args[2].clone();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.close_transfer_account(transfer_id.bytes, gas_price);
        let pending_tx = data
            .unwrap()
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx: {:?}", pending_tx);
    } else {
        println!("{} is not supported", args[1]);
        exit(1);
    };
}
