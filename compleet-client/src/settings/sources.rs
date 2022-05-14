use std::fmt;
use std::sync::Arc;

use serde::de::{Deserializer, MapAccess, Visitor};
use sources::prelude::{CompletionSource, Sources, ValidSource};
use sources::*;
use tokio::sync::Mutex;

pub fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Sources, D::Error> {
    deserializer.deserialize_map(SourcesVisitor)
}

struct SourcesVisitor;

impl<'de> Visitor<'de> for SourcesVisitor {
    type Value = Sources;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a list of completion sources")
    }

    fn visit_map<M: MapAccess<'de>>(
        self,
        mut access: M,
    ) -> Result<Self::Value, M::Error> {
        let mut sources =
            Vec::with_capacity(access.size_hint().unwrap_or_default());

        use ValidSource::*;
        while let Some(name) = access.next_key::<ValidSource>()? {
            match name {
                Lipsum => {
                    let config =
                        access.next_value::<lipsum::LipsumConfig>()?;
                    if config.enable {
                        let lipsum =
                            Arc::new(Mutex::new(lipsum::Lipsum::from(config)));
                        sources
                            .push(lipsum as Arc<Mutex<dyn CompletionSource>>);
                    }
                },

                Lsp => {
                    let config = access.next_value::<lsp::LspConfig>()?;
                    if config.enable {
                        let lsp = Arc::new(Mutex::new(lsp::Lsp::from(config)));
                        sources.push(lsp as Arc<Mutex<dyn CompletionSource>>);
                    }
                },
            }
        }

        Ok(sources)
    }
}
