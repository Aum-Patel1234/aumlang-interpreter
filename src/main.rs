use std::{
    cell::RefCell,
    env, fs,
    io::{self, Write},
    rc::Rc,
};

use aumlang::{environment::Environment, processor::process_input};

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
    let env = Rc::new(RefCell::new(Environment::default()));

    loop {
        print!("\x1b[32maum > \x1b[0m");
        io::stdout().flush().unwrap();

        input.clear();

        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed_line = input.trim();

                if trimmed_line.eq("exit") {
                    break;
                }
                // println!("{}", trimmed_line);

                process_input(trimmed_line, env.clone());
            }
            Err(e) => {
                eprintln!("Error reading the input: {}", e);
            }
        }
    }

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
