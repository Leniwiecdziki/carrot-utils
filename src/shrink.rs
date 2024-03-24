#![allow(dead_code)]
use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process;
use carrot_libs::args;


fn main() {
    let mut opts = args::opts().clone();
    let (swcs, _) = args::swcs();
    for v in swcs {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    if opts.len() < 2 && io::stdin().is_terminal()  {
        eprintln!("This program requires the width to be set and a source to process!");
        process::exit(1);
    }
    // Save the first option as a width parameter
    let width = opts[0].parse::<usize>().expect("Failed to convert user input to a number");
    // and remove it from the list
    opts.remove(0);

    // Process piped stuff
    if !io::stdin().is_terminal() {
        // Save contents of STDIN to a string
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");
        let output = shrink(width, &contents_of_stdin);
        println!("{}", output);
    };

    if opts.is_empty() {
        process::exit(0);
    }
    // Process stuff requested as options
    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => {
                let output = shrink(width, &f);
                println!("{}", output);
            },
        };
        index += 1;
    };
}

pub fn shrink(width:usize, text:&str) -> String {
    // Prepared lines will be stored there
    let mut lines = String::new();

    // Now, for every line in our retrieved contents...
    for line in text.lines() {
        if line.len() > width {
            // How many times do we need to shrink it?
            let how_many_iterations = line.len() / width;
            // Start counting from 1
            let mut split_count = 1;
            while split_count != how_many_iterations+1 {
                // If iteration was already ran, remove unneeded characters from line
                let previous_split_count = split_count-1;
                // The number of characters that were split in previous loop iteration
                let n = previous_split_count * width;
                let shorter_line = line[n..].split_at(width);
                // Add a number and contents of a line to the list
                lines.push_str(format!("{}\n", shorter_line.0).as_str());
                // If this is the last iteration, add the last piece of text
                if split_count == how_many_iterations {
                    lines.push_str(format!("{}\n", shorter_line.1).as_str());
                };
                split_count +=1;
            };
        }
    }
    lines
}
