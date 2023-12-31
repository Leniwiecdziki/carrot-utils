use std::fs;
use std::path::Path;
use std::process;
mod libargs;

fn main() {
    let opts = libargs::opts();
    let (swcs, vals) = libargs::swcs();
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    let mut parents = false;
    for s in swcs {
        if s != "v" && s != "verbose"
        && s != "p" && s != "parents" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "v" || s == "verbose" {
            verbose = true;
        }
        if s == "p" || s == "parents" {
            parents = true;
        }
    }

    if opts.is_empty() {
        eprintln!("Type the name of directories to create!");
        process::exit(1);
    }

    let mut index = 0;
    while index < opts.len() {

        if Path::new(&opts[index]).exists() {
            eprintln!("{}: Requested resource already exists!", opts[index]);
            index += 1;
            continue;
        }

        let command = if parents {
            fs::create_dir_all(&opts[index])
        }
        else {
            fs::create_dir(&opts[index])
        };
        match command {
            Err(e) => eprintln!("{}: Directory wasn't added because of an error: {:?}!", opts[index], e.kind()),
            _ => if verbose {println!("{}: Added successfully.", opts[index]);},
        }
        index += 1;
    }
}
