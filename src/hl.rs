use std::fs;
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
    for s in swcs {
        if s != "v" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        else {
            verbose = true;
        }
    }

    if opts.len() < 2 {
        eprintln!("Type the name of directories to create!");
        process::exit(1);
    }
    let mut index = 1;
    let src = &opts[0];

    if fs::metadata(src).unwrap().is_dir() {
        eprintln!("{src}: Destination cannot be a directory!");
        process::exit(1);
    }

    if verbose {
        println!("Hard linking to: {src}");
    }
    while index < opts.len() {
        match fs::hard_link(src, &opts[index]) {
            Err(e) => { eprintln!("{}: Cannot hard link to destination because of an error: {:?}!", opts[index], e.kind()); },
            Ok(_) => { if verbose {println!("{}: Hard linked successfully.", opts[index]);} }
        }
    index += 1;
    }
}
