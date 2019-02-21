pub fn reply(message: &str) -> &str {
    let trim_msg = message.trim();
    let empty = trim_msg.is_empty();
    let yell = trim_msg.chars().any(char::is_alphabetic) && !trim_msg.chars().any(char::is_lowercase);
    let question = trim_msg.ends_with('?');
    if empty {
        "Fine. Be that way!"
    } else if question && yell {
        "Calm down, I know what I'm doing!"
    } else if question && !yell {
        "Sure."
    } else if yell {
        "Whoa, chill out!"
    } else {
        "Whatever."
    }
}
