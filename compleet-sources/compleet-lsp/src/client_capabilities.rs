use nvim_oxi::{self as nvim, Dictionary, Object};

pub(super) fn client_capabilities(
    _settings: Object,
) -> nvim::Result<Dictionary> {
    Ok(Dictionary::from_iter([("textDocument", Object::nil())]))
}
