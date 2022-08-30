use nvim_oxi::{object, Object, ObjectKind};
use serde::Deserialize;

use crate::completions::CompletionConfig;
use crate::sources::SourceConfigs;
use crate::ui::UiConfig;

#[derive(Default, Deserialize)]
pub(crate) struct Config {
    #[serde(default)]
    pub(crate) ui: UiConfig,

    #[serde(default)]
    pub(crate) completion: CompletionConfig,

    #[serde(default)]
    pub(crate) sources: SourceConfigs,
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
