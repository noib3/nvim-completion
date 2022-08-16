use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let words = File::open("words.txt")
        .map(|file| BufReader::new(file).lines())?
        .map(|line| format!("\"{}\",", line.unwrap()))
        .collect::<Vec<_>>()
        .join("\n");

    let words_rs = format!(
        r#"
/// The 30_000 most common English words in order of frequency taken from
/// [high-frequency-vocabulary].
///
/// [high-frequency-vocabulary]: https://github.com/derekchuank/high-frequency-vocabulary
pub(super) const WORDS: &[&str] = &[
    {words}
];
"#
    );

    OpenOptions::new()
        .create(true)
        .write(true)
        .open("src/words.rs")
        .and_then(|mut file| file.write_all(words_rs.as_bytes()))
}
