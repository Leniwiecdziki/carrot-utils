#![allow(dead_code)]
use std::fs;
use std::collections::HashMap;
use std::io::{self, IsTerminal, Read};
use std::process;
use carrot_libs::args;


// Make a list of commands that share common purpose
const READ_FROM_START:[&str;4] = 
    ["top", "top-char", "t", "T"];
const READ_FROM_END:[&str;8] = 
    ["reverse", "reverse-char", "bottom", "bottom-char", "r", "R", "b", "B"];
const READ_PER_LINE:[&str;6] = 
    ["reverse", "top", "bottom", "r", "t", "b"];
const READ_PER_CHAR:[&str;6] = 
    ["reverse-char", "top-char", "bottom-char", "R", "T", "B"];

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    
    let mut line_number = false;
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "r" && s != "reverse"
        && s != "R" && s != "reverse-char"
        && s != "t" && s != "top"
        && s != "T" && s != "top-char"
        && s != "b" && s != "bottom"
        && s != "B" && s != "bottom-char"
        && s != "l" && s != "line-number"
        && s != "w" && s != "width" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        // Make sure, that for some options, a value can't be set
        if (s=="r"||s=="reverse"||s=="R"||s=="reverse-char"||s=="l"||s=="line-number") 
        && !v.is_empty() { 
            eprintln!("Unsupported value for a switch: {s}={v}!"); process::exit(1); 
        }
        if (s=="t"||s=="top"||s=="b"||s=="bottom"||s=="T"||s=="top-char"||s=="B"||s=="bottom-char")
        && v.is_empty() { 
            eprintln!("This switch requires a value: {s}!"); process::exit(1); 
        }
        if s=="l"||s=="line-counter" {
            line_number = true;
        }
        index += 1;
    }


    // Show error when there are no files requested as options by user and nothing is piped to the program
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("Type the name of elements to use!");
        process::exit(1);
    }

    // Show piped stuff
    if !io::stdin().is_terminal() {
        // Save contents of STDIN to a string
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");

        // Make a list of lines in a file and associate them with line numbers
        chtext(index_lines(&contents_of_stdin), index_chars(&contents_of_stdin), line_number);
    };

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
                chtext(index_lines(&f), index_chars(&f), line_number);
            },
        };
        index += 1;
    };
}

pub fn index_lines(text:&str) -> HashMap<usize, String> {
    // Prepared lines will be stored there
    let mut lines = HashMap::new();

    // Now, for every line in our retrieved contents...
    for (idx, line) in text.lines().enumerate() {
        // Add it to "lines" with proper ID
        lines.insert(idx, line.to_string());
    };
    lines
}

pub fn index_chars(text:&str) -> HashMap<usize, String> {
    // Prepared characters will be stored there
    let mut chars = HashMap::new();

    // Now, for every single letter in our retrieved contents...
    for (idx, char) in text.chars().enumerate() {
        chars.insert(idx, char.to_string());
    };
    chars
}

pub fn chtext(lines:HashMap<usize, String>, chars:HashMap<usize, String>, line_number:bool) {
    // Error handling was already done. Just iterate blindly through all switches
    let (swcs, vals) = args::swcs();
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];
       
        // Parse INT from value
        let nr_from_value = v.parse::<usize>().unwrap_or(1);

        let read_per_line = READ_PER_LINE.contains(&s.as_str());
        let read_per_char = READ_PER_CHAR.contains(&s.as_str());
        let read_from_start = READ_FROM_START.contains(&s.as_str());
        let read_from_end = READ_FROM_END.contains(&s.as_str());

        // Printing 0 lines/chars from start/end does not do anything
        if nr_from_value == 0 {
            eprintln!("Value '0' is unacceptable!");
            process::exit(1);
        }
        // Printing more characters or lines than are in the file is an horible idea
        if read_per_line && nr_from_value > lines.len() {
            eprintln!("Requested to many lines!");
            process::exit(1);
        }
        if read_per_char && nr_from_value > chars.len() {
            eprintln!("Requested to many characters!");
            process::exit(1);
        };

        // Use "lines" or "chars" depending on what user wants to print
        let thing_to_match = if read_per_line {
                &lines
        } else {
                &chars
        };

        let mut i = if read_from_end && read_per_line {
            lines.len()
        } else if read_from_end && read_per_char {
            chars.len()
        } else {
            0
        };

        if read_from_end {
            // Print from line no. 0 if "reverse" switch is being used
            let end = if s=="r"||s=="reverse" {
                0
            // Otherwise, just start printing from the number passed in a value
            } else {
                nr_from_value
            };
            while i > end-1 {
                match thing_to_match.get_key_value(&(i-1)) {
                    Some(line) => {
                        printline(line.0, line.1, read_per_line, line_number)
                    },
                    None => { eprintln!("Failed to read from input!"); process::exit(1) }
                };
                i -= 1;
            };
        }
        else if read_from_start {
            while i != nr_from_value {
                match thing_to_match.get_key_value(&i) {
                    Some(line) => {
                        printline(line.0, line.1, read_per_line, line_number)
                    },
                    None => { eprintln!("Failed to read from input!"); process::exit(1) }
                };
                i += 1;
            };
        }
        index += 1;
    };
}

fn printline(linenumber:&usize, text:&str, read_per_line:bool, line_number:bool) {
    // If line-counter is enabled, count all printed lines
    // line-counter is NOT used to show actual line number from input!
    let c = if line_number {
        format!("{}: ", linenumber+1)
    } else {
        String::new()
    };

    if read_per_line {
        println!("{c}{text}");
    } else {
        print!("{text}");
    };
}
