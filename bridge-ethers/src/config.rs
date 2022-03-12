use serde_json::{Map, Value};
use std::error::Error;
use std::fs;
pub struct Config;

impl Config {
    pub fn read_config(path: &str) -> Result<Map<String, Value>, Box<Error>> {
        let config = fs::read_to_string(path)?;
        let parsed: Value = serde_json::from_str(&config)?;
        let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
        Ok(obj)
    }
}