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
                    "courses" | "classes" => {
                        let r = client.courses(map).unwrap();
                        out = to_string(r);
                    },
                    "textbooks" | "tb" => {
                        let r = client.textbooks(map).unwrap();
                        out = to_string(r);
                    },
                    "exams" => {
                        let r = client.exams(map).unwrap();
                        out = to_string(r);
                    },
                    "evals" => {
                        let r = client.evals(map).unwrap();
                        out = to_string(r);
                    },
                    "food" => {
                        let r = client.food(map).unwrap();
                        out = to_string(r);
                    },
                    "services" | "serv" => {
                        let r = client.services(map).unwrap();
                        out = to_string(r);
                    },
                    "parking" | "park" => {
                        let r = client.parking(map).unwrap();
                        out = to_string(r);
                    }
                    _ => panic!()
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