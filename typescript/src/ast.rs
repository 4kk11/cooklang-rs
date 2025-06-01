use serde::Serialize;

use crate::owned::OwnedAst;

#[derive(Debug, Serialize, Clone, tsify::Tsify)]
#[tsify(into_wasm_abi)]
pub struct AstInterop(pub OwnedAst);