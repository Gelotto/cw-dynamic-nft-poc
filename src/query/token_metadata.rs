use crate::{error::ContractError, msg::TokenMetadataResponse, state::storage::TOKEN_METADATA};

use super::ReadonlyContext;

pub fn query_token_metadata(
    ctx: ReadonlyContext,
    token_id: String,
) -> Result<TokenMetadataResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    Ok(TOKEN_METADATA
        .load(deps.storage, &token_id)
        .and_then(|metadata| Ok(TokenMetadataResponse(metadata)))?)
}
