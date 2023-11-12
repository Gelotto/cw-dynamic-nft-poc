use cosmwasm_std::{to_json_binary, Binary};
use svg::Document;

use crate::{error::ContractError, msg::TokenImageResponse, state::storage::TOKEN_SVG};

use super::ReadonlyContext;

pub fn query_token_image(
    ctx: ReadonlyContext,
    token_id: String,
) -> Result<TokenImageResponse, ContractError> {
    let ReadonlyContext { deps, .. } = ctx;
    let svg = TOKEN_SVG.load(deps.storage, &token_id)?;
    Ok(TokenImageResponse(svg.build()?.to_base64()))
}

pub struct SvgImageParams {
    pub href: String,
    pub w: String,
    pub h: String,
}

pub struct SvgParams {
    pub w: u32,
    pub h: u32,
    pub elements: Vec<SvgElementParams>,
}

pub enum SvgElementParams {
    Image(SvgImageParams),
}

pub fn build_svg(params: &SvgParams) -> Result<Binary, ContractError> {
    let mut doc = Document::new().set("viewBox", (0, 0, params.w, params.h));
    for element_params in params.elements.iter() {
        doc = doc.add(match element_params {
            SvgElementParams::Image(params) => build_svg_image(params),
        }?);
    }
    Ok(to_json_binary(&doc.to_string())?)
}

pub fn build_svg_image(
    params: &SvgImageParams
) -> Result<svg::node::element::Image, ContractError> {
    Ok(svg::node::element::Image::new()
        .set("href", params.href.clone())
        .set("width", params.w.clone())
        .set("height", params.h.clone()))
}
