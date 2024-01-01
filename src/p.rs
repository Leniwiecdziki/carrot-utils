use std::fs;
use std::process;
mod libargs;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();

    if opts.is_empty() {
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }

    let mut showcount = false;
    let mut index = 0;
    while index < swcs.len() {
        let s = &swcs[index];
        let v = &vals[index];

        if s != "c" && s != "count" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        else {
            showcount = true;
            if !v.is_empty() { eprintln!("Unsupported value for a switch: {s}={v}"); process::exit(1); }
        }
        index += 1; 
    }

    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => { 
                // Create a vector that stores entire file in it
                let mut counter = 1;
                for line in f.lines() {
                    let show_counter_or_not = if showcount {
                        format!("{counter}: ")
                    }
                    else {
                        String::new()
                    };
                    println!("{show_counter_or_not}{line}");
                    counter += 1;
                }
            },
        };
        index += 1; 
    };
}
