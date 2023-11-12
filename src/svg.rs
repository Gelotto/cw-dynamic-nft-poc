use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_binary, Binary, Empty};
use svg::Document;

use crate::error::ContractError;

#[cw_serde]
pub struct SvgImageParams {
    pub href: String,
    pub x: String,
    pub y: String,
    pub w: String,
    pub h: String,
}

#[cw_serde]
pub struct SvgPathParams {
    pub fill: Option<String>,
    pub stroke: Option<String>,
    pub stroke_width: Option<String>,
    pub data: Option<Empty>,
}

#[cw_serde]
pub struct Svg {
    pub w: u32,
    pub h: u32,
    pub elements: Vec<SvgElementParams>,
}

#[cw_serde]
pub enum SvgElementParams {
    Image(SvgImageParams),
    Path(SvgPathParams),
}

impl Svg {
    pub fn build(&self) -> Result<Binary, ContractError> {
        let mut doc = Document::new().set("viewBox", (0, 0, self.w, self.h));
        for element_params in self.elements.iter() {
            doc = doc.add(match element_params {
                SvgElementParams::Image(params) => Self::build_image_node(params),
                SvgElementParams::Path(_params) => todo!(),
            }?);
        }
        Ok(to_json_binary(&doc.to_string())?)
    }

    fn build_image_node(
        params: &SvgImageParams
    ) -> Result<svg::node::element::Image, ContractError> {
        Ok(svg::node::element::Image::new()
            .set("x", params.x.clone())
            .set("y", params.y.clone())
            .set("width", params.w.clone())
            .set("height", params.h.clone())
            .set("href", params.href.clone()))
    }
}
