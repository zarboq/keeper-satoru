use cainome::cairo_serde::{ContractAddress, U256};
use starknet::core::types::FieldElement;
  
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
  
pub enum DecreasePositionSwapType {
    NoSwap,
    SwapPnlTokenToCollateralToken,
    SwapCollateralTokenToPnlToken
}