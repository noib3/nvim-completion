use serde::Deserialize;

// // TODO: if I could deserialize into this it would be awesome!
// #[derive(Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub enum CompletionSource {
//     Lipsum(crate::completion::sources::Lipsum),
// }

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionSource {
    Lipsum,
    Lsp,
}
