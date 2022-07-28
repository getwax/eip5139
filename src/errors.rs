use std::fmt::Write;

#[derive(Debug)]
pub struct FetchError {}

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

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

#[derive(Debug)]
pub struct JsonError(pub(crate) serde_json::Error);

#[derive(Debug)]
pub struct PatchError(pub(crate) json_patch::PatchError);

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Fetch { source: FetchError },
    Cycle { duplicate: String },
    Json { source: JsonError },
    Patch { source: PatchError },
    Validation { source: ValidationError },
    TooDeep,
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
