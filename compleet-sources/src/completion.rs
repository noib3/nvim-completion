use std::ops::Range;

// use rmpv::Value;
// use serde::{Deserialize, Serialize};

pub type Completions = Vec<CompletionItem>;

#[derive(Debug, Clone /* Serialize, Deserialize */)]
pub struct CompletionItem {
    /// The text to display in the details window as a vector of strings.
    pub details: Option<Vec<String>>,

    /// The formatted completion item as shown inside the completion menu.
    pub format: String,

    /// A vector or `(range, hl_group)` tuples, where each byte in the `range`
    /// is highlighted with the `hl_group` highlight group.
    pub hl_ranges: Vec<(Range<usize>, String)>,

    /// The number of bytes before the current cursor position that are
    /// matched by the completion item.
    pub matched_bytes: u32,

    /// The name of the source this completion comes from.
    pub source: &'static str,

    /// The text that will be inserted into the buffer if the completion is
    /// selected.
    pub text: String,
}

// /// Convert a `Completion` into a msgpack value.
// impl From<Completion> for Value {
//     fn from(c: Completion) -> Self {
//         Value::Array(vec![
//             {
//                 match c.details {
//                     Some(lines) => lines
//                         .into_iter()
//                         .map(|line| Value::from(line))
//                         .collect(),
//                     None => Value::Nil,
//                 }
//             },
//             Value::from(c.format),
//             Value::Nil,
//             Value::from(c.matched_bytes),
//             Value::from(c.source),
//             Value::from(c.text),
//         ])
//     }
// }

// /// Try to obtain a `Completion` from a msgpack value.
// impl TryFrom<Value> for Completion {
//     type Error = &'static str;

//     fn try_from(v: Value) -> Result<Self, Self::Error> {
//         match v {
//             Value::Array(v) => {
//                 if v.len() != 6 {
//                     return Err("was expecting 6 elements");
//                 }

//                 let mut iter = v.into_iter();

//                 let details = match iter.next() {
//                     Some(Value::Nil) => None,
//                     Some(Value::Array(lines)) => Some(
//                         lines
//                             .into_iter()
//                             .flat_map(|line| String::try_from(line))
//                             .collect::<Vec<String>>(),
//                     ),

//                     _ => {
//                         return Err("details should be either a nil or a \
//                                     list of strings")
//                     },
//                 };

//                 let format = match iter.next() {
//                     Some(Value::String(s)) if s.is_str() => {
//                         s.into_str().expect("already checked valid utf8")
//                     },
//                     _ => return Err("format should be a string"),
//                 };

//                 let hl_ranges = Vec::new();
//                 iter.next();

//                 let matched_bytes = match iter.next() {
//                     Some(Value::Integer(n)) if n.is_u64() => {
//                         n.as_u64().expect("already checked that it's a u64")
//                             as u32
//                     },

//                     _ => return Err("matched_bytes isn't valid"),
//                 };

//                 let source = match iter.next() {
//                     Some(Value::String(s)) if s.is_str() => {
//                         s.into_str().expect("already checked valid utf8")
//                     },
//                     _ => return Err("source should be a string"),
//                 };

//                 let text = match iter.next() {
//                     Some(Value::String(s)) if s.is_str() => {
//                         s.into_str().expect("already checked valid utf8")
//                     },
//                     _ => return Err("text should be a string"),
//                 };

//                 Ok(Completion {
//                     details,
//                     format,
//                     hl_ranges,
//                     matched_bytes,
//                     source,
//                     text,
//                 })
//             },

//             _ => Err("was expecting an array"),
//         }
//     }
// }
