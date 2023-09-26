use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr, Uint256};

pub const CLASS_ID: Item<String> = Item::new("class_id");
pub const PREREVEAL_TOKEN_URI: Item<String> = Item::new("prereveal_token_uri");
pub const TREASURY_ADDRESS: Item<Addr> = Item::new("treasury_address");
pub const PROTOCOL_ADDRESS: Item<Addr> = Item::new("protocol_address");
pub const CURRENT_TOKEN_ID: Item<Uint256> = Item::new("current_token_id");
pub const MINT_PRICE: Item<Uint256> = Item::new("mint_price");
pub const SALE_START_TIME: Item<Uint256> = Item::new("sale_start_time");
pub const SALE_END_TIME: Item<Uint256> = Item::new("sale_end_time");
pub const PROTOCOL_FEE: Item<Uint256> = Item::new("protocol_fee");
pub const MAX_TOTAL_MINT: Item<Uint256> = Item::new("max_total_mint");
pub const IS_WHITELISTED: Map<&String, bool> = Map::new("is_whitelisted");
pub const URI_STATUS: Item<bool> = Item::new("uri_status");
pub const NFTS: Item<Vec<Uint256>> = Item::new("uri_status");
pub const DENOM: Item<String> = Item::new("state");