use std::fs;
use std::process;
use carrot_libs::args;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    if opts.is_empty() {
        eprintln!("This program requires at least one file name!");
        process::exit(1);
    }

    if !swcs.is_empty() || !vals.is_empty() {
        eprintln!("This program does not need any switches nor values!");
        process::exit(1);
    }

    let mut index = 0;
    while index < opts.len() {
        match fs::read_to_string(&opts[index]) {
            Err(e) => { 
                eprintln!("{}: Cannot preview the file: {:?}!", opts[index], e.kind());
                index += 1;
            },
            Ok(f) => { 
                for line in f.lines() {
                    println!("{line}");
                }
            },
        };
        index += 1; 
    };
}
