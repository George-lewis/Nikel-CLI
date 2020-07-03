use rustyline::Editor;
use nikel_rs::NikelAPI;
use nikel_rs::*;

use std::vec::Vec;

const HISTORY_FILENAME: &str = "nikel.history";
const ITEM_SEP: &str = "\n-----\n";
const PROMPT: &str = ">>> ";

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
    let mut rl = Editor::<()>::new();

    // Load history safely
    if history_file.as_ref().is_some() {
        let hf: &str = history_file.as_ref().unwrap();
        if std::path::Path::new(hf).exists() {
            if rl.load_history(hf).is_err() {
                eprintln!("Failed to load history file {}", hf);
                history_file = None;
            } else {
                println!("Loaded history file {}", hf);
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

                rl.add_history_entry(&line);

                if line.trim().is_empty() {
                    continue;
                }

                let input: Vec<&str> = line
                                        .split_ascii_whitespace()
                                        .collect();

                if input.len() < 2 {
                    continue;
                }

                let command = input[0];

                // Convert input into `Parameters` (`HashMap<&str, &str>`)
                let args: Parameters = input[1].split(",")
                .map(|arg| arg.split(":").collect())
                .map(|v: Vec<&str>| (v[0], v[1]))
                .collect();

                let out: String;

                // Make appropriate API call
                match command {
                    "courses" | "classes" => out = to_string(client.courses(args).unwrap()),
                    "textbooks" | "tb" => out = to_string(client.textbooks(args).unwrap()),
                    "exams" => out = to_string(client.exams(args).unwrap()),
                    "evals" => out = to_string(client.evals(args).unwrap()),
                    "food" => out = to_string(client.food(args).unwrap()),
                    "services" | "serv" => out = to_string(client.services(args).unwrap()),
                    "parking" | "park" => out = to_string(client.parking(args).unwrap()),
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