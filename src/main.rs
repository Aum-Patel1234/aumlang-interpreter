use std::{
    env, fs,
    io::{self, Write},
};

use aumlang::processor::process_input;

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

    loop {
        print!("aum > ");
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

                process_input(trimmed_line);
            }
            Err(e) => {
                eprintln!("Error reading the input: {}", e);
            }
        }
    }
}

fn run_file(args: &[String]) {
    let file_path = &args[1];

    match fs::read_to_string(file_path) {
        Ok(content) => process_input(&content),
        Err(e) => eprintln!("Failed to read file: {}", e),
    }
}
