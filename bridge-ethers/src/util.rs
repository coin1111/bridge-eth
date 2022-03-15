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
