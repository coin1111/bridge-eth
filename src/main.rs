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
use ethers::types::{Address, H256};
use serde_json::Error;
use std::fs;
use std::io::Read;
use async_std::task;
use std::convert::TryInto;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
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
    let transfer_id_str="eab47fa3a3dc42bc8cbc48c02182669e";
    let transfer_id:[u8;16] = hex_to_bytes(&String::from(transfer_id_str)).unwrap()
        .try_into().unwrap();
    let balance:u64 = 10;
    println!("{:?} {:?} {:?} {:?}",Address::from(sender_wallet.private_key()),
             Address::from(receiver_wallet.private_key()),
             balance,
             transfer_id);
    let data = bridge_escrow_contract
        .method::<_, ()>("withdrawFromEscrowThis",
                           (
                               Address::from(sender_wallet.private_key()),
                               Address::from(receiver_wallet.private_key()),
                               balance,
                               transfer_id)).map_err(|e|{
        println!("Error: {}",e)
    }).unwrap();
        let tx_hash = data
            .send().await.map_err(|e|{
            println!("Error tx: {:?}",e)
        }).unwrap();
        println!("withdrawFromEscrowThis: {:?}", tx_hash);

    fn hex_to_bytes(s: &String) -> Option<Vec<u8>> {
        if s.len() % 2 == 0 {
            (0..s.len())
                .step_by(2)
                .map(|i| {
                    s.get(i..i + 2)
                        .and_then(|sub| u8::from_str_radix(sub, 16).ok())
                })
                .collect()
        } else {
            None
        }
    }
}




