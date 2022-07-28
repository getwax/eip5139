use eip5139::errors::*;
use eip5139::RpcProviders;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

use futures_executor::LocalPool;

#[cfg(target_family = "wasm")]
use wasm_bindgen_test::wasm_bindgen_test;

struct Fetch {
    contents: HashMap<String, String>,
}

impl Fetch {
    fn with_two<O, T>(one: O, two: T) -> Self
    where
        O: Into<String>,
        T: Into<String>,
    {
        let mut contents = HashMap::new();
        contents.insert("file://one".into(), one.into());
        contents.insert("file://two".into(), two.into());
        Self { contents }
    }
}

impl eip5139::fetch::Fetch for Fetch {
    fn fetch(&mut self, uri: &str) -> Pin<Box<dyn Future<Output = Result<String, FetchError>>>> {
        let output = Ok(self.contents[uri].to_owned());
        Box::pin(async move { output })
    }
}

#[test]
#[cfg_attr(target_family = "wasm", wasm_bindgen_test)]
fn valid_empty_add_one() {
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

    let two = r#"{
  "name": "Extension List",
  "version": {
    "major": 10,
    "minor": 1,
    "patch": 0,
    "build": "wWw"
  },
  "timestamp": "2024-08-08T00:00:00.0Z",
  "logo": "https://mylist2.invalid/logo.png",
  "extends": {
    "from": "file://one",
    "version": {
      "major": 0,
      "minor": 1,
      "patch": 0
    }
  },
  "changes": [
    {
        "op": "add",
        "path": "/some-key",
        "value": {
            "name": "Frustrata",
            "chains": [
                {
                    "chainId": 1,
                    "endpoints": [
                        "https://mainnet1.frustrata.invalid/",
                        "https://mainnet2.frustrana.invalid/"
                    ]
                },
                {
                    "chainId": 3,
                    "endpoints": [
                        "https://ropsten.frustrana.invalid/"
                    ]
                }
            ]
        }
    },
    {
        "op": "add",
        "path": "/other-key",
        "value": {
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
        }
    }
]
}"#;

    let fetch = Fetch::with_two(one, two);
    let mut pool = LocalPool::new();
    let list = pool.run_until(RpcProviders::fetch(fetch, "file://two"))
        .unwrap();

    assert_eq!(list.name, "Extension List");

    assert_eq!(list.version().major, 10);
    assert_eq!(list.version().minor, 1);
    assert_eq!(list.version().patch, 0);
    assert_eq!(list.version().build, Some("wWw".into()));

    todo!()
}
