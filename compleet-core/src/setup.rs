use nvim_oxi::{object, Object, ObjectKind};

use crate::{Config, Result, State};

pub(crate) fn setup(_state: &mut State, preferences: Object) -> Result<()> {
    let _config = match preferences.kind() {
        ObjectKind::Nil => Config::default(),

        _ => {
            let deserializer = object::Deserializer::new(preferences);
            serde_path_to_error::deserialize::<_, Config>(deserializer)?
        },
    };

    Ok(())
}
