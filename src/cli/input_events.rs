use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

use arboard::Clipboard;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};

use crate::{environment::Environment, processor::process_input, utils::print_error};

pub fn key_event_up(input: &mut String) {
    // TODO: history previous
    input.clear();
    input.push_str("up");

    print!("\r\x1b[2K\x1b[32maum > \x1b[0m{}", input);
    io::stdout().flush().unwrap();
}

pub fn key_event_down(input: &mut String) {
    // TODO: history next
    input.clear();
    input.push_str("down");

    print!("\r\x1b[2K\x1b[32maum > \x1b[0m{}", input);
    io::stdout().flush().unwrap();
}

pub fn key_event_enter(input: &mut String, env: Rc<RefCell<Environment>>) -> bool {
    let line = input.trim().to_string();

    print!("\r\n");
    io::stdout().flush().unwrap();

    if line == "exit" {
        return false;
    }

    disable_raw_mode().unwrap();
    process_input(&line, env);
    enable_raw_mode().unwrap();

    input.clear();

    print!("\r\x1b[2K\x1b[32maum > \x1b[0m");
    io::stdout().flush().unwrap();
    true
}

pub fn key_event_backspace(input: &mut String) {
    if !input.is_empty() {
        input.pop();

        print!("\r\x1b[2K\x1b[32maum > \x1b[0m{}", input);
        io::stdout().flush().unwrap();
    }
}

pub fn key_event_add_char(input: &mut String, c: char) {
    input.push(c);

    print!("{c}");
    io::stdout().flush().unwrap();
}

pub fn key_event_paste(input: &mut String) {
    // NOTE:
    // 1. to avoid unexpected printing i am removing \n
    // 2. also there is a bug if you write # it will consider remaining thing
    //    as comment and i didnt want to fix this
    let mut clipboard = match Clipboard::new() {
        Ok(c) => c,
        Err(e) => {
            print_error(&format!("Error while pasting: {}", e));
            return;
        }
    };

    if let Ok(text) = clipboard.get_text() {
        let text = text.replace("\r\n", " ").replace('\n', " ");
        input.push_str(&text);
        print!("{text}");
        io::stdout().flush().unwrap();
    }
}
