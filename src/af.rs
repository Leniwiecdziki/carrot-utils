use std::fs;
use std::io::ErrorKind;
use std::process;
use carrot_libs::args;

fn main() {

    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    let mut ignore = false;
    for s in swcs {
        if s != "v" && s != "verbose"
        && s != "i" && s != "ignore" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "v" || s == "verbose" {
            verbose = true;
        }
        if s == "i" || s == "ignore" {
            ignore = true;
        }
    }

    if opts.is_empty() {
        eprintln!("Type the name of directories to create!");
        process::exit(1);
    }

    let mut index = 0;
    while index < opts.len() {
        match fs::File::create(&opts[index]) {
            Err(e) => {
                if !ignore && e.kind() != ErrorKind::AlreadyExists {
                    eprintln!("{}: Directory wasn't added because of an error: {:?}!", opts[index], e.kind());
                };
            },
            _ => {
                if verbose {
                    println!("{}: Added successfully.", opts[index]);
                }
            },
        }
        index += 1;
    }
}
