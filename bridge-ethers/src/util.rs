use ethers::abi::Tokenizable;
use ethers::types::H160;
use std::convert::TryInto;
use ethers_core::abi::Tokenize;
use ethers::abi::Token::Address;
use ethers::abi::Token::FixedBytes;
use ethers::abi::Token::Uint;
use ethers_core::abi::Token::Bool;

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
fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

impl AccountInfo {
    pub fn from(tuple: ethers::abi::Token) -> Result<AccountInfo, String> {
        match  tuple.clone() {
            ethers::abi::Token::Tuple(a) =>
                {
                    match &a[..] {
                        [Address(sender_this), FixedBytes(sender_other),
                        Address(receiver_this), FixedBytes(receiver_other), Uint(balance),
                        FixedBytes(transfer_id), Uint(idx), Bool(is_closed)] =>
                            {
                        let ai = AccountInfo {
                            sender_this:*sender_this,
                            sender_other:vec_to_array(sender_other.to_vec().clone()),
                            receiver_this:*receiver_this,
                            receiver_other:vec_to_array(receiver_other.to_vec().clone()),
                            balance:balance.as_u64(),
                            transfer_id:vec_to_array(transfer_id.to_vec().clone()),
                            idx:*idx,
                            is_closed:*is_closed,
                        };
                        println!("{:?}", ai)},
                        _ => (),
                    };
                },
            _ => (),
        };
        println!("{:?}",tuple);
        let v = tuple.clone().into_tokens();
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
