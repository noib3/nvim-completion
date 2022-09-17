use std::collections::HashMap;
use std::fmt;

use completion_types::SourceEnable;
use nvim_oxi::Object;
use serde::de::{Deserializer, Error, MapAccess, Visitor};
use serde::Deserialize;

use crate::setup::registered_source_names;

pub(crate) type SourcesConfig = HashMap<String, SourceConfig>;

#[derive(Deserialize)]
pub(crate) struct SourceConfig {
    pub(crate) enable: SourceEnable,

    #[serde(flatten)]
    pub(crate) rest: Object,
}

pub(super) fn deserialize<'de, D>(d: D) -> Result<SourcesConfig, D::Error>
where
    D: Deserializer<'de>,
{
    struct SourcesVisitor;

    impl<'de> Visitor<'de> for SourcesVisitor {
        type Value = SourcesConfig;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a table of completion source configurations")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            // TODO: filter_enabled has the same information that we have here.
            // Can we do better?

            let mut configs =
                SourcesConfig::with_capacity(map.size_hint().unwrap_or(0));

            let names = registered_source_names();

            while let Some(name) = map.next_key::<String>()? {
                if !names.contains(&&*name) {
                    // FIXME: this is a memory leak
                    return Err(M::Error::unknown_variant(
                        &name,
                        names.leak(),
                    ));
                }

                let config = map.next_value::<SourceConfig>()?;
                configs.insert(name, config);
            }

            Ok(configs)
        }
    }

    d.deserialize_map(SourcesVisitor)
}
