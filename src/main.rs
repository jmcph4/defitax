use std::process;

use clap::{crate_version, Clap};
use ethereum_types::Address;

use data_source::{fetch_token_transactions, EtherscanTokenTxList};

pub mod data_source;

type Year = u16;

#[derive(Clap)]
#[clap(version = crate_version!())]
#[allow(dead_code)]
pub struct Opts {
    address: Address,
    financial_year: Year,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    /* retrieve transactions */
    let transactions: EtherscanTokenTxList =
        match fetch_token_transactions(opts.address).await {
            Ok(t) => t,
            Err(e) => {
                eprintln!("defitax: {}", e);
                process::exit(1);
            }
        };

    println!("Total gas usage: {}", transactions.total_gas_cost());
}
