use cainome::cairo_serde::{ContractAddress, U256};
use starknet::core::types::FieldElement;
  
pub struct Withdrawal {
    pub key: FieldElement,
    pub account: ContractAddress,
    pub receiver: ContractAddress,
    pub callback_contract: ContractAddress,
    pub market: ContractAddress,
    pub long_token_swap_path: Vec<ContractAddress>,
    pub short_token_swap_path: Vec<ContractAddress>,
    pub market_token_amount: U256,
    pub min_long_token_amount: U256,
    pub min_short_token_amount: U256,
    pub updated_at_block: u64,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
}