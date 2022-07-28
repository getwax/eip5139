pub mod errors;
pub mod fetch;
mod resolve;
#[cfg(target_family = "wasm")]
mod wasm;

pub use self::errors::Error;

use serde::{Deserialize, Serialize};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProviderChain {
    #[serde(rename = "chainId")]
    pub chain_id: u64,
    pub endpoints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Provider {
    pub name: String,
    pub logo: Option<String>,
    pub priority: Option<u32>,
    pub chains: Vec<ProviderChain>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[non_exhaustive]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,

    #[serde(
        default,
        rename = "pre-release",
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_release: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Default)]
pub struct RpcProviders {
    pub name: String,
    pub logo: Option<String>,
    pub timestamp: String,

    version: Version,
    providers: Vec<Provider>,
}

impl RpcProviders {
    pub async fn fetch<F, S>(mut fetch: F, uri: S) -> Result<Self, Error>
    where
        F: fetch::Fetch,
        S: AsRef<str>,
    {
        resolve::resolve(&mut fetch, uri.as_ref()).await
    }

    pub fn providers(&self) -> &[Provider] {
        &self.providers
    }

    pub fn providers_mut(&mut self) -> &mut [Provider] {
        &mut self.providers
    }

    pub fn set_providers(&mut self, p: Vec<Provider>) {
        self.providers = p;
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }
}
