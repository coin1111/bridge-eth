use bridge_ethers::bridge_escrow_mod;
use bridge_ethers::oltoken_mod;
use bridge_ethers::util::{AccountInfo, TransferId};
use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
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
        println!("Usage: bridge-eth get-locked-info <transfer_id>");
        println!("Usage: bridge-eth get-unlocked-info <transfer_id>");
        println!("Usage: bridge-eth balance <account>");
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

    if args[1] == "deposit" {
        if args.len() < 6 {
            println!("Usage: bridge-eth deposit <sender> <receiver> <balance> <transfer_id>");
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
        let ol_token = oltoken_mod::OLToken::new(ol_addr, &client_ol);
        let data_approve = ol_token
            .approve(escrow_addr, U256::from(amount))
            .gas_price(gas_price);

        let pending_tx_approve = data_approve
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx_approve: {:?}", pending_tx_approve);

        let client = sender_wallet.clone().connect(provider.clone());
        let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

        let data = bridge_escrow
            .create_transfer_account_this(
                Address::from(receiver_wallet.private_key()),
                amount,
                transfer_id.bytes,
            )
            .gas_price(gas_price);
        let pending_tx = data
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
        let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

        let sender_name = args[2].clone();
        let receiver_name = args[3].clone();
        let balance = args[4].parse::<u64>().unwrap();
        let transfer_id_str = args[5].clone();
        let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
        let receiver_wallet = bridge_ethers::signers::get_signer(&signers, &receiver_name).unwrap();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow
            .withdraw_from_escrow_this(
                Address::from(sender_wallet.private_key()),
                Address::from(receiver_wallet.private_key()),
                balance,
                transfer_id.bytes,
            )
            .gas_price(gas_price);
        let pending_tx = data
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx: {:?}", pending_tx);
    } else if args[1] == "close-transfer-account" {
        let client = validator_wallet.clone().connect(provider.clone());
        let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

        let transfer_id_str = args[2].clone();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow
            .close_transfer_account_sender(transfer_id.bytes)
            .gas_price(gas_price);
        let pending_tx = data
            .send()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("pending_tx: {:?}", pending_tx);
    } else if args[1] == "get-locked-info" {
        let client = validator_wallet.clone().connect(provider.clone());
        let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

        let transfer_id_str = args[2].clone();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.get_locked_account_info(transfer_id.bytes);
        let info = data
            .call()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("info: {:?}", info);
    } else if args[1] == "get-unlocked-info" {
        let client = validator_wallet.clone().connect(provider.clone());
        let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

        let transfer_id_str = args[2].clone();
        let transfer_id = TransferId::new(&transfer_id_str).unwrap();
        let data = bridge_escrow.get_unlocked_account_info(transfer_id.bytes);
        let info = data
            .call()
            .await
            .map_err(|e| println!("Error info: {}", e))
            .unwrap();
        let ai = AccountInfo::from(info).unwrap();
        println!("info: {:?}", ai);
    } else if args[1] == "balance" {
        if args.len() < 3 {
            println!("Usage: bridge-eth balance <account>");
            exit(1);
        }

        let sender_name = args[2].clone();
        let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();

        let ol_addr = config.get_ol_contract_address().unwrap();
        let client_ol = sender_wallet.clone().connect(provider.clone());
        let ol_token = oltoken_mod::OLToken::new(ol_addr, &client_ol);

        let data = ol_token.balance_of(Address::from(sender_wallet.private_key()));
        let call = data
            .call()
            .await
            .map_err(|e| println!("Error pending: {}", e))
            .unwrap();
        println!("call: {:?}", call);
    } else {
        println!("{} is not supported", args[1]);
        exit(1);
    };
}
