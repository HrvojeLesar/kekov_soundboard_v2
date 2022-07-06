pub mod guild;
pub mod guild_file;
pub mod ids;
pub mod sound_file;
pub mod state;
pub mod user;

pub fn postgres_like_escape(input: String) -> String {
    for (i, ch) in input.chars().enumerate() {
        if characters_to_escape(ch).is_some() {
            let mut escaped_string = String::with_capacity(input.len());

            escaped_string.push_str(&input[..i]);

            for ch in input[i..].chars() {
                match characters_to_escape(ch) {
                    Some(c) => escaped_string.push_str(c),
                    None => escaped_string.push(ch),
                }
            }
            return escaped_string;
        }
    }
    return input;
}

fn characters_to_escape(ch: char) ->  Option<&'static str> {
    match ch {
        '%' => Some(r#"\%"#),
        '\\' => Some(r#"\\"#),
        '_' => Some(r#"\_"#),
        _ => None,
    }
}
