use std::iter;

const CHUNK_LENGTH: usize = 5;

fn mirror_letters(text: &str) -> String {
    const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

    text.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if let Some(i) = ALPHABET.find(c) {
                ALPHABET.chars().nth(ALPHABET.len() - 1 - i).unwrap()
            } else {
                c
            }
        })
        .collect()
}

fn split_into_chunks(text: &str) -> String {
    text.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && i % CHUNK_LENGTH == 0 {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(iter::once(c))
        })
        .collect()
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    split_into_chunks(&mirror_letters(plain))
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    mirror_letters(cipher)
}
