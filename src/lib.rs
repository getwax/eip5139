pub mod errors;
pub mod fetch;
mod resolve;
#[cfg(target_family = "wasm")]
mod wasm;

pub use self::errors::Error;

use semver::{BuildMetadata, Prerelease};

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,
    pub chains: Vec<ProviderChain>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[non_exhaustive]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,

    #[serde(
        default,
        rename = "preRelease",
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_release: Option<String>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
}

impl Version {
    fn into_semver(self) -> semver::Version {
        semver::Version {
            major: self.major,
            minor: self.minor,
            patch: self.patch,
            pre: self
                .pre_release
                .map(|p| Prerelease::new(&p).unwrap())
                .unwrap_or(Prerelease::EMPTY),
            build: self
                .build
                .map(|b| BuildMetadata::new(&b).unwrap())
                .unwrap_or(BuildMetadata::EMPTY),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Source {
    #[serde(rename = "ens")]
    Ens(String),

    #[serde(rename = "uri")]
    Uri(String),
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
    pub async fn fetch<F>(mut fetch: F, source: Source) -> Result<Self, Error>
    where
        F: fetch::Fetch,
    {
        resolve::resolve(&mut fetch, source).await
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
