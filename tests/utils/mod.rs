use eip5139::errors::FetchError;
use eip5139::Source;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

pub struct Fetch {
    contents: HashMap<Source, String>,
}

impl Fetch {
    pub fn with_one<O>(one: O) -> Self
    where
        O: Into<String>,
    {
        let mut contents = HashMap::new();
        contents.insert(Source::Uri("file://one".into()), one.into());
        Self { contents }
    }

    pub fn with_two<O, T>(one: O, two: T) -> Self
    where
        O: Into<String>,
        T: Into<String>,
    {
        let mut contents = HashMap::new();
        contents.insert(Source::Uri("file://one".into()), one.into());
        contents.insert(Source::Uri("file://two".into()), two.into());
        Self { contents }
    }
}

impl eip5139::fetch::Fetch for Fetch {
    fn fetch(
        &mut self,
        source: Source,
    ) -> Pin<Box<dyn Future<Output = Result<String, FetchError>>>> {
        let output = Ok(self.contents[&source].to_owned());
        Box::pin(async move { output })
    }
}
