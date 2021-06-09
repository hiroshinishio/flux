// module for all flux WASM functions
pub use crate::ast::*;
pub use crate::formatter::convert_to_string;
pub use crate::{ast, find_var_type};
pub use fluxcore::parser::Parser;
pub use fluxcore::semantic::types::{MonoType, Tvar};
pub use wasm_bindgen::prelude::*;

/// (Generated by WASM.)
#[wasm_bindgen]
pub fn parse(s: &str) -> JsValue {
    let mut p = Parser::new(s);
    let file = p.parse_file(String::from(""));

    JsValue::from_serde(&file).unwrap()
}

/// Format a JS file.
#[wasm_bindgen]
pub fn format_from_js_file(js_file: JsValue) -> String {
    if let Ok(file) = js_file.into_serde::<File>() {
        if let Ok(converted) = convert_to_string(&file) {
            return converted;
        }
    }
    "".to_string()
}

/// wasm version of the flux_find_var_type() API. Instead of returning a flat buffer that contains
/// the MonoType, it returns a JsValue。
#[wasm_bindgen]
pub fn wasm_find_var_type(source: &str, file_name: &str, var_name: &str) -> JsValue {
    let mut p = Parser::new(source);
    let pkg: ast::Package = p.parse_file(file_name.to_string()).into();
    let ty = find_var_type(pkg, var_name.to_string()).unwrap_or(MonoType::Var(Tvar(0)));
    JsValue::from_serde(&ty).unwrap()
}
