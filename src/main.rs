use rustyline::{Editor, Config};
use nikel_rs::NikelAPI;
use nikel_rs::*;

use std::vec::Vec;

const HISTORY_FILENAME: &str = "nikel.history";
const ITEM_SEP: &str = "\n-----\n";
const PROMPT: &str = ">>> ";

const AUTO_ADD_HISTORY: bool = true;
const HISTORY_IGNORE_SPACE: bool = true;
const MAX_HISTORY: usize = 100;

fn main() {
    let mut history_file: Option<String>;

    // Get history file
    match std::env::temp_dir().to_str() {
        Some(temp_dir) => history_file = Some(format!("{}{}{}", temp_dir, std::path::MAIN_SEPARATOR, HISTORY_FILENAME)),
        _ => {
            eprintln!("Couldn't get temporary dir, history will not be saved");
            history_file = None;
        }
    }

    let client = NikelAPI::new();

    let config = Config::builder()
                .auto_add_history(AUTO_ADD_HISTORY)
                .history_ignore_space(HISTORY_IGNORE_SPACE)
                .max_history_size(MAX_HISTORY)
                .build();

    let mut rl = Editor::<()>::with_config(config);

    // Load history safely
    if history_file.as_ref().is_some() {
        let hf: &str = history_file.as_ref().unwrap();
        if std::path::Path::new(hf).exists() {
            if rl.load_history(hf).is_err() {
                eprintln!("Failed to load history file {}", hf);
                history_file = None;
            } else {
                println!("Loaded history file {} with {} entries", hf, rl.history().len());
            }
        } else {
            println!("History file {} doesn't exist -- not loading", hf);
        }
    }

    // Main loop
    loop {
        let result = rl.readline(PROMPT);
        match result {
            Ok(line) => {
                
                if line.trim().is_empty() {
                    continue;
                }
                let (command, args) = split_once(&line, ' ');

                // Convert input into `Parameters` (`Vec<(&str, &str)>`)
                let params: Parameters = args.split(',')
                .map(|arg| arg.trim().split(":").collect())
                .map(|v: Vec<&str>| (v[0], v[1]))
                .collect();

                let out: String;

                // Make appropriate API call
                match command {
                    "courses" | "classes" => out = to_string(client.courses(params).unwrap()),
                    "textbooks" | "tb" => out = to_string(client.textbooks(params).unwrap()),
                    "exams" => out = to_string(client.exams(params).unwrap()),
                    "evals" => out = to_string(client.evals(params).unwrap()),
                    "food" => out = to_string(client.food(params).unwrap()),
                    "services" | "serv" => out = to_string(client.services(params).unwrap()),
                    "parking" | "park" => out = to_string(client.parking(params).unwrap()),
                    _ => continue
                }
                println!("==========\n{}\n==========", out);
            },
            _ => {
                println!("Exit");
                break;
            }
        }
    }

    if history_file.is_some() {
        if rl.save_history(&history_file.unwrap()).is_err() {
            eprintln!("Failed to save history file");
        }
    }

}

fn to_string<T: std::fmt::Debug>(resp: Response<T>) -> String {
    resp.response.iter().map(|e| format!("{:#?}", e)).collect::<Vec<String>>().join(ITEM_SEP)
}

fn split_once(in_string: &str, delim: char) -> (&str, &str) {
    let mut splitter = in_string.splitn(2, delim);
    let first = splitter.next().unwrap();
    let second = splitter.next().unwrap();
    (first, second)
}