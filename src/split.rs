// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::process;
use std::io::{self, Read, IsTerminal};
use carrot_libs::args;
use carrot_libs::input;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    // If no options were passed
    if opts.is_empty() {
        eprintln!("This program requires at least one file name to write to!");
        process::exit(1);
    }

    let mut index = 0;
    let mut ask_if_exists = true;
    // Meaning:
    // 0 - Append to a file
    // 1 - Overwrite a file
    let mut write_policy_if_exists = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < opts.len() {
        for s in &swcs {
            if s != "a" && s != "ask" && s != "u" && s != "update" && s != "o" && s != "overwrite" {
                eprintln!("Unknown switch: {s}");
                process::exit(1);
            }
            if s == "a" || s == "ask" {
                let toclear = match input::ask(&opts[index]) {
                    Err(e) => {
                        eprintln!("Can't get user input: {}!", e);
                        process::exit(1);
                    },
                    Ok(e) => e
                };
            }
            if s == "u" || s == "update" {
                write_policy_if_exists = 0;
            }
            if s == "o" || s == "overwrite" {
                write_policy_if_exists = 1;
            }
        }
        index += 1;
    }
    
    // Process piped stuff
    if io::stdin().is_terminal() {
        eprintln!("Pipe another command through this program!");
        process::exit(1);
    }
    else {
        // Save contents of STDIN to a string
        //let mut contents_of_stdin = String::new();
        let mut stdin = io::stdin();
        let mut line = [0];
        while let Ok(n_bytes) = stdin.read(&mut line) {
            if n_bytes == 0 { break }
            println!("{:o}", line[0]);
        }
    };
}

fn split(content:&String, opts:&Vec<String>) {
    for o in opts {
        ();
    }
    println!("{}", content)
}
