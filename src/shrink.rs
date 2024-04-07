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
    let width = opts[0].parse::<usize>().expect("Failed to convert user input to a number!");
    // and remove it from the list
    opts.remove(0);

    // Process piped stuff
    if !io::stdin().is_terminal() {
        // Save contents of STDIN to a string
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");
        shrink(width, &contents_of_stdin);
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
                shrink(width, &f);
            },
        };
        index += 1;
    };
}

pub fn shrink(width:usize, text:&str) {
    // Prepared lines will be stored there
    let mut output = String::new();

    // Now, for every line in our retrieved contents...
    let mut idx = 1;
    for c in text.chars() {
        // Insert chars as usual
        output.push(c);
        // If the width is reached, make a new line
        if idx > width {
            output.push('\n');
            idx=1;
        }
        idx+=1;
    }
    println!("{}", output);
}
