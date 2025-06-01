use cooklang::metadata::{NameAndUrl, RecipeTime};
use serde::Serialize;

#[derive(Debug, tsify::Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct MetadataInterop {
    pub tags: Option<Vec<String>>,
    pub author: Option<NameAndUrl>,
    pub source: Option<NameAndUrl>,
    pub time: Option<RecipeTime>,
    pub servings: Option<Vec<u32>>,
    pub locale: Option<(String, Option<String>)>,
}