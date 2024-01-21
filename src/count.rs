use std::fs;
use std::io::{self, IsTerminal, Read};
use std::process;
use carrot_libs::args;


fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    
    let mut print = false;
    let mut line_counter = false;
    let mut char_counter = false;

    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }

    for s in swcs {
        if s != "p" && s != "print"
        && s != "l" && s != "lines"
        && s != "c" && s != "chars" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s=="p" || s=="print" {
            print = true;
        }
        if s=="l" || s=="lines" {
            line_counter = true;
        }
        if s=="c" || s=="chars" {
            char_counter = true;
        }
    }

    if !print && !line_counter && !char_counter {
        eprintln!("Type at least one action to do!");
        process::exit(1);
    }

    // Show error when there are no files requested as options by user and nothing is piped to the program
    if opts.is_empty() && io::stdin().is_terminal() {
        eprintln!("Type the name of elements to preview!");
        process::exit(1);
    }

    // Show piped stuff
    if !io::stdin().is_terminal() {
        // Save contents of STDIN to a string
        let mut contents_of_stdin = String::new();
        io::stdin().lock().read_to_string(&mut contents_of_stdin).expect("Failed to retrieve contents of stdin!");

        if print {
            printline(&contents_of_stdin);
        };
        if line_counter {
            index_lines(&contents_of_stdin);
        };
        if char_counter {
            index_chars(&contents_of_stdin);
        };
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
                if print {
                    printline(&f);
                };
                if line_counter {
                    index_lines(&f);
                };
                if char_counter {
                    index_chars(&f);
                };
            },
        };
        index += 1;
    };
}

fn printline(text:&String) {
    // Associated number (start with number 0)
    let mut idx = 1;

    for line in text.lines() {
        println!("{idx}: {line}");
        idx += 1;
    };
}

fn index_lines(text:&String) {
    println!("{}", text.lines().count());
}

fn index_chars(text:&String) {
    println!("{}", text.chars().count());
}
