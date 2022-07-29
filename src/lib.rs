//! Remote Procedure Call Provider Lists
//!
//! [EIP-5139] is a standard for collecting and managing RPC providers for use
//! in Ethereum wallets.
//!
//! This crate is an implementation of [EIP-5139] for Rust and JavaScript.
//!
//! [EIP-5139]: https://eips.ethereum.org/EIPS/eip-5139
#![deny(unsafe_code)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod errors;
mod fetch;
mod resolve;
#[cfg(target_family = "wasm")]
mod wasm;

pub use self::errors::Error;
pub use self::fetch::Fetch;

use semver::{BuildMetadata, Prerelease};

use serde::{Deserialize, Serialize};

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

/// Endpoints supported for a particular chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProviderChain {
    /// The identifier for a particular chain (eg. `1` for Ethereum mainnet.)
    #[serde(rename = "chainId")]
    pub chain_id: u64,

    /// Addresses serving the Ethereum JSON RPC interface.
    pub endpoints: Vec<String>,
}

/// A single entity that serves the Ethereum JSON RPC interface for one or more chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Provider {
    /// Human-readable name of the provider.
    pub name: String,

    /// An optional URI where a logo for this provider can be found.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo: Option<String>,

    /// A priority value for this provider, where zero is the highest priority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<u32>,

    /// The chains this provider supports.
    pub chains: Vec<ProviderChain>,
}

/// [Semantic version] of an [`RpcProviders`] list.
///
/// [Semantic version]: https://semver.org/
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
#[non_exhaustive]
pub struct Version {
    /// The major version indicates backwards compatibility.
    pub major: u64,

    /// The minor version indicates feature availability.
    pub minor: u64,

    /// The patch version indicates bug fixes.
    pub patch: u64,

    /// The pre-release string indicates that this version isn't released yet.
    #[serde(
        default,
        rename = "preRelease",
        skip_serializing_if = "Option::is_none"
    )]
    pub pre_release: Option<String>,

    /// Build metadata.
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

/// The location where a list can be retrieved from.
#[derive(Debug, Serialize, Deserialize, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum Source {
    /// Fetch using [EIP-1577] from the given ENS name.
    ///
    /// [EIP-1577]: https://eips.ethereum.org/EIPS/eip-1577
    #[serde(rename = "ens")]
    Ens(String),

    /// Fetch from the given URI, usually using HTTPS.
    #[serde(rename = "uri")]
    Uri(String),
}

/// A resolved EIP-5139 provider list.
///
/// Retrieve a list using [`fetch`](RpcProviders::fetch).
#[cfg_attr(target_family = "wasm", wasm_bindgen(getter_with_clone))]
#[derive(Debug, Default)]
pub struct RpcProviders {
    /// Human-readable name of this list.
    pub name: String,

    /// An optional URI where a logo for this list can be found.
    pub logo: Option<String>,

    /// The date/time this list was created (in RFC 3339 format.)
    pub timestamp: String,

    version: Version,
    providers: Vec<Provider>,
}

impl RpcProviders {
    /// Retrieve the list from `source` and resolve any extension lists.
    pub async fn fetch<F>(mut fetch: F, source: Source) -> Result<Self, Error>
    where
        F: fetch::Fetch,
    {
        resolve::resolve(&mut fetch, source).await
    }

    /// Get the providers contained in this list.
    pub fn providers(&self) -> &[Provider] {
        &self.providers
    }

    /// Get a mutable reference to the providers contained in this list.
    pub fn providers_mut(&mut self) -> &mut [Provider] {
        &mut self.providers
    }

    /// Set the providers contained in this list.
    pub fn set_providers(&mut self, p: Vec<Provider>) {
        self.providers = p;
    }

    /// Get the version of this list.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// Get a mutable reference to the version of this list.
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.version
    }

    /// Set the version of this list.
    pub fn set_version(&mut self, version: Version) {
        self.version = version;
    }
}
