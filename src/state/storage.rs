use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use super::models::{Config, TokenMetadata};

pub const CONFIG: Item<Config> = Item::new("config");
pub const CW721_ADDR: Item<Addr> = Item::new("cw721_addr");
pub const TOKEN_DATA_NETWORK: Item<String> = Item::new("token_data_network");
pub const TOKEN_ID_COUNTER: Item<u16> = Item::new("token_id_counter");
pub const TOKEN_METADATA: Map<&String, TokenMetadata> = Map::new("token_metadata");
