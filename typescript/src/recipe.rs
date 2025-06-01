use cooklang::{RecipeResult, ScalableRecipe, ScaledRecipe};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;


#[derive(Debug, tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct RecipeResultInterop (pub RecipeResult);

#[derive(Debug, tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScalableRecipeInterop(pub ScalableRecipe);

#[derive(Debug, tsify::Tsify, Serialize, Deserialize)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct ScaledRecipeInterop(pub ScaledRecipe);

