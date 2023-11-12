use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

use crate::{
    state::models::{Config, TokenMetadata},
    svg::Svg,
};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    SetConfig(Config),
    Mint {
        owner: Addr,
        metadata: TokenMetadata,
        svg: Option<Svg>,
    },
}

#[cw_serde]
pub enum QueryMsg {
    Config {},
    TokenMetadata { token_id: String },
    TokenImage { token_id: String },
}

#[cw_serde]
pub struct MigrateMsg {}

#[cw_serde]
pub struct ConfigResponse(pub Config);

#[cw_serde]
pub struct TokenMetadataResponse(pub TokenMetadata);

#[cw_serde]
pub struct TokenImageResponse(pub String);
