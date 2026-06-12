use std::io::{self, Write};

pub fn print_error(msg: &str) {
    eprintln!("\x1b[31m{}\x1b[0m", msg);
}

pub fn print_success(msg: &str) {
    println!("\x1b[32m{}\x1b[0m", msg);
}

pub fn redraw_prompt(input: &str) {
    print!("\r\x1b[2K\x1b[32maum > \x1b[0m{}", input);
    io::stdout().flush().unwrap();
}
