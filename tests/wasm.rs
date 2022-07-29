#![cfg(target_family = "wasm")]

use eip5139::{RpcProviders, Source};

use js_sys::Function;

use serde_json::json;

use wasm_bindgen::JsValue;

use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn throws() {
    let fetch = Function::new_no_args(r#"throw Error("hello world")"#);
    let fetch = JsValue::from(fetch);

    let source = JsValue::from_serde(&json!({"uri": "file://one"})).unwrap();

    let err = RpcProviders::fetch_js(fetch.into(), source.into())
        .await
        .unwrap_err();

    let err = js_sys::Error::from(err);
    let cause = js_sys::Error::from(err.cause());
    assert_eq!(cause.message(), "hello world");
}
