use rustyline::Editor;
use nikel_rs::NikelAPI;
use nikel_rs::*;

use std::vec::Vec;

const HISTORY_FILE: &str = "/tmp/nikel.history";
const ITEM_SEP: &str = "\n-----\n";
const PROMPT: &str = ">>> ";

fn main() {
    let client = NikelAPI::new();
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY_FILE).ok();
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
                let args: Parameters = input[1].split(",")
                .map(|arg| arg.split(":").collect())
                .map(|v: Vec<&str>| (v[0], v[1]))
                .collect();
                let out: String;
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
    rl.save_history(HISTORY_FILE).ok();
}

fn to_string<T: std::fmt::Debug>(resp: Response<T>) -> String {
    resp.response.iter().map(|e| format!("{:#?}", e)).collect::<Vec<String>>().join(ITEM_SEP)
}