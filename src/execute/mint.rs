use crate::{
    error::ContractError,
    msg::QueryMsg,
    state::{
        models::TokenMetadata,
        storage::{CW721_ADDR, TOKEN_ID_COUNTER, TOKEN_METADATA, TOKEN_NETWORK, TOKEN_SVG},
    },
    svg::Svg,
};
use cosmwasm_std::{attr, to_json_binary, Addr, Empty, Response, StdResult, WasmMsg};
use cw721_base::ExecuteMsg;
use serde::Serialize;

use super::Context;

pub fn exec_mint(
    ctx: Context,
    owner: Addr,
    metadata: TokenMetadata,
    svg: Option<Svg>,
) -> Result<Response, ContractError> {
    let Context { deps, env, .. } = ctx;

    let cw721_addr = CW721_ADDR.load(deps.storage)?;
    let network = TOKEN_NETWORK.load(deps.storage)?;
    let mut metadata = metadata;

    // Get next token ID
    let token_id = TOKEN_ID_COUNTER
        .update(deps.storage, |n| -> StdResult<_> { Ok(n + 1) })?
        .to_string();

    // Build token_uri
    let token_uri = build_cw_uri(
        &network,
        &env.contract.address,
        &QueryMsg::TokenMetadata {
            token_id: token_id.clone(),
        },
    )?;

    if let Some(params) = svg {
        TOKEN_SVG.save(deps.storage, &token_id, &params)?;
        metadata.image = Some(build_cw_uri(
            &network,
            &env.contract.address,
            &QueryMsg::TokenImage {
                token_id: token_id.clone(),
            },
        )?)
    }

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

fn build_cw_uri<T>(
    network: &String,
    contract_addr: &Addr,
    msg: &T,
) -> StdResult<String>
where
    T: Serialize + ?Sized,
{
    Ok(format!(
        "cw://{}/{}/{}",
        network,
        contract_addr,
        to_json_binary(&msg)?.to_base64()
    ))
}
