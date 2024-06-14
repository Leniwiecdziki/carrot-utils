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
    // true - Show lines from pipe/file as they are appended
    // false - Classic, default mode. Show output from pipe or a file when writing is finished.
    let mut show_updates_only = false;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < opts.len() {
        for s in &swcs {
            if s != "u" && s != "update" && s != "w" && s != "wait" {
                eprintln!("Unknown switch: {s}");
                process::exit(1);
            }
            if s == "w" || s == "wait" {
                show_updates_only = false;
            }
            if s == "u" || s == "update" {
                show_updates_only = true;
            }
        }
        index += 1;
    }
    
    // Something is being piped?
    if !io::stdin().is_terminal() {
        todo!("Pipe support is not ready yet!");
    }

    for o in &opts {
        // Create 4 byte buffer
        let mut buffer: [u8; 4] = [0,1,2,3];
        // While reading from a file...
        while let Ok(n_bytes) = fs::read(&mut buffer) {
            // Quit if text is empty
            if n_bytes == 0 { break }
            // Convert UTF-8 to string
            let text = core::str::from_utf8(&buffer).unwrap();
            // Print string
            print!("{text}");
            // Clear buffer
            buffer.fill(0);
        }
}
