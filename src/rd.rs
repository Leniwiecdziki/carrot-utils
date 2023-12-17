// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::process;
use std::io;
mod libargs;

fn ask(opt: &String, mut _index: usize) -> bool {
    let mut input = String::new();
    let mut toclear:bool = false;
    println!("{}: Do you really want to delete this? [y/n]: ", opt);
    io::stdin().read_line(&mut input).unwrap();
    if &mut input.to_lowercase() == "y\n" {
        toclear = true;
    }
    else if &mut input.to_lowercase() == "n\n" {
        toclear = false;
    }
    else {
        println!("I do not understand, can you repeat?");
        ask(opt, _index);
    }
    toclear
}

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    // If no options were passed
    if opts.is_empty() {
        eprintln!("This program requires at least one dir name!");
        process::exit(1);
    }
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    let mut index = 0;
    let mut toclear = true;
    let mut verbose = false;
    let mut rec = false;
    for s in swcs {
        if s != "a" && s != "ask" && s != "r" && s != "rec" && s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "a" || s == "ask" {
            toclear = ask(&opts[index], index);
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        if s == "v" {
            verbose = true;
        }
    }
    while index < opts.len() {
        // Ask user if he/she really wants to remove a directory (if '-a'/'-ask' is available)
        
        // D E S T R O Y  if 'toclear' is true
        if toclear {
            if rec {
                match fs::remove_dir_all(&opts[index]) {
                    Err(e) => { eprintln!("{}: Cannot remove the file because of an error: {:?}!", opts[index], e.kind()); },
                    _ => { if verbose {println!("{}: Removed successfully.", opts[index]); }}
                }
            }
            else {
                match fs::remove_dir(&opts[index]) {
                    Err(e) => { eprintln!("{}: Cannot remove the file because of an error: {:?}!", opts[index], e.kind()); },
                    _ => if verbose {{ println!("{}: Removed successfully.", opts[index]); }}
                }
            }
        }
        index += 1;
    }
}
