/// Returns the two lines of a verse.
pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", first_line(n), second_line(n))
}

/// Returns the first line of a verse.
fn first_line(n: u32) -> String {
    format!("{}, {}.", on_the_wall(n, false), bottles_of_beer(n, true))
}

/// Returns the second line of a verse.
fn second_line(n: u32) -> String {
    format!(
        "{}, {}.",
        match n {
            0 => "Go to the store and buy some more".to_string(),
            _ => format!(
                "Take {} down and pass it around",
                if n == 1 { "it" } else { "one" }
            ),
        },
        on_the_wall(if n == 0 { 99 } else { n - 1 }, true)
    )
}

/// Returns 'n bottle(s) of beer'.
///
/// The string is capitalised if lowercase is false.
fn bottles_of_beer(n: u32, lowercase: bool) -> String {
    format!(
        "{} bottle{} of beer",
        match n {
            0 => format!("{}o more", if lowercase { 'n' } else { 'N' }),
            x => x.to_string(),
        },
        if n == 1 { "" } else { "s" }
    )
}

/// Returns 'n bottle(s) of beer on a wall'.
///
/// The string is capitalised if lowercase is false.
fn on_the_wall(n: u32, lowercase: bool) -> String {
    format!("{} on the wall", bottles_of_beer(n, lowercase))
}

/// Returns the verses in the start to end range, inclusively.
pub fn sing(start: u32, end: u32) -> String {
    let mut verses = String::new();

    for i in (end..=start).rev() {
        verses.push_str(&verse(i));

        if i > end {
            verses.push_str("\n");
        }
    }

    verses
}
