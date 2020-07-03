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
                let (command, args) = match split_once(&line, ' ') {
                    Ok(tup) => tup,
                    _ => {
                        println!("Failed to parse command");
                        continue;
                    }
                };

                // Convert input into `Parameters` (`Vec<(&str, &str)>`)
                let params: Parameters = args.split(',')
                .map(|arg| arg.split(":").map(|e| e.trim()).collect())
                .filter(|v: &Vec<&str>| {
                    if v.len() != 2 {
                        println!("Couldn't parse option {:?}, ignoring", v);
                        false
                    } else {
                        true
                    }
                })
                .map(|v: Vec<&str>| (v[0], v[1]))
                .collect();

                // Make appropriate API call
                 match req(&client, command, params) {
                    Ok(string) => {
                        println!("==========\n{}\n==========", string);
                    },
                    _ => {
                        println!("There was a problem with that request");
                        continue;
                    }
                };
                
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

fn req(client: &NikelAPI, command: &str, params: Parameters) -> Result<String, Box<dyn std::error::Error>> {
    let out = match command {
        "courses" | "classes" => to_string(client.courses(params)?),
        "textbooks" | "tb" => to_string(client.textbooks(params)?),
        "exams" => to_string(client.exams(params)?),
        "evals" => to_string(client.evals(params)?),
        "food" => to_string(client.food(params)?),
        "services" | "serv" => to_string(client.services(params)?),
        "parking" | "park" => to_string(client.parking(params)?),
        _ => return Err("".into()) // Why..
    };
    return Ok(out);
}

fn to_string<T: std::fmt::Debug>(resp: Response<T>) -> String {
    resp.response.iter().map(|e| format!("{:#?}", e)).collect::<Vec<String>>().join(ITEM_SEP)
}

fn split_once(in_string: &str, delim: char) -> Result<(&str, &str), ()> {
    let mut splitter = in_string.splitn(2, delim);
    let first = match splitter.next() {
        Some(s) => s,
        _ => return Err(())
    };
    let second = match splitter.next() {
        Some(s) => s,
        _ => return Err(())
    };
    Ok((first, second))
}