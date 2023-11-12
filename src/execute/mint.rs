use crate::{
    error::ContractError,
    msg::QueryMsg,
    state::{
        models::TokenMetadata,
        storage::{CW721_ADDR, TOKEN_DATA_NETWORK, TOKEN_ID_COUNTER, TOKEN_METADATA},
    },
};
use cosmwasm_std::{attr, to_json_binary, Addr, Empty, Response, StdResult, WasmMsg};
use cw721_base::ExecuteMsg;

use super::Context;

pub fn exec_mint(
    ctx: Context,
    owner: Addr,
    metadata: TokenMetadata,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;

    let cw721_addr = CW721_ADDR.load(deps.storage)?;
    let network = TOKEN_DATA_NETWORK.load(deps.storage)?;

    // Get next token ID
    let token_id = TOKEN_ID_COUNTER
        .update(deps.storage, |n| -> StdResult<_> { Ok(n + 1) })?
        .to_string();

    // Build token_uri
    let token_uri = format!(
        "cw://{}/{}/{}",
        network,
        env.contract.address,
        to_json_binary(&QueryMsg::TokenMetadata {
            token_id: token_id.clone(),
        })?
        .to_base64()
    );

    TOKEN_METADATA.save(deps.storage, &token_id, &metadata)?;

    Ok(Response::new()
        .add_attributes(vec![attr("action", "mint")])
        .add_message(WasmMsg::Execute {
            contract_addr: cw721_addr.into(),
            msg: to_json_binary(&ExecuteMsg::Mint::<Option<Empty>, Empty> {
                token_id,
                owner: owner.into(),
                token_uri: Some(token_uri),
                extension: None,
            })?,
            funds: vec![],
        }))
}
