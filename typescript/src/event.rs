
use std::collections::VecDeque;
use serde::Serialize;
use wasm_bindgen::prelude::*;
use crate::owned::OwnedEvent;

#[derive(Debug, tsify::Tsify, Serialize)]
#[tsify(into_wasm_abi)]
pub struct EventsInterop(pub VecDeque<OwnedEvent>);
