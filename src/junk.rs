// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::os::unix::fs::FileExt;
use std::process;
use std::io;
use rand::Rng;
pub mod libargs;

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

fn parselen(lenght:&String) -> u64 {
    match lenght.parse::<u64>() {
        Err(e) => {
            eprintln!("{lenght}: Failed to parse file lenght because of an error: {:?}", e.kind());
            process::exit(1);
        }
        Ok(string_to_int) => string_to_int
    }
}

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    // If no options were passed
    if opts.is_empty() {
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }
    let mut write_random_data = true;
    let mut get_lenght_automatically = true;
    let mut manually_set_len = 0;
    let mut toclear = true;
    let mut verbose = false;
    
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        let v = &vals[index];
        if s != "a" && s != "ask" && s != "v" 
        && s != "r" && s != "random"
        && s != "z" && s != "zero"
        && s != "l" && s != "len" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "a" || s == "ask" {
            if !v.is_empty() { eprintln!("{s}: This switch doesn't accept a value!"); process::exit(1); };
            toclear = ask(&opts[index], index);
        }
        if s == "r" || s == "random" {
            if !v.is_empty() { eprintln!("{s}: This switch doesn't accept a value!"); process::exit(1); };
            write_random_data = true;
        }
        if s == "z" || s == "zero" {
            if !v.is_empty() { eprintln!("{s}: This switch doesn't accept a value!"); process::exit(1); };
            write_random_data = false;
        }
        if s == "l" || s == "len" {
            if v.is_empty() { eprintln!("{s}: This switch needs a value!"); process::exit(1); };
            get_lenght_automatically = false;
            manually_set_len = parselen(v);
        }
        if s == "v" {
            verbose = true;
        }
        index += 1;
    };
    let mut index = 0;
    while index < opts.len() {
        // If 'toclear' is true, write a random number to each byte of a file
        if toclear {
            match fs::OpenOptions::new().read(true).write(true).create(true).open(&opts[index]) {
                Err(e) => eprintln!("{}: Couldn't open a file because of an error: {}!", &opts[index], e.kind()),
                Ok(file) => {
                    let bytes_to_write = if get_lenght_automatically {
                        file.metadata().expect("Failed to get file length!").len()
                    }
                    else {
                        manually_set_len
                    };
                    for i in 0..bytes_to_write {
                        let data = if write_random_data{
                            rand::random::<u8>()
                        } else {
                            48
                        };
                        match file.write_at(&[data], i) {
                            Err(e) => {
                                eprintln!("{}: Write operation failed because of an error: {}", &opts[index], e.kind());
                                break;
                            },
                            Ok(_) => {
                                if verbose && bytes_to_write == i+1 {
                                    println!("{}: Successfully wrote {} bytes.", &opts[index], bytes_to_write);
                                }
                            },
                        };
                    }
                },
            };
        }
        index += 1;
    }
}
