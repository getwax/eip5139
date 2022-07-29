#![cfg(target_family = "wasm")]

pub mod utils;

use eip5139::{RpcProviders, Source};

use js_sys::Function;

use self::utils::Fetch;

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

#[wasm_bindgen_test]
async fn version() {
    let one = r#"{
  "name": "Root List",
  "version": {
    "major": 0,
    "minor": 1,
    "patch": 1,
    "build": "XPSr.p.I.g.l"
  },
  "timestamp": "2004-08-08T00:00:00.0Z",
  "logo": "https://mylist.invalid/logo.png",
  "providers": {
  }
}"#;

    let fetch = Fetch::with_one(one);
    let version: serde_json::Value = RpcProviders::fetch(fetch, Source::Uri("file://one".into()))
        .await
        .unwrap()
        .version_js()
        .into_serde()
        .unwrap();

    assert_eq!(
        version,
        json!({
            "major": 0,
            "minor": 1,
            "patch": 1,
            "build": "XPSr.p.I.g.l"
        })
    );
}

#[wasm_bindgen_test]
async fn providers() {
    let one = r#"{
  "name": "Root List",
  "version": {
    "major": 0,
    "minor": 1,
    "patch": 1
  },
  "timestamp": "2004-08-08T00:00:00.0Z",
  "logo": "https://mylist.invalid/logo.png",
  "providers": {
    "foo": {
        "name": "Sourceri",
        "priority": 3,
        "chains": [
            {
                "chainId": 1,
                "endpoints": [
                    "https://mainnet.sourceri.invalid/"
                ]
            },
            {
                "chainId": 42,
                "endpoints": [
                    "https://kovan.sourceri.invalid"
                ]
            }
        ]
    },
    "bar": {
        "name": "Floop",
        "chains": [
            {
                "chainId": 42,
                "endpoints": [
                    "https://kovan.floop.invalid"
                ]
            }
        ]
    }
  }
}"#;

    let fetch = Fetch::with_one(one);
    let providers: serde_json::Value = RpcProviders::fetch(fetch, Source::Uri("file://one".into()))
        .await
        .unwrap()
        .providers_js()
        .into_serde()
        .unwrap();

    assert_eq!(
        providers,
        json!([
        {
            "name": "Sourceri",
            "priority": 3,
            "chains": [
                {
                    "chainId": 1,
                    "endpoints": [
                        "https://mainnet.sourceri.invalid/"
                    ]
                },
                {
                    "chainId": 42,
                    "endpoints": [
                        "https://kovan.sourceri.invalid"
                    ]
                }
            ]
        },
        {
            "name": "Floop",
            "chains": [
                {
                    "chainId": 42,
                    "endpoints": [
                        "https://kovan.floop.invalid"
                    ]
                }
            ]
        }
        ])
    );
}
