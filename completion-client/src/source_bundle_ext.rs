use completion_types::{SourceBundle, SourceEnable};
use nvim_oxi as nvim;

use crate::Result;

pub(crate) trait SourceBundleExt {
    fn api(&self) -> nvim::Object;

    fn is_initialized(&self) -> bool;

    fn set_config(&mut self, config: nvim::Object) -> Result<()>;

    fn set_enable(&mut self, enable: SourceEnable);
}

impl SourceBundleExt for SourceBundle {
    #[inline]
    fn api(&self) -> nvim::Object {
        self.source.api()
    }

    #[inline]
    fn is_initialized(&self) -> bool {
        self.config.is_some() && self.enable.is_some()
    }

    #[inline]
    fn set_config(&mut self, config: nvim::Object) -> Result<()> {
        self.config = Some(self.source.deserialize_config(config)?);
        Ok(())
    }

    #[inline]
    fn set_enable(&mut self, enable: SourceEnable) {
        self.enable = Some(enable);
    }
}
