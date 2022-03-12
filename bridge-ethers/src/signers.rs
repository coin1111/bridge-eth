use ethers::{
    providers::{Http, Provider},
    signers::Wallet,
    types::TransactionRequest,
};
use std::fs;
use std::io::{Error, ErrorKind};

pub struct Signers;

impl Signers {
    fn get_private_key(path: &str) ->Result<String, Box<Error>> {
        let line = fs::read_to_string(path)?;
        // line[1].split(":")[1].trim();
        match line.split("\n").take(2).last()
            .and_then(|s|{s.clone().split(":").take(2).last().
                and_then(|s|{Some(s.clone().trim())})})  {
            Some(s) => Ok(String::from(s)),
            None => Err(Box::from(Error::new(ErrorKind::Other, "can't find private key")))
        }
    }
}