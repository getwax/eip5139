use crate::errors::{Error, JsonError, PatchError, ValidationError};
use crate::fetch::Fetch;
use crate::{RpcProviders, Source, Version};

use jsonschema::JSONSchema;

use lazy_static::lazy_static;

use semver::Prerelease;
use serde::{Deserialize, Serialize};

use serde_json::Value;

use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref SCHEMA: JSONSchema = {
        let raw = include_str!("schema.json");
        let json = serde_json::from_str(raw).unwrap();
        JSONSchema::compile(&json).unwrap()
    };
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "mode")]
enum Mode {
    #[serde(rename = "=")]
    Exact {
        #[serde(
            default,
            rename = "preRelease",
            skip_serializing_if = "Option::is_none"
        )]
        pre_release: Option<String>,
    },
    #[serde(rename = "^")]
    Caret,
}

impl Default for Mode {
    fn default() -> Self {
        Self::Caret
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct VersionRange {
    major: u64,
    minor: u64,
    patch: u64,
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    mode: Option<Mode>,
}

impl VersionRange {
    fn into_semver(self) -> semver::VersionReq {
        let op;
        let pre;

        match self.mode {
            None | Some(Mode::Caret) => {
                op = semver::Op::Caret;
                pre = None;
            }
            Some(Mode::Exact { pre_release }) => {
                op = semver::Op::Exact;
                pre = pre_release.map(|p| Prerelease::new(&p).unwrap());
            }
        };

        semver::VersionReq {
            comparators: vec![semver::Comparator {
                op,
                pre: pre.unwrap_or(Prerelease::EMPTY),
                major: self.major,
                minor: Some(self.minor),
                patch: Some(self.patch),
            }],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Extends {
    version: VersionRange,
    #[serde(flatten)]
    from: Source,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Kind {
    Root { providers: Value },
    Extension { extends: Extends, changes: Value },
}

#[derive(Debug, Serialize, Deserialize)]
struct List {
    name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    logo: Option<String>,
    version: Version,
    timestamp: String,

    #[serde(flatten)]
    kind: Kind,
}

impl List {
    fn check_version(&self, parent: &Self) -> Result<(), Error> {
        let version_req = match self.kind {
            Kind::Extension { ref extends, .. } => extends.version.clone().into_semver(),
            _ => panic!("only extension lists need a version check"),
        };

        let parent_version = parent.version.clone().into_semver();

        if version_req.matches(&parent_version) {
            Ok(())
        } else {
            Err(Error::VersionMismatch {})
        }
    }
}

pub async fn resolve(fetch: &mut dyn Fetch, source: Source) -> Result<RpcProviders, Error> {
    let mut seen = HashSet::new();

    let mut stack = Vec::<List>::new();
    let mut current = source;

    loop {
        // Ensure that this `from` has not been seen before.
        if let Some(duplicate) = seen.replace(current.clone()) {
            return Err(Error::Cycle { duplicate });
        }

        // Retrieve the parent list.
        let text = fetch.fetch(current).await?;
        let json = serde_json::from_str(&text).map_err(JsonError)?;

        // Verify that the parent list is valid according to the JSON schema.
        SCHEMA.validate(&json).map_err(ValidationError::new)?;

        // Parse the list.
        let parent: List = serde_json::from_value(json).unwrap();

        // Ensure that the parent list is version compatible.
        if let Some(child) = stack.last() {
            child.check_version(&parent)?;
        }

        stack.push(parent);

        // Is the current list an extension list?
        current = match &stack.last().unwrap().kind {
            Kind::Extension { extends, .. } => extends.from.clone(),
            Kind::Root { .. } => break,
        };

        if stack.len() > 10 {
            return Err(Error::TooDeep {});
        }
    }

    let mut output = stack.pop().unwrap();

    for mut list in stack.into_iter() {
        let patch = match list.kind {
            Kind::Extension { changes, .. } => json_patch::from_value(changes).unwrap(),
            _ => unreachable!(),
        };

        list.kind = output.kind;
        output = list;

        let providers = match output.kind {
            Kind::Root { ref mut providers } => providers,
            _ => unreachable!(),
        };

        json_patch::patch(providers, &patch).map_err(PatchError)?;

        // Verify that the list is valid according to the JSON schema.
        let json = serde_json::to_value(&output).unwrap();
        println!("{:#?}", json);
        SCHEMA.validate(&json).map_err(ValidationError::new)?;
    }

    let providers = match output.kind {
        Kind::Root { providers } => providers,
        _ => unreachable!(),
    };

    let providers = serde_json::from_value::<HashMap<String, Value>>(providers)
        .unwrap()
        .into_iter()
        .map(|(_, v)| serde_json::from_value(v).unwrap())
        .collect();

    Ok(RpcProviders {
        name: output.name,
        logo: output.logo,
        version: output.version,
        timestamp: output.timestamp,
        providers,
    })
}
