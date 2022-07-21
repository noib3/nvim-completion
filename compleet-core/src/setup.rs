use nvim_oxi::{object, Object, ObjectKind};

use crate::{autocmds, commands, hlgroups, mappings};
use crate::{Config, Error, Result, State};

pub(crate) fn setup(state: &mut State, preferences: Object) -> Result<()> {
    if state.already_setup() {
        return Err(Error::AlreadySetup);
    }

    // Set the highlight groups *before* deserializing the preferences so that
    // error messages will be displayed with the right colors.
    hlgroups::setup()?;

    let _config = match preferences.kind() {
        ObjectKind::Nil => Config::default(),

        _ => {
            let deserializer = object::Deserializer::new(preferences);
            serde_path_to_error::deserialize::<_, Config>(deserializer)?
        },
    };

    autocmds::setup()?;
    commands::setup()?;
    mappings::setup()?;

    // state.did_setup();

    Ok(())
}
