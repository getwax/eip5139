use crate::errors::FetchError;

use std::future::Future;
use std::pin::Pin;

/// Used to retrieve resources required to resolve [`RpcProviders`](super::RpcProviders).
///
/// ## Example
///
/// ```
/// use eip5139::{Fetch, Source};
/// use eip5139::errors::FetchError;
///
/// use std::future::Future;
/// use std::io::{Error, ErrorKind};
/// use std::pin::Pin;
///
/// struct StaticFetch;
///
/// impl Fetch for StaticFetch {
///     fn fetch(&mut self, source: Source) ->
///         Pin<Box<dyn Future<Output = Result<String, FetchError>>>>
///     {
///         let future = async move {
///
///             // Resources can be specified by URI or by EIP-1577 hash.
///             let uri = match source {
///                 Source::Uri(u) => u,
///                 Source::Ens(_) => {
///                     // We're using `std::io::Error` for simplicity, but you
///                     // can use any error type.
///
///                     let error = Error::from(ErrorKind::Unsupported);
///                     return Err(FetchError::custom(error));
///                 },
///             };
///
///             if uri == "https://example.com/list.json" {
///                 Ok("...".to_string())
///             } else {
///                 let error = Error::from(ErrorKind::NotFound);
///                 Err(FetchError::custom(error))
///             }
///         };
///
///         Box::pin(future)
///     }
/// }
/// ```
pub trait Fetch {
    /// Retrieve the requested resource located by `source`.
    fn fetch(
        &mut self,
        source: crate::Source,
    ) -> Pin<Box<dyn Future<Output = Result<String, FetchError>>>>;
}
