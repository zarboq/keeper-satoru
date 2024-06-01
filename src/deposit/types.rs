use cainome::cairo_serde::{ContractAddress, U256};
use starknet::core::types::FieldElement;

pub struct Deposit {
    pub key: FieldElement,
    pub account: ContractAddress,
    pub receiver: ContractAddress,
    pub callback_contract: ContractAddress,
    pub market: ContractAddress,
    pub initial_long_token: ContractAddress,
    pub initial_short_token: ContractAddress,
    pub long_token_swap_path: Vec<ContractAddress>,
    pub short_token_swap_path: Vec<ContractAddress>,
    pub initial_long_token_amount: U256,
    pub initial_short_token_amount: U256,
    pub min_market_tokens: U256,
    pub updated_at_block: u64,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
}