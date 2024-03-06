

use std::fs::File;
use std::env;
use std::process::exit;
use std::io::{BufRead, BufReader};
use sha2::{Sha256, Digest};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("There must be three arguments!(cargo run [hash] [wordlist])");
        exit(1);
    }

    // get given hash and wordlist
    let given_hash = &args[1];
    let filename = &args[2];
    let file = File::open(filename).unwrap();
    let f = BufReader::new(file);

    println!("Attempting to crack the hash {}!\n", given_hash);
    println!("Trying passwords:");

    // search for the hash in wordlist
    for line in f.lines() {
        let line = line.unwrap();
        println!("{}", line);
        let word = line.trim().to_owned().into_bytes();
        // hash every word from wordlist
        let hash = format!("{:x}", Sha256::digest(&word));

        // verify if hashes match
        if given_hash == &hash {
            println!("Password found!!! The password is {}!", line);
            exit(0);
        }
    }
    println!("The password not found!");
}
