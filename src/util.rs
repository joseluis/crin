use chrono::{DateTime, Local, Utc};
use colored::*;

/// Format a DateTime as either YYYY-MM-DD or with a custom format
pub fn date_str(date: &DateTime<Utc>, format: &str) -> String {
    if format.is_empty() {
        date.with_timezone(&Local).date().naive_local().to_string()
    } else {
        date.with_timezone(&Local).format(format).to_string()
    }
}


/// Return the time passed since a date until now, surrounded by parenthesis
pub fn date_ago(date: &DateTime<Utc>, num_items: usize) -> String {
    format!("({})", timeago::Formatter::new()
        .num_items(num_items)
        .convert_chrono(
            date.with_timezone(&Local),
            Local::now())
        .to_string())
}

/// Return a string of words separated by commas,
/// optionally surrounding each word with a string.
pub fn commify(words: Vec<&str>, surround: &str,
    _color_word: Option<Color>, _color_comma: Option<Color>) -> String {
    let mut text = "".to_string();
    for word in words {
        text = format!("{t}{c} {s}{w}{s}", t = text,
            s = surround,
            // TODO: use the optional colors for words and commas
            c = ",",
            w = word
        );
    }
    return text[1..].trim().to_string(); // remove the leading comma
}
