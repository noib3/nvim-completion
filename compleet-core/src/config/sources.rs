use std::fmt;

use serde::{
    de::{Deserializer, Error, MapAccess, Visitor},
    Deserialize,
};

const SOURCE_NAMES: [&str; 0] = [];

use super::SourcesConfig;

#[derive(Deserialize)]
struct EnableSource {
    enable: bool,
}

pub(super) fn deserialize<'de, D>(
    deserializer: D,
) -> Result<SourcesConfig, D::Error>
where
    D: Deserializer<'de>,
{
    struct SourcesVisitor;

    impl<'de> Visitor<'de> for SourcesVisitor {
        type Value = SourcesConfig;

        fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.write_str("a table of completion sources")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut configs = SourcesConfig::with_capacity(
                map.size_hint().unwrap_or_default(),
            );

            while let Some(name) = map.next_key::<String>()? {
                if !SOURCE_NAMES.contains(&name.as_ref()) {
                    return Err(M::Error::unknown_variant(
                        &name,
                        &SOURCE_NAMES,
                    ));
                }

                let config = map.next_value::<EnableSource>()?;
                configs.insert(name, config.enable);
            }

            Ok(configs)
        }
    }

    deserializer.deserialize_map(SourcesVisitor)
}
