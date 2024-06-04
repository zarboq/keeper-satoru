use cainome::cairo_serde::{ContractAddress, U256};
use serde::Deserialize;
use starknet::core::types::FieldElement;

use crate::listen_db::ActionType;
  
pub struct Order {
    pub key: FieldElement,
    pub order_type: OrderType,
    pub decrease_position_swap_type: DecreasePositionSwapType,
    pub account: ContractAddress,
    pub receiver: ContractAddress,
    pub callback_contract: ContractAddress,
    pub ui_fee_receiver: ContractAddress,
    pub market: ContractAddress,
    pub initial_collateral_token: ContractAddress,
    pub swap_path: Vec<ContractAddress>,
    pub size_delta_usd: U256,
    pub initial_collateral_delta_amount: U256,
    pub trigger_price: U256,
    pub acceptable_price: U256,
    pub execution_fee: U256,
    pub callback_gas_limit: U256,
    pub min_output_amount: U256,
    pub updated_at_block: u64,
    pub is_long: bool,
    pub is_frozen: bool,
}

// Struct used to decode JSON formatted data received from the database.
#[derive(Debug, Deserialize)]
pub struct RowDataOrder {
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
pub struct PayloadOrder {
    pub table: String,
    pub action_type: ActionType,
    pub row_data: RowDataOrder,
}
  
#[derive(Deserialize, Debug)]
pub enum OrderType {
    MarketSwap,
    LimitSwap,
    MarketIncrease,
    LimitIncrease,
    MarketDecrease,
    LimitDecrease,
    StopLossDecrease,
    Liquidation
}

#[derive(Deserialize, Debug)]
pub enum DecreasePositionSwapType {
    NoSwap,
    SwapPnlTokenToCollateralToken,
    SwapCollateralTokenToPnlToken
}