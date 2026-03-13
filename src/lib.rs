//! Placeholder WASM crate for the DEXA compiler.
//! Full implementation lives in dxa-dev until 1.0.0.

use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RunResult {
    ok: bool,
    error: Option<String>,
}

/// Run DEXA source in the browser. Placeholder: returns error directing to dxa-dev.
#[wasm_bindgen(js_name = runDexa)]
pub fn run_dexa(source: &str) -> JsValue {
    let _ = source;
    let result = RunResult {
        ok: false,
        error: Some("Placeholder. Full implementation in dxa-dev until 1.0.0.".into()),
    };
    serde_wasm_bindgen::to_value(&result).unwrap_or_else(|_| JsValue::NULL)
}
