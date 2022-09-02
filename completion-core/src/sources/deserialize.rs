use std::fmt;

use serde::de::{Deserializer, Error, MapAccess, Visitor};

use super::{SourceConfig, SourceConfigs};
use crate::setup::registered_source_names;

struct SourcesVisitor;

impl<'de> Visitor<'de> for SourcesVisitor {
    type Value = SourceConfigs;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("a table of completion source configurations")
    }

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        // TODO: filter_enabled has the same information that we have here. Can
        // we do better?

        let mut configs =
            SourceConfigs::with_capacity(map.size_hint().unwrap_or(0));

        let names = registered_source_names();

        while let Some(name) = map.next_key::<String>()? {
            if !names.contains(&&*name) {
                // FIXME: this is a memory leak
                return Err(M::Error::unknown_variant(&name, names.leak()));
            }

            let config = map.next_value::<SourceConfig>()?;
            configs.insert(name, config);
        }

        Ok(configs)
    }
}

pub(crate) fn deserialize<'de, D>(d: D) -> Result<SourceConfigs, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_map(SourcesVisitor)
}
