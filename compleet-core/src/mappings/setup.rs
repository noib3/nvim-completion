use nvim_oxi::Object;

use crate::Client;

pub(super) enum IdentifyCompletion {
    /// A specific completion.
    ByIndex(usize),

    /// A positive or negative offset from the currently selected completion.
    FromSelected(isize),
}

pub(crate) fn setup(
    client: &Client,
) -> impl IntoIterator<Item = (&'static str, Object)> {
    let accept_first = client.create_fn(|client, ()| {
        super::accept_completion(client, IdentifyCompletion::ByIndex(0))
    });

    let accept_selected = client.create_fn(|client, ()| {
        super::accept_completion(client, IdentifyCompletion::FromSelected(0))
    });

    let scroll_details = client.create_fn(super::scroll_details);

    let select_next = client.create_fn(|client, ()| {
        super::select_completion(client, IdentifyCompletion::FromSelected(1))
    });

    let select_prev = client.create_fn(|client, ()| {
        super::select_completion(client, IdentifyCompletion::FromSelected(-1))
    });

    let show = client.create_fn(|client, ()| super::show_completions(client));

    [
        ("accept_first", Object::from(accept_first)),
        ("accept_selected", Object::from(accept_selected)),
        ("scroll_details", Object::from(scroll_details)),
        ("select_next", Object::from(select_next)),
        ("select_prev", Object::from(select_prev)),
        ("show", Object::from(show)),
    ]
}
