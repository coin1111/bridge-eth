use bridge_ethers::bridge_escrow_mod;
use bridge_ethers::config::Config;
use bridge_ethers::oltoken_mod;
use bridge_ethers::util::{hex_to_bytes, vec_to_array, AccountInfo, TransferId};
use ethers::prelude::Wallet;
use ethers::providers::{Http, JsonRpcClient, Provider};
use ethers::types::{Address, U256};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::process::exit;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        println!("Usage: bridge-eth <action> <args>");
        println!("Usage: bridge-eth deposit <sender> <receiver> <amount> [<transfer_id>]");
        println!("Usage: bridge-eth withdraw <sender> <receiver> <balance> <transfer_id>");
        println!("Usage: bridge-eth close-transfer-account <transfer_id>");
        println!("Usage: bridge-eth get-locked-info <transfer_id>");
        println!("Usage: bridge-eth get-unlocked-info <transfer_id>");
        println!("Usage: bridge-eth get-next-transfer-id <start> <n>");
        println!("Usage: bridge-eth balance <account>");
        exit(0);
    }

    let config = bridge_ethers::config::Config::new(".bridge_escrow.config").unwrap();
    let escrow_addr = config.get_escrow_contract_address().unwrap();
    println!("escrow_addr: {:?}", escrow_addr);
    let url = config.get_provider_url().unwrap();
    let gas_price = config.get_gas_price().unwrap();
    let provider: Provider<Http> = Provider::<Http>::try_from(url.as_str()).unwrap();
    let signers = bridge_ethers::signers::get_signers("accounts").unwrap();

    let validator_wallet = bridge_ethers::signers::get_signer(&signers, &"alice").unwrap();
    println!(
        "caller: {:?}",
        Address::from(validator_wallet.private_key())
    );

    if args[1] == "deposit" {
        deposit_cmd(&args, &config, escrow_addr, gas_price, &provider, &signers).await;
    } else if args[1] == "withdraw" {
        withdraw_cmd(
            &args,
            escrow_addr,
            gas_price,
            &provider,
            &signers,
            &validator_wallet,
        )
        .await;
    } else if args[1] == "close-transfer-account" {
        close_transfer_account_cmd(&args, escrow_addr, gas_price, &provider, &validator_wallet)
            .await;
    } else if args[1] == "get-locked-info" {
        get_locked_info_cmd(&args, escrow_addr, &provider, &validator_wallet).await;
    } else if args[1] == "get-unlocked-info" {
        get_unlocked_info_cmd(&args, escrow_addr, &provider, validator_wallet).await;
    } else if args[1] == "get-next-transfer-id" {
        get_next_transfer_id_cmd(&args, escrow_addr, &provider, validator_wallet).await;
    } else if args[1] == "balance" {
        balance_cmd(&args, config, provider, &signers).await;
    } else {
        println!("{} is not supported", args[1]);
        exit(1);
    };
}

async fn balance_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    config: Config,
    provider: Provider<P>,
    signers: &HashMap<&str, Wallet>,
) {
    if args.len() < 3 {
        println!("Usage: bridge-eth balance <account>");
        exit(1);
    }

    let sender_name = args[2].clone();
    let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
    let target = args.get(3).and_then(|x|{
        let pubk = match x.get(..2) {
            Some("0x") => x.get(2..).and_then(|s| {
                hex_to_bytes(&String::from(s).to_lowercase()).and_then(|v| {
                    match vec_to_array::<u8, 20>(v) {
                        Ok(a) => Some(Address::from(a)),
                        _ => None,
                    }
                })
            }),
            _ => None,
        };
        pubk
    });

    let target_addr = match target {
        Some(t) => Address::from(t),
        _ => Address::from(sender_wallet.public_key()),
    };

    let ol_addr = config.get_ol_contract_address().unwrap();
    let client_ol = sender_wallet.clone().connect(provider.clone());
    let ol_token = oltoken_mod::OLToken::new(ol_addr, &client_ol);

    let data = ol_token.balance_of(target_addr);
    let call = data
        .call()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("call: {:?}", call);
}
async fn get_next_transfer_id_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    escrow_addr: Address,
    provider: &Provider<P>,
    validator_wallet: Wallet,
) {
    if args.len() < 4 {
        panic!("too few arguments");
    }
    let client = validator_wallet.clone().connect(provider.clone());
    let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

    let start_i: u64 = args[2].clone().parse::<u64>().unwrap();
    let n_i: u64 = args[3].clone().parse::<u64>().unwrap();
    let start = U256::from(start_i);
    let n = U256::from(n_i);
    let data = bridge_escrow.get_next_transfer_id(start, n);
    let call = data.call();
    let info = call
        .await
        .map_err(|e| println!("Error info: {}", e))
        .unwrap();
    // let ai = AccountInfo::from(info).unwrap();
    println!("next transfer_id: {:?}", info.0)
}

async fn get_unlocked_info_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    escrow_addr: Address,
    provider: &Provider<P>,
    validator_wallet: Wallet,
) {
    let client = validator_wallet.clone().connect(provider.clone());
    let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

    let transfer_id_str = args[2].clone();
    let transfer_id = TransferId::from(&transfer_id_str).unwrap();
    let data = bridge_escrow.get_unlocked_account_info(transfer_id.bytes);
    let info = data
        .call()
        .await
        .map_err(|e| println!("Error info: {}", e))
        .unwrap();
    let ai = AccountInfo::from(info).unwrap();
    println!("unlocked: {:?}", ai);
}

async fn get_locked_info_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    escrow_addr: Address,
    provider: &Provider<P>,
    validator_wallet: &Wallet,
) {
    let client = validator_wallet.clone().connect(provider.clone());
    let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

    let transfer_id_str = args[2].clone();
    let transfer_id = TransferId::from(&transfer_id_str).unwrap();
    let data = bridge_escrow.get_locked_account_info(transfer_id.bytes);
    let info = data
        .call()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    let ai = AccountInfo::from(info).unwrap();
    println!("locked: {:?}", ai);
}

async fn close_transfer_account_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    escrow_addr: Address,
    gas_price: u64,
    provider: &Provider<P>,
    validator_wallet: &Wallet,
) {
    let client = validator_wallet.clone().connect(provider.clone());
    let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

    let transfer_id_str = args[2].clone();
    let transfer_id = TransferId::from(&transfer_id_str).unwrap();
    let data = bridge_escrow
        .close_transfer_account_sender(transfer_id.bytes)
        .gas_price(gas_price);
    let pending_tx = data
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx: {:?}", pending_tx);
}

async fn withdraw_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    escrow_addr: Address,
    gas_price: u64,
    provider: &Provider<P>,
    signers: &HashMap<&str, Wallet>,
    validator_wallet: &Wallet,
) {
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
    let transfer_id = TransferId::from(&transfer_id_str).unwrap();
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
}

async fn deposit_cmd<P: JsonRpcClient>(
    args: &Vec<String>,
    config: &Config,
    escrow_addr: Address,
    gas_price: u64,
    provider: &Provider<P>,
    signers: &HashMap<&str, Wallet>,
) {
    if args.len() < 5 {
        println!("Usage: bridge-eth deposit <sender> <receiver> <balance> [<transfer_id>]");
        exit(1);
    }

    let sender_name = args[2].clone();
    let receiver_name = args[3].clone();
    let amount = args[4].parse::<u64>().unwrap();
    let transfer_id = if args.len() > 5 {
        TransferId::from(&args[5]).unwrap()
    } else {
        let tid = TransferId::new().unwrap();
        println!("INFO: created transfer_id: {:?}", tid.id);
        tid
    };
    let sender_wallet = bridge_ethers::signers::get_signer(&signers, &sender_name).unwrap();
    let ol_addr = config.get_ol_contract_address().unwrap();
    let client_ol = sender_wallet.clone().connect(provider.clone());
    let ol_token = oltoken_mod::OLToken::new(ol_addr, &client_ol);
    let data_approve = ol_token
        .approve(escrow_addr, U256::from(amount))
        .gas_price(gas_price);

    let receiver_ol = match receiver_name.get(..2) {
        Some("0x") => receiver_name.get(2..).and_then(|s| {
            hex_to_bytes(&String::from(s).to_lowercase()).and_then(|v| {
                match vec_to_array::<u8, 16>(v) {
                    Ok(a) => Some(a),
                    _ => None,
                }
            })
        }),
        _ => None,
    };
    let receiver_address = if receiver_ol.is_none() {
        bridge_ethers::signers::get_signer(&signers, &receiver_name)
            .and_then(|w| Ok(Address::from(w.private_key())))
    } else {
        Err(format!("invalid eth receiver address"))
    };

    let pending_tx_approve = data_approve
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx_approve: {:?}", pending_tx_approve);

    let client = sender_wallet.clone().connect(provider.clone());
    let bridge_escrow = bridge_escrow_mod::BridgeEscrow::new(escrow_addr, &client);

    let data = if receiver_ol.is_none() {
        bridge_escrow
            .create_transfer_account_this(receiver_address.unwrap(), amount, transfer_id.bytes)
            .gas_price(gas_price)
    } else {
        bridge_escrow
            .create_transfer_account(receiver_ol.unwrap(), amount, transfer_id.bytes)
            .gas_price(gas_price)
    };
    let pending_tx = data
        .send()
        .await
        .map_err(|e| println!("Error pending: {}", e))
        .unwrap();
    println!("pending_tx: {:?}", pending_tx);
}
