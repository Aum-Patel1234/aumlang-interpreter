use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::Path,
};

use crate::utils::print_error;

const HISTORY_FILE: &str = ".aumlang_repl_history";
const HISTORY_SEPERATOR: &str = "\n";
pub const HISTORY_COMMAND_MAX_SIZE: usize = 2000;

pub struct ReplHistory {
    pub history: Vec<String>,
    pub curr_idx: usize,
}

impl Default for ReplHistory {
    fn default() -> Self {
        Self::new()
    }
}
impl ReplHistory {
    pub fn new() -> Self {
        let path = Path::new(HISTORY_FILE);
        let mut file = match File::open(path) {
            Ok(f) => f,
            Err(_) => {
                return ReplHistory {
                    history: Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE),
                    curr_idx: 0,
                };
            }
        };

        let mut file_str = String::new();
        match file.read_to_string(&mut file_str) {
            Ok(_) => {}
            Err(_) => {
                return ReplHistory {
                    history: Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE),
                    curr_idx: 0,
                };
            }
        }
        // println!("{}", history);

        let commands = file_str.split(HISTORY_SEPERATOR);
        let mut history: Vec<String> = Vec::with_capacity(HISTORY_COMMAND_MAX_SIZE);
        for command in commands {
            history.push(command.to_string());
        }
        let n = history.len();
        ReplHistory {
            history,
            curr_idx: n,
        }
    }

    #[allow(clippy::should_implement_trait)] // clippy giving warning for confusing name with iterators
    pub fn next(&mut self) -> Option<&str> {
        if self.curr_idx >= self.history.len() {
            return None;
        }
        self.curr_idx += 1;
        if self.curr_idx >= self.history.len() {
            return None;
        }
        Some(&self.history[self.curr_idx])
    }
    pub fn back(&mut self) -> Option<&str> {
        if self.history.is_empty() || self.curr_idx == 0 {
            return None;
        }
        self.curr_idx -= 1;
        Some(&self.history[self.curr_idx])
    }

    pub fn push_command(&mut self, command: String) {
        if self.history.len() >= HISTORY_COMMAND_MAX_SIZE {
            self.history.clear();
        }
        self.history.push(command);
        self.curr_idx = self.history.len();
    }
    // for debug purpose
    // pub fn print_history(&self) {
    //     println!("curr_idx = {}, {:?}", self.curr_idx, self.history)
    // }
}

impl Drop for ReplHistory {
    fn drop(&mut self) {
        if self.history.len() > HISTORY_COMMAND_MAX_SIZE {
            let excess = self.history.len() - HISTORY_COMMAND_MAX_SIZE;
            self.history.drain(0..excess);
        }
        assert!(self.history.len() <= HISTORY_COMMAND_MAX_SIZE);
        let path = Path::new(HISTORY_FILE);

        let mut file = match OpenOptions::new()
            .read(true)
            .truncate(true) // truncate deletes prev content in file
            .write(true)
            .create(true)
            .open(path)
        {
            Ok(f) => f,
            Err(e) => {
                print_error(&e.to_string());
                return;
            }
        };

        let contenet = self
            .history
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(HISTORY_SEPERATOR);

        if let Err(e) = file.write_all(contenet.as_bytes()) {
            print_error(&e.to_string());
        }
    }
}
