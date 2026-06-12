use std::{cell::RefCell, env, fs, rc::Rc};

use aumlang::{
    cli::{
        cli_history::ReplHistory,
        input_events::{
            key_event_add_char, key_event_backspace, key_event_down, key_event_enter,
            key_event_paste, key_event_up,
        },
    },
    environment::Environment,
    processor::process_input,
    utils::redraw_prompt,
};
use crossterm::{
    event::{Event, KeyCode, KeyEventKind, KeyModifiers, read},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        run_cli();
    } else {
        run_file(&args);
    }
}

fn run_cli() {
    let mut input = String::new();
    let mut history = ReplHistory::default();
    let env = Rc::new(RefCell::new(Environment::default()));
    // this gets every event of the key raw
    enable_raw_mode().unwrap();
    redraw_prompt(&input);

    loop {
        match read().unwrap() {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                KeyCode::Up => key_event_up(&mut input, &mut history),
                KeyCode::Down => key_event_down(&mut input, &mut history),
                KeyCode::Enter if !key_event_enter(&mut input, &mut history, env.clone()) => {
                    break;
                }
                KeyCode::Backspace => key_event_backspace(&mut input),
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    break;
                }
                KeyCode::Char('v') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    key_event_paste(&mut input)
                }
                KeyCode::Char(c) => key_event_add_char(&mut input, c),
                // KeyCode::Tab => {
                //     // TODO: autocomplete
                // }
                _ => {}
            },
            _ => {}
        }
    }

    disable_raw_mode().unwrap();
    println!();
    for (k, v) in env.borrow().iter() {
        println!("key = {}, val = {}", k, v);
    }
}

fn run_file(args: &[String]) {
    let file_path = &args[1];
    let env = Rc::new(RefCell::new(Environment::default()));

    match fs::read_to_string(file_path) {
        Ok(content) => process_input(&content, env.clone()),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
    for (k, v) in env.borrow().iter() {
        println!("key = {}, val = {}", k, v);
    }
}
