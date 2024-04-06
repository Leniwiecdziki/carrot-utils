use std::io::{self, Read};
use std::io::IsTerminal;
use std::fs;
use std::process;
use carrot_libs::args;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    // Mode 0 - convert everything to uppercase
    // Mode 1 - convert everything to lowercase
    let mut mode = 0;

    // Ask for one switch
    if swcs.len() > 1 {
        eprintln!("This program requires one switch to be defined!");
        process::exit(1);
    }

    let s = &swcs[0];
    let v = &vals[0];

    if s != "l" && s != "lr" && s != "low" && s != "lower" && s != "lowercase" &&
    s != "u" && s != "up" && s != "upper" && s != "uppercase"
    {
        eprintln!("Unknown switch: {s}");
        process::exit(1);
    }
    // Make sure, that value is set!
    if !v.is_empty() { 
        eprintln!("This switch does not support any values: {s}!"); process::exit(1); 
    }
    
    // Change to lowercase mode if needed
    if s == "l" || s == "lr" || s == "low" || s == "lower" || s == "lowercase" {
        mode = 1
    }

    // Too many switches?
    if swcs.len() > 1 {
        eprintln!("This program does not accept more than one switch!");
        process::exit(1);
    }

    // Show error when there are no files requested as options by user and nothing is piped to the program
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("Type the name of elements to use!");
        process::exit(1);
    }

    // If something is piped to our program, show it.
    if !io::stdin().is_terminal() {
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");
        change_case(contents_of_stdin, mode);
    }

    if opts.is_empty() {
        process::exit(0);
    }
    // Show stuff requested as options
    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => {
                change_case(f, mode);
            },
        };
        index += 1;
    };
}

fn change_case(text:String, mode:u8) {
    if mode == 0 {
        println!("{}", text.to_uppercase());
    } else {
        println!("{}", text.to_lowercase());
    }
}