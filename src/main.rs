#![allow(dead_code)]
use clap::{crate_version, Clap};
use ethereum_types::Address;

type Year = u8;

#[derive(Clap)]
#[clap(version = crate_version!())]
pub struct Opts {
    address: Address,
    financial_year: Year,
}

fn main() {
    let _opts: Opts = Opts::parse(); /* TODO: remove underscore */

    println!("Hello, world!");
}
