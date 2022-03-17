use ethers::abi::Tokenizable;
use ethers::types::H160;
use std::convert::TryInto;

/// Transfer id to track bridge transactions
pub struct TransferId {
    pub id: String,
    pub bytes: [u8; 16],
}
impl TransferId {
    /// Initialize using string literal
    pub fn new(id: &str) -> Result<TransferId, String> {
        let str = String::from(id);
        let bytes: [u8; 16] = hex_to_bytes(&str)
            .unwrap()
            .try_into()
            .map_err(|e| format!("cannot convert to hex: {:?}", e))?;
        Ok(TransferId {
            id: str,
            bytes: bytes,
        })
    }
}

/// Converts string to hex bytes.
/// Must not have 0x prefix.
/// Must have event number of characters.
pub fn hex_to_bytes(s: &String) -> Option<Vec<u8>> {
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

#[derive(Debug)]
pub struct AccountInfo {
    sender_this: H160,
    sender_other: [u8; 16],
    receiver_this: H160,
    receiver_other: [u8; 16],
    balance: u64,
    transfer_id: [u8; 16],
    idx: ethers::prelude::U256,
    is_closed: bool,
}

impl AccountInfo {
    pub fn from(tuple: ethers::abi::Token) -> Result<AccountInfo, String> {
        let v: Vec<ethers::abi::Token> = tuple
            .to_fixed_array()
            .ok_or_else(|| format!("Can't conver to tuple"))?;
        let sender_this: ethers::abi::Address = v[0]
            .clone()
            .to_address()
            .ok_or(format!("Can't conver sender_this"))?;
        let sender_other: [u8; 16] = v[1]
            .clone()
            .to_fixed_bytes()
            .ok_or(format!("Can't conver sender_other"))?
            .try_into()
            .map_err(|_| format!("Can't conver sender_other"))?;

        let receiver_this: ethers::abi::Address = v[2]
            .clone()
            .to_address()
            .ok_or(format!("Can't conver receiver_this"))?;
        let receiver_other: [u8; 16] = v[3]
            .clone()
            .to_fixed_bytes()
            .ok_or(format!("Can't conver receiver_other"))?
            .try_into()
            .map_err(|_| format!("Can't conver receiver_other"))?;
        let balance: u64 = v[4]
            .clone()
            .to_uint()
            .ok_or(format!("Can't conver balance"))?
            .as_u64();
        let transfer_id: [u8; 16] = v[5]
            .clone()
            .to_fixed_bytes()
            .ok_or(format!("cannot convert transfer_id"))?
            .try_into()
            .map_err(|_| format!("Can't conver transfer_id"))?;
        let idx: ethers::prelude::U256 =
            ethers::abi::Uint::from_token(v[6].clone()).map_err(|_| format!("Can't conver idx"))?;
        let is_closed: bool = v[7]
            .clone()
            .to_bool()
            .ok_or(format!("Can't conver is_closed"))?;
        Ok(AccountInfo {
            sender_this,
            sender_other,
            receiver_this,
            receiver_other,
            balance,
            transfer_id,
            idx,
            is_closed,
        })
    }
}
