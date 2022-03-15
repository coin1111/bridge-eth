use ethers::{
    abi::Abi, contract::Contract, prelude::builders::ContractCall, prelude::Client,
    providers::JsonRpcClient, signers::Wallet, types::Address, types::U256,
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

    pub fn create_transfer_account<T: Into<U256>>(
        &self,
        receiver: Address,
        transfer_id: [u8; 16],
        amount: u64,
        gas_price: T,
    ) -> Result<ContractCall<P, Wallet, ()>, String> {
        let data = self
            .contract
            .method::<_, ()>("createTransferAccountThis", (receiver, amount, transfer_id))
            .map_err(|e| format!("Error data: {:?}", e))?;
        Ok(data.gas_price(gas_price))
    }

    pub fn withdraw_from_escrow_this<T: Into<U256>>(
        &self,
        sender: Address,
        receiver: Address,
        transfer_id: [u8; 16],
        balance: u64,
        gas_price: T,
    ) -> Result<ContractCall<P, Wallet, ()>, String> {
        let data = self
            .contract
            .method::<_, ()>(
                "withdrawFromEscrowThis",
                (sender, receiver, balance, transfer_id),
            )
            .map_err(|e| format!("Error data: {:?}", e))?;
        Ok(data.gas_price(gas_price))
    }
    pub fn close_transfer_account<T: Into<U256>>(
        &self,
        transfer_id: [u8; 16],
        gas_price: T,
    ) -> Result<ContractCall<P, Wallet, ()>, String> {
        let data = self
            .contract
            .method::<_, ()>("closeTransferAccountSender", (transfer_id,))
            .map_err(|e| println!("Error data: {}", e))
            .map_err(|e| format!("Error data: {:?}", e))?;
        Ok(data.gas_price(gas_price))
    }
}
