use wasm_bindgen::prelude::*;

#[wasm_bindgen(typescript_custom_section)]
const TS: &'static str = r#"
/// Manually added types due to limitations in the 'wasm-bindgen' & 'tsify' crates
export type Mapping = Map<any, any>;
export type Internal = any;
export type EnumMap<K, V> = Map<K, V>;
"#;
