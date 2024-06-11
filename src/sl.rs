use std::process;
use std::os::unix::fs;
use carrot_libs::args;

fn main() {
    // Import all arguments from command line to 'args' variable
    let opts = args::opts();
    let (swcs, vals) = args::swcs();
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    let mut verbose = false;
    for s in swcs {
        if s != "v" && s != "verbose" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        else {
            verbose = true;
        }
    }
    // If there are no arguments passed to the program, print an error
    if opts.len() < 2 {
        eprintln!("Type the name of directories to create!");
        process::exit(1);
    }
    let mut index = 1;
    let src = &opts[0];

    if verbose {        
        println!("Soft linking to: {src}");
    }
    while index < opts.len() {
        match fs::symlink(src, &opts[index]) {
            Err(e) => { eprintln!("{}: Cannot soft link to destination because of an error: {:?}!", opts[index], e.kind()); },
            Ok(_) => if verbose { println!("{}: Soft linked successfully.", opts[index]); }
        }
    index += 1;
    }
}
