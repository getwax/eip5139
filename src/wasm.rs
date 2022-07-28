use crate::errors::FetchError;
use crate::{fetch, RpcProviders};

use js_sys::{Date, Function, JsString, Promise};

use std::future::Future;
use std::pin::Pin;

use wasm_bindgen::prelude::*;

use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type Provider = {};
export type Version = {};
"#;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "(uri: string) => Promise<string>")]
    pub type FetchFn;

    #[wasm_bindgen(typescript_type = "Provider")]
    pub type Provider;

    #[wasm_bindgen(typescript_type = "Version")]
    pub type Version;
}

#[wasm_bindgen]
impl RpcProviders {
    #[doc(hidden)]
    #[wasm_bindgen(js_name = "fetch")]
    pub async fn fetch_js(fetch: FetchFn, uri: String) -> Result<RpcProviders, JsValue> {
        let value: JsValue = fetch.into();
        let fetch = JsFetch(value.into());
        let result = Self::fetch(fetch, uri).await?;
        Ok(result)
    }

    #[doc(hidden)]
    #[wasm_bindgen(getter, js_name = version)]
    pub fn version_js(&self) -> Version {
        todo!()
    }

    #[doc(hidden)]
    #[wasm_bindgen(setter, js_name = version)]
    pub fn set_version_js(&self, version: Version) {
        todo!()
    }

    #[doc(hidden)]
    #[wasm_bindgen(getter, js_name = providers)]
    pub fn providers_js(&self) -> Vec<Provider> {
        todo!()
    }

    #[doc(hidden)]
    #[wasm_bindgen(setter, js_name = providers)]
    pub fn set_providers_js(&self, provider: Vec<Provider>) {
        todo!()
    }
}

#[derive(Debug)]
pub(crate) struct JsFetch(pub Function);

impl fetch::Fetch for JsFetch {
    fn fetch(&mut self, uri: &str) -> Pin<Box<dyn Future<Output = Result<String, FetchError>>>> {
        let fetch = self.0.clone();
        let uri = JsValue::from_str(uri);

        let future = async move {
            let this = JsValue::null();
            let promise = Promise::from(fetch.call1(&this, &uri)?);

            let result = JsFuture::from(promise).await?;
            let string = JsString::from(result);

            Ok(string.as_string().unwrap())
        };

        Box::pin(future)
    }
}

impl From<JsValue> for FetchError {
    fn from(value: JsValue) -> Self {
        todo!()
    }
}

impl From<crate::Error> for JsValue {
    fn from(value: crate::Error) -> Self {
        todo!()
    }
}
