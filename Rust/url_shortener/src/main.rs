#![allow(unused)]

use std::env::{self, args};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use rand::{distributions::Alphanumeric, Rng};
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("There must be three arguments!(cargo run [url] [filename]");
        exit(1);
    }

    let url = &args[1];
    let filename = &args[2];
    let file = File::open(filename).unwrap();

    if url.starts_with("http") {
        let mut rng = rand::thread_rng();
        let short_url: String = std::iter::repeat(()).map(|()|rng.sample(Alphanumeric) as char).take(8).collect();

        let mut file = match OpenOptions::new().write(true).append(true).open(filename) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening file!");
                exit(1);
            }
        };

        let pair = format!("{}, {}", short_url, url);
        if let Err(_) = file.write_all(pair.as_bytes()) {
            println!("Error writing to file!");
            exit(1);
        }
        println!("{}, {}",short_url, url);
    } else {
        let file = match File::open(filename) {
            Ok(file) => file,
            Err(_) => {
                println!("Error opening file!");
                exit(1);
            }
        };
        let f = BufReader::new(file);
        for line in f.lines() {
            let l = match line {
                Ok(line) => line,
                Err(_) => {
                    println!("Error reading file!");
                    continue;
                }
            };
            let words: Vec<&str> = l.split(",").collect();
            if words.len() != 2 {
                continue;
            }
            if words[0] == url {
                println!("Long url: {}", words[1]);
                exit(0);
            }
        }
        println!("Short url not found!");
    }
}
