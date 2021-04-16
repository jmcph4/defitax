use std::env;
use std::fmt::Display;
use std::str::FromStr;

use ethereum_types::{Address, H256, U256};
use serde::de::{self, Deserializer, Unexpected, Error};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EtherscanTokenTx {
    #[serde(alias = "blockNumber", deserialize_with = "from_str")]
    pub block_number: u64,
    #[serde(alias = "timeStamp", deserialize_with = "from_str")]
    pub timestamp: u64,
    pub hash: H256,
    #[serde(deserialize_with = "from_str")]
    pub nonce: u64,
    #[serde(alias = "blockHash")]
    pub block_hash: H256,
    pub from: Address,
    #[serde(alias = "contractAddress")]
    pub contract_address: Address,
    pub to: Address,
    #[serde(deserialize_with = "from_dec_str")]
    pub value: U256,
    #[serde(alias = "tokenName")]
    pub token_name: String,
    #[serde(alias = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(alias = "tokenDecimal", deserialize_with = "from_str")]
    pub token_decimal: u64,
    #[serde(alias = "transactionIndex", deserialize_with = "from_str")]
    pub transaction_index: u64,
    #[serde(deserialize_with = "from_dec_str")]
    pub gas: U256,
    #[serde(alias = "gasPrice", deserialize_with = "from_dec_str")]
    pub gas_price: U256,
    #[serde(alias = "gasUsed", deserialize_with = "from_dec_str")]
    pub gas_used: U256,
    #[serde(alias = "cumulativeGasUsed", deserialize_with = "from_dec_str")]
    pub cumulative_gas_used: U256,
    pub input: String, /* deprecated */
    #[serde(deserialize_with = "from_str")]
    pub confirmations: u64,
}

fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

fn from_dec_str<'de, D>(deserializer: D) -> Result<U256, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    U256::from_dec_str(&s).map_err(|_e| {
        D::Error::invalid_type(
            Unexpected::Other(&"non-decimal string"),
            &"decimal string",
        )
    })
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EtherscanTokenTxList(pub Vec<EtherscanTokenTx>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EtherscanTokenResponse {
    #[serde(deserialize_with = "from_str")]
    pub status: u64,
    pub message: String,
    pub result: Vec<EtherscanTokenTx>,
}

impl EtherscanTokenTxList {
    pub fn total_gas_cost(&self) -> u64 {
        self.0
            .iter()
            .map(|tx| tx.gas_used.as_u64())
            .collect::<Vec<u64>>()
            .iter()
            .sum()
    }
}

fn etherscan_request_url(address: Address, api_key: String) -> String {
    format!("https://api.etherscan.io/api?module=account&action=tokentx&address={:#x}&startBlock=0&endBlock=999999999&sort=asc&apiKey={}", address, api_key)
}

pub async fn fetch_token_transactions(
    address: Address,
) -> Result<EtherscanTokenTxList, String> {
    let api_key: String = match env::var("DEFITAX_ETHERSCAN_API_KEY") {
        Ok(t) => t,
        Err(e) => return Err(format!("{}", e)),
    };

    let body = reqwest::get(etherscan_request_url(address, api_key))
        .await
        .map_err(|_e| "Request failed".to_string())?
        .text()
        .await
        .map_err(|_e| "Empty response".to_string())?;

    let response: EtherscanTokenResponse = match serde_json::from_str(&body) {
        Ok(t) => t,
        Err(e) => return Err(format!("fetch_token_transactions: {}", e))
    };

    Ok(EtherscanTokenTxList(response.result))
}
