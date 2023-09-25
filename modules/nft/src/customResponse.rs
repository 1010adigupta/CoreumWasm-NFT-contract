use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Uint256};
use coreum_wasm_sdk::nft::BalanceResponse;
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GetInfoResponse {
    pub current_token_id: Uint256,
    pub balance: BalanceResponse,
    pub max_total_mint: Uint256,
}