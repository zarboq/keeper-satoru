use cainome::cairo_serde::{ContractAddress, U256};
use serde::Deserialize;
use starknet::core::types::FieldElement;

use crate::listen_db::ActionType;

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

// Struct used to decode JSON formatted data received from the database.
#[derive(Debug, Deserialize)]
pub struct RowDataDeposit {
    block_number: u64,
    transaction_hash: String,
    key: String,
    order_type: String,
    decrease_position_swap_type: String,
    account: String,
    receiver: String,
    callback_contract: String,
    ui_fee_receiver: String,
    market: String,
    initial_collateral_token: String,
    swap_path: String,
    size_delta_usd: u64,
    initial_collateral_delta_amount: u64,
    trigger_price: u64,
    acceptable_price: u64,
    execution_fee: u64,
    callback_gas_limit: u64,
    min_output_amount: u64,
    updated_at_block: u64,
    is_long: bool,
    is_frozen: bool,
}

// A struct representing the payload of a notification.
// @table: The table name in the database.
// @action_type: The type of action (using the ActionType enum).
// @row_data: The data of the affected row.
#[derive(Deserialize, Debug)]
pub struct PayloadDeposit {
    pub table: String,
    pub action_type: ActionType,
    pub row_data: RowDataDeposit,
}