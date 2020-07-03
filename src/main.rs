use rustyline::Editor;
use nikel_rs::NikelAPI;
use nikel_rs::*;

use std::vec::Vec;
use std::collections::hash_map::HashMap;

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
                let args: Vec<(&str, &str)> = input[1].split(",").map(|arg| {
                    let x: Vec<&str> = arg.split(":").collect();
                    return (x[0], x[1]);
                }).collect();
                let mut map = HashMap::<&str, &str>::new();
                for (k, v) in args {
                    map.insert(k, v);
                }
                let out: String;
                match command {
                    "courses" | "classes" => out = to_string(client.courses(map).unwrap()),
                    "textbooks" | "tb" => out = to_string(client.textbooks(map).unwrap()),
                    "exams" => out = to_string(client.exams(map).unwrap()),
                    "evals" => out = to_string(client.evals(map).unwrap()),
                    "food" => out = to_string(client.food(map).unwrap()),
                    "services" | "serv" => out = to_string(client.services(map).unwrap()),
                    "parking" | "park" => out = to_string(client.parking(map).unwrap()),
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