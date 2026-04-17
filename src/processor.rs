use crate::parser::{get_tokens, print_tokens};

pub fn process_line(line: &str) {
    if line.is_empty() || line.starts_with("#") {
        return;
    }
    let tokens = get_tokens(line);
    print_tokens(&tokens);
}
