use ethers::{
    abi::Abi, contract::Contract, prelude::builders::ContractCall, prelude::Client,
    providers::JsonRpcClient, signers::Wallet, types::Address, types::U256,
};
use std::fs;

pub struct OLToken<'a, P: JsonRpcClient> {
    pub contract: Contract<'a, P, Wallet>,
}

impl<'a, P: JsonRpcClient> OLToken<'a, P> {
    pub fn new(
        escrow_address: Address,
        path_abi: &str,
        client: &'a Client<P, Wallet>,
    ) -> Result<OLToken<'a, P>, String> {
        let abi_json = fs::read_to_string(path_abi)
            .map_err(|e| format!("Can't open abi file: error: {:?}", e))?;
        let abi: Abi = serde_json::from_str(&abi_json)
            .map_err(|e| format!("Can't deserialize abi file: {}, error: {:?}", path_abi, e))?;

        Ok(OLToken {
            contract: Contract::new(escrow_address, abi, client),
        })
    }

    pub fn approve<T: Into<U256>>(
        &self,
        receiver: Address,
        amount: u64,
        gas_price: T,
    ) -> Result<ContractCall<P, Wallet, ()>, String> {
        let data = self
            .contract
            .method::<_, ()>("approve", (receiver, amount))
            .map_err(|e| format!("Error data: {:?}", e))?;
        Ok(data.gas_price(gas_price))
    }

    pub fn balance_of(&self, account: Address) -> Result<ContractCall<P, Wallet, U256>, String> {
        let data = self
            .contract
            .method::<_, U256>("balanceOf", account)
            .map_err(|e| format!("Error data: {:?}", e))?;
        Ok(data)
    }
}
