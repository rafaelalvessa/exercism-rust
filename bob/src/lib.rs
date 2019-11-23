const QUESTION_REPLY: &str = "Sure.";
const YELL_REPLY: &str = "Whoa, chill out!";
const YELL_QUESTION_REPLY: &str = "Calm down, I know what I'm doing!";
const EMPTY_REPLY: &str = "Fine. Be that way!";
const DEFAULT_REPLY: &str = "Whatever.";

pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = message.ends_with('?');
    let is_yelling =
        message.chars().any(|char| char.is_alphabetic()) && message.to_uppercase() == message;

    if message.is_empty() {
        return EMPTY_REPLY;
    }

    match (is_question, is_yelling) {
        (true, true) => YELL_QUESTION_REPLY,
        (true, false) => QUESTION_REPLY,
        (false, true) => YELL_REPLY,
        (false, false) => DEFAULT_REPLY,
    }
}
