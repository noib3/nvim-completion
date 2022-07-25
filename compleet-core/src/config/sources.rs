use std::cell::RefCell;
use std::fmt;

use once_cell::unsync::Lazy;
use serde::{
    de::{Deserializer, Error, MapAccess, Visitor},
    Deserialize,
};

use super::SourcesConfig;

thread_local! {
    pub(crate) static SOURCE_NAMES: Lazy<RefCell<Option<Vec<&'static str>>>> =
        Lazy::new(|| RefCell::new(Some(Vec::new())));
}

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

            // Leaking the source names to get a `'static` reference needed by
            // `de::Error::unknown_variant`.
            let names: &'static [&'static str] = SOURCE_NAMES
                .with(|names| names.borrow_mut().take().unwrap())
                .leak();

            let mut deser_sources = || {
                while let Some(name) = map.next_key::<String>()? {
                    if !names.contains(&name.as_ref()) {
                        return Err(M::Error::unknown_variant(&name, names));
                    }

                    let config = map.next_value::<EnableSource>()?;
                    configs.insert(name, config.enable);
                }

                Ok(())
            };

            deser_sources().map(|_| configs).map_err(|err| {
                // If the deserialization failed we repopulate `SOURCE_NAMES`
                // to allow [`setup`](crate::setup::setup) to be called again.
                SOURCE_NAMES.with(move |global| {
                    let vec = unsafe {
                        Vec::from_raw_parts(
                            names.as_ptr() as *mut &'static str,
                            names.len(),
                            names.len(),
                        )
                    };
                    *global.borrow_mut() = Some(vec);
                });

                err
            })
        }
    }

    deserializer.deserialize_map(SourcesVisitor)
}
