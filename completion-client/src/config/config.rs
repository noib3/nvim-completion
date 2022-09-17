use nvim_oxi::{object, Object, ObjectKind};
use serde::Deserialize;

use super::{CompletionConfig, SourcesConfig};
use crate::ui::UiConfig;

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) ui: UiConfig,

    #[serde(default)]
    pub(crate) completion: CompletionConfig,

    #[serde(default, deserialize_with = "super::sources_config::deserialize")]
    pub(crate) sources: SourcesConfig,
}

impl TryFrom<Object> for Config {
    type Error = crate::Error;

    fn try_from(preferences: Object) -> Result<Self, Self::Error> {
        match preferences.kind() {
            ObjectKind::Nil => Ok(Self::default()),

            _ => {
                let deserializer = object::Deserializer::new(preferences);
                serde_path_to_error::deserialize::<_, Self>(deserializer)
                    .map_err(Into::into)
            },
        }
    }
}
