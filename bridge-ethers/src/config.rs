use ethers::types::Address;
use serde_json::{Map, Value};
use std::fs;
pub struct Config {
    obj: Map<String, Value>,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, String> {
        let config =
            fs::read_to_string(path).map_err(|e| format!("error reading config: {}", e))?;
        let parsed: Value =
            serde_json::from_str(&config).map_err(|e| format!("error deseralizing json: {}", e))?;
        Ok(Config {
            obj: parsed.as_object().unwrap().clone(),
        })
    }
    /// Gets escrow contract address
    pub fn get_escrow_contract_address(&self) -> Result<Address, String> {
        let escrow_addr_str = self.obj.get("escrowContract").map_or_else(
            || Err(format!("error escrowContract value is missing")),
            |x| Ok(x),
        )?;
        (escrow_addr_str.to_string().replace("\"", ""))[2..]
            .parse::<Address>()
            .map_err(|e| format!("error parsing address: {:?}", e))
    }
}
