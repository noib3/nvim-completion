use std::fmt;
use std::sync::Arc;

use serde::de::{Deserializer, MapAccess, Visitor};

use super::CompletionSource;
use crate::completion::{self, sources::*};
use crate::state::Sources;

struct SourcesVisitor;

impl<'de> Visitor<'de> for SourcesVisitor {
    type Value = Sources;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a list of completion sources")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut sources = Vec::new();

        // TODO: create the remaining completion sources.
        while let Some(source) = access.next_key::<CompletionSource>()? {
            match source {
                CompletionSource::Lipsum => {
                    let lipsum = access.next_value::<Lipsum>()?;
                    if lipsum.enable {
                        sources.push(Arc::new(lipsum)
                            as Arc<dyn completion::CompletionSource>);
                    }
                },
            }
        }

        Ok(sources)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Sources, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(SourcesVisitor)
}
