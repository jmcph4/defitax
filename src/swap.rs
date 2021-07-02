use ethereum_types::U256;

use crate::data_source::get_reference_price;

#[derive(Clone, Debug)]
pub struct TokenQuantity {
    pub ticker: String,
    pub amount: U256,
    pub reference_amount: Option<u128>,
}

#[derive(Clone, Debug)]
pub struct TokenSwap {
    pub from: TokenQuantity,
    pub to: TokenQuantity,
}

impl TokenQuantity {
    pub fn assign_reference_price(&mut self) {
        self.reference_amount =
            get_reference_price(self.ticker.clone(), self.amount).ok();
    }
}

pub fn scale_token_amount(amount: U256, decimals: u64) -> U256 {
    let denominator: u128 = 10_u128.pow(decimals as u32);

    amount / denominator
}
