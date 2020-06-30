use rustyline::Editor;
use nikel_rs::NikelAPI;

use std::vec::Vec;
use std::collections::hash_map::HashMap;

const HISTORY_FILE: &str = "/tmp/nikel.history";

fn main() {
    let client = NikelAPI::new();
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY_FILE).ok();
    loop {
        let result = rl.readline(">>> ");
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
                        out = r.response.iter().map(|e| format!("{}|{}: {}", e.code.as_ref().unwrap(), e.campus.as_ref().unwrap(), e.description.as_ref().unwrap())).collect::<Vec<String>>().join("\n-----\n");
                    },
                    "textbooks" | "tb" => {
                        let r = client.textbooks(map).unwrap();
                        out = r.response.iter().map(|e| format!("{}|{}: ${}", e.title.as_ref().unwrap(), e.author.as_ref().unwrap(), e.price.as_ref().unwrap())).collect::<Vec<String>>().join("\n-----\n");        
                    },
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
