use std::fmt;
use std::sync::Arc;

use compleet::{
    source::{Source, Sources},
    sources,
};
use serde::{
    de::{Deserializer, MapAccess, Visitor},
    Deserialize,
};

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidSource {
    Lipsum,
    Lsp,
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Sources, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_map(SourcesVisitor)
}

struct SourcesVisitor;

impl<'de> Visitor<'de> for SourcesVisitor {
    type Value = Sources;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a list of completion sources")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut sources = match access.size_hint() {
            Some(len) => Vec::with_capacity(len),
            None => Vec::new(),
        };

        while let Some(source) = access.next_key::<ValidSource>()? {
            match source {
                ValidSource::Lipsum => {
                    let lipsum = access.next_value::<sources::Lipsum>()?;
                    if lipsum.enable {
                        sources.push(Arc::new(lipsum) as Arc<dyn Source>);
                    }
                },

                ValidSource::Lsp => {
                    let lsp = access.next_value::<sources::Lsp>()?;
                    if lsp.enable {
                        sources.push(Arc::new(lsp) as Arc<dyn Source>);
                    }
                },
            }
        }

        Ok(sources)
    }
}
