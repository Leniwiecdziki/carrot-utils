// Function 'ask' is used to modify 'index' variable but never reads it. That's ok.
#![allow(unused)]
use std::fs;
use std::process;
use std::io::{self, Read, Write, IsTerminal};
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
    // Meaning:
    // true - Append to a file
    // false - Overwrite a file
    let mut update_if_exists = true;
    let mut ask = false;
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
                ask = true;
            }
            if s == "u" || s == "update" {
                update_if_exists = true;
            }
            if s == "o" || s == "overwrite" {
                update_if_exists = false;
            }
        }
        index += 1;
    }
    
    // No piped content?
    if io::stdin().is_terminal() {
        eprintln!("Pipe another command through this program!");
        process::exit(1);
    }

    // If user has enabled asking before changing existing files,
    // save answers from all confirmation prompts to this list
    let mut answers = Vec::new();

    for o in &opts {
        // Go through all options and check if file exists
        let file_exists = fs::metadata(o).is_ok();
        // If file requested as option already exists...
        if file_exists && ask {
            // Show confirmation dialog if user wants this program to ask before changing existing files
            // We'll change existing file depending on the answer from user
            let go_ahead = if ask {
                match input::ask(format!("{o}: Do you really want to change this file?")) {
                    Err(e) => {
                        eprintln!("Failed to get user input: {}!", e);
                        process::exit(1);
                    },
                    Ok(to_delete_or_not_to_delete_that_is_the_question) => to_delete_or_not_to_delete_that_is_the_question,
                }
            }
            // If asking is disabled, just go ahead and change what is requested
            else {
                true
            };
            answers.push(go_ahead);
        }
        // Write what you want if the file does not exist. Don't care about "-ask".
        else {
            answers.push(true);
        }
    }

    // Clear existing files if "-o" is used before actually working with pipes and appending
    // it's output to files and the terminal
    for (i,o) in opts.iter().enumerate() {
        if answers[i] {
            clear_file(update_if_exists, o);
        }
    }

    // Create 4 byte buffer
    let mut buffer: [u8; 4] = [0,1,2,3];
    // While reading from STDIN...
    while let Ok(n_bytes) = io::stdin().read(&mut buffer) {
        // Quit if read text is empty
        if n_bytes == 0 { break }
        // Convert UTF-8 to string
        let text = core::str::from_utf8(&buffer).unwrap();
        // Print string
        print!("{text}");
        // Save string to a file
        for (i, o) in opts.iter().enumerate() {
            // If we can continue, because user accepted changing existing file when asked,
            // or if the desired file does not exist, do the magic.
            if answers[i] {
                // Open the file with or without append option and create if it doesn't exist
                write_to_file(o, &text.to_string());
            }
        }
        // Clear buffer
        buffer.fill(0);
    }
}

fn clear_file<S: AsRef<str>>(update_if_exists:bool, o: S) {
    // Generally speaking using append(false) in write_to_file() causes a lot of chaos.
    // If user does not want to preserve previous file contents (if any exists) just
    // delete everything inside first and then do some appending with write_to_file() while reading
    // text from pipe.
    if !update_if_exists {
        let file = match fs::OpenOptions::new().create(false).write(true).truncate(false).open(o.as_ref()) {
            Err(e) => {
                eprintln!("{}: Failed to open file for truncate operation: {:?}!", o.as_ref(), e.kind());
                process::exit(1);
            },
            Ok(a) => a,
        };
        if let Err(e) = fs::File::set_len(&file, 0) {
            eprintln!("{}: Failed to truncate a file: {:?}!", o.as_ref(), e.kind());
            process::exit(1);
        }
    }
}

fn write_to_file<S: AsRef<str>>(o: S, text: S) {
    // This function only appends text to the file!
    // if you want to look how the program clears the file if "-o" is used, see clear_file()
    match fs::OpenOptions::new().create(true).append(true).truncate(false).open(o.as_ref()) {
        Err(e) => {
            eprintln!("{}: Failed to open a file: {:?}", o.as_ref(), e.kind());
            process::exit(1);
        },
        // Write line to the file
        Ok(mut a) => {
            if let Err(e) = write!(a, "{}", text.as_ref()) {
                eprintln!("{}: Couldn't write to file: {:?}", o.as_ref(), e.kind());
            }
        }
    }
}