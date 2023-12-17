// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::process;
use std::io;
extern crate rand;
pub mod libargs;
use crate::rand::Rng;

fn ask(opt: &String, mut _index: usize) -> bool {
    let mut input = String::new();
    let mut toclear:bool = false;
    println!("{}: Do you really want to delete this? [y/n]: ", opt);
    io::stdin().read_line(&mut input).unwrap();
    if &mut input == "y\n" || &mut input == "Y\n" {
        toclear = true;
    }
    else if &mut input == "n\n" || &mut input == "N\n" {
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
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    for s in swcs {
        if s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        else {
            verbose = true;
        }
    }

    let mut index = 0;
    let mut toclear = true;
    while index < opts.len() {
        // Ask user if he/she really wants to remove a file (if '-a'/'-ask' is available)
        /*
        for s in swcs {
            if s != "a" && s != "ask" {
                eprintln!("Unknown change: {s}");
                process::exit(1);
            }
            if s == "a" || s == "ask" {
                toclear = ask(&opts[index], index);
            }
        }
        */
        // D E S T R O Y  if 'toclear' is true
        /*
        if toclear {
            match io::BufWriter::new(&opts[index]).write(rand::thread_rng().gen::<u32>()) 
            {
                Err(e) => {
                    eprintln!("{}: Cannot remove the file because of an error: {:?}!", opts[index], e.kind());
                },
                _ => {
                    ;
                    println!("{}: Nuked successfully.", opts[index]);
                }
            }
        }
        */
        index += 1;
    }
}
