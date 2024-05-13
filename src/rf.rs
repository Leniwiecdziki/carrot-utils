// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::process;
use std::io;
use carrot_libs::args;
use carrot_libs::input;

fn ask(opt: &String) -> bool {
    let mut toclear:bool = false;
    let input = input::get(format!("{}: Do you really want to delete this? [y/n]: ", opt), false);
    if input.len() != 1 {
        println!("Sorry! I don't undestand your input.");
        ask(opt);
    }
    let lowercased_input = input[0].trim().to_lowercase();
    if lowercased_input == "y" || lowercased_input == "yes" { toclear = true; }
    else if lowercased_input == "n" || lowercased_input == "no" { toclear = false; }
    else { println!("Sorry! I don't undestand your input."); ask(opt); }

    toclear
}

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    // If no options were passed
    if opts.is_empty() {
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }
    let mut index = 0;
    let mut toclear = true;
    let mut verbose = false;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < opts.len() {
        // Ask user if he/she really wants to remove a file (if '-a'/'-ask' is available)
        for s in &swcs {
            if s != "a" && s != "ask" && s != "v" && s != "verbose" {
                eprintln!("Unknown switch: {s}");
                process::exit(1);
            }
            if s == "a" || s == "ask" {
                toclear = ask(&opts[index]);
            }
            if s == "v" || s == "verbose" {
                verbose = true;
            }
        }
        // D E S T R O Y  if 'toclear' is true
        if toclear {
            match fs::remove_file(&opts[index]) 
            {
                Err(e) => {
                    eprintln!("{}: Cannot remove the file because of an error: {:?}!", opts[index], e.kind());
                },
                _ => {
                    if verbose {
                        println!("{}: Removed successfully.", opts[index]);                        
                    }
                }
            }
        }
        index += 1;
    }
}
