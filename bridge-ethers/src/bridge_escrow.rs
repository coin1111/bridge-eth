use ethers::{
    abi::Abi, contract::Contract, prelude::builders::ContractCall, prelude::Client,
    providers::JsonRpcClient, signers::Wallet, types::Address,
};
use std::fs;

pub struct BridgeEscrow<'a, P: JsonRpcClient> {
    pub contract: Contract<'a, P, Wallet>,
}

impl<'a, P: JsonRpcClient> BridgeEscrow<'a, P> {
    pub fn new(
        escrow_address: Address,
        path_abi: &str,
        client: &'a Client<P, Wallet>,
    ) -> Result<BridgeEscrow<'a, P>, String> {
        let abi_json = fs::read_to_string(path_abi)
            .map_err(|e| format!("Can't open abi file: error: {:?}", e))?;
        let abi: Abi = serde_json::from_str(&abi_json)
            .map_err(|e| format!("Can't deserialize abi file: {}, error: {:?}", path_abi, e))?;

        Ok(BridgeEscrow {
            contract: Contract::new(escrow_address, abi, client),
        })
    }

    pub fn withdraw_from_escrow_this(
        &self,
        sender: Wallet,
        receiver: Wallet,
        client: &'a Client<P, Wallet>,
        transfer_id: [u8; 16],
        balance: u64,
    ) -> ContractCall<P, Wallet, ()> {
        let data = self
            .contract
            .connect(client)
            .method::<_, ()>(
                "withdrawFromEscrowThis",
                (
                    Address::from(sender.private_key()),
                    Address::from(receiver.private_key()),
                    balance,
                    transfer_id,
                ),
            )
            .map_err(|e| println!("Error data: {}", e))
            .unwrap()
            .gas_price(83241151);
        data
    }
}
