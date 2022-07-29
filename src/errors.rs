//! Error types returned by functions in this crate.

use std::error::Error as StdError;
use std::fmt::{self, Write};

#[derive(Debug)]
pub(crate) enum InnerFetchError {
    Custom(Box<dyn StdError + 'static>),

    #[cfg(target_family = "wasm")]
    Js(wasm_bindgen::JsValue),
}

/// Error type for [`Fetch`](crate::Fetch) operations.
#[derive(Debug)]
pub struct FetchError {
    pub(crate) inner: InnerFetchError,
}

impl FetchError {
    /// Create a [`FetchError`] from a source [`Error`](std::error::Error).
    pub fn custom<E>(err: E) -> Self
    where
        E: 'static + StdError,
    {
        Self {
            inner: InnerFetchError::Custom(Box::new(err)),
        }
    }
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            InnerFetchError::Custom(ref e) => write!(f, "{}", e),

            #[cfg(target_family = "wasm")]
            InnerFetchError::Js(_) => write!(f, "an exception was thrown while fetching"),
        }
    }
}

impl StdError for FetchError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self.inner {
            InnerFetchError::Custom(ref e) => Some(Box::as_ref(e)),
            #[cfg(target_family = "wasm")]
            InnerFetchError::Js(_) => None,
        }
    }
}

/// Type representing JSON Schema validation errors.
#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for ValidationError {}

impl ValidationError {
    pub(crate) fn new<'a, I>(iter: I) -> Self
    where
        I: Iterator<Item = jsonschema::ValidationError<'a>>,
    {
        let mut message = String::new();
        for item in iter {
            writeln!(message, "{}", item).unwrap();
        }
        Self { message }
    }
}

/// Type representing errors encountered while parsing JSON.
#[derive(Debug)]
pub struct JsonError(pub(crate) serde_json::Error);

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for JsonError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.0)
    }
}

/// Type representing errors encountered while applying a JSON patch.
#[derive(Debug)]
pub struct PatchError(pub(crate) json_patch::PatchError);

impl fmt::Display for PatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl StdError for PatchError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.0)
    }
}

/// Type representing errors encountered while fetching an [`RpcProviders`](crate::RpcProviders).
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error encountered while fetching a resource.
    #[non_exhaustive]
    Fetch {
        /// The underlying cause of this error.
        source: FetchError,
    },

    /// A cycle was detected while resolving an extension list.
    #[non_exhaustive]
    Cycle {
        /// The source which was requested twice.
        duplicate: crate::Source,
    },

    /// An error encountered while parsing JSON.
    #[non_exhaustive]
    Json {
        /// The underlying cause of this error.
        source: JsonError,
    },

    /// An error encountered while applying a JSON patch.
    #[non_exhaustive]
    Patch {
        /// The underlying cause of this error.
        source: PatchError,
    },

    /// A schema validation error.
    #[non_exhaustive]
    Validation {
        /// The underlying cause of this error.
        source: ValidationError,
    },

    /// The chain of extension lists was too long.
    #[non_exhaustive]
    TooDeep {},

    /// An extension list requested a parent with an incompatible version.
    #[non_exhaustive]
    VersionMismatch {},
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fetch { source } => write!(f, "fetch failed: {}", source),
            Self::Cycle { duplicate } => write!(f, "cycle detected at: {:?}", duplicate),
            Self::Json { source } => write!(f, "parsing json failed: {}", source),
            Self::Patch { source } => write!(f, "applying patch failed: {}", source),
            Self::Validation { source } => write!(f, "schema validation failed: {}", source),
            Self::TooDeep { .. } => write!(f, "too many extension lists"),
            Self::VersionMismatch { .. } => write!(f, "parent list not compatible with child"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Fetch { source } => Some(source),
            Self::Cycle { .. } => None,
            Self::Json { source } => Some(source),
            Self::Patch { source } => Some(source),
            Self::Validation { source } => Some(source),
            Self::TooDeep { .. } => None,
            Self::VersionMismatch { .. } => None,
        }
    }
}

impl From<JsonError> for Error {
    fn from(source: JsonError) -> Self {
        Self::Json { source }
    }
}

impl From<FetchError> for Error {
    fn from(source: FetchError) -> Self {
        Self::Fetch { source }
    }
}

impl From<ValidationError> for Error {
    fn from(source: ValidationError) -> Self {
        Self::Validation { source }
    }
}

impl From<PatchError> for Error {
    fn from(source: PatchError) -> Self {
        Self::Patch { source }
    }
}
