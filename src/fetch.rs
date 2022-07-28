use crate::errors::FetchError;

use std::future::Future;
use std::pin::Pin;

pub trait Fetch {
    fn fetch(
        &mut self,
        uri: crate::Source,
    ) -> Pin<Box<dyn Future<Output = Result<String, FetchError>>>>;
}
