use cursive::reexports::log::warn;

#[warn(unused_assignments)]

use std::io;
use std::fs::{self };
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub fn run() {

    // get arguments

    let args: Vec<String> = std::env::args().collect();
    if args.len() >= 2 {
        let first_arg = &args[1];
        //println!("First argument: {}", first_arg);
    } else {
        println!("There must be at least 2 arguments!");
    }
    // select the directory to be listed
    //  - if the first argument exists, that is the directory
    //  - if it does not, the current directory will be listed

    let mut directory = "";

    if args.len() < 2 {
        directory = ".";
    } else {
        directory = &args[1];
    }

    println!("{}", directory);

    // print the contents of the directory
    // print the type of each item (dir, file, link)
    // print the properties of each directory / file

    let path = Path::new(directory);

    match fs::read_dir(path) {
        Ok(dir) => {
            for entry in dir {
                match entry {
                    Ok(ent) => {
                        let file = ent.path();
                        match file.metadata() {
                            Ok(f) => {
                                let permissions = f.permissions();
                                let p = permissions.mode();
                                if file.is_dir() {
                                    println!("{} {:?} - dir", p, file);
                                } else if file.is_file() {
                                    println!("{} {:?} - file", p, file);
                                } else {
                                    println!("{} {:?} - link", p, file);
                                }
                            }
                            Err(_errr) => return
                        }
                    },
                    Err(_er) => return
                }
            }
        }
        Err(_e) => return
    }
}
