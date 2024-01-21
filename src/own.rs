use std::os;
use std::path::Path;
use std::path::PathBuf;
use std::process;
use carrot_libs::kinder;
use carrot_libs::unkinder;
use carrot_libs::args;
use carrot_libs::dir;

fn main() {
    let opts = args::opts();
    let (swcs, vals) = args::swcs();

    let mut rec = false;
    let mut link = false;
    let mut verbose = false;
    let mut index = 0;
    for v in vals {
        if !v.is_empty() {
            eprintln!("None of this program's switches accepts a value."); process::exit(1); 
        } 
    }
    while index < swcs.len() {
        let s = &swcs[index];

        if s != "r" && s != "rec"
        && s != "l" && s != "link"
        && s != "v" && s != "verbose" {
            eprintln!("Unknown switch: {s}");
            process::exit(1);
        }
        if s == "r" || s == "rec" {
            rec = true;
        }
        if s == "l" || s == "link" {
            link = true;
        }
        if s == "v" || s == "verbose" {
            verbose = true;
        }
        index += 1;
    }
    // If there are no arguments passed to the program, print an error
    if opts.is_empty() {
        eprintln!("This program requires ownership IDs to set!");
        process::exit(1);
    }
    if opts.len() < 2 {
        eprintln!("This program requires at least one resource name!");
        process::exit(1);
    }

    let mut index = 1;
    let ids: Vec<_> = opts[0].split(',').collect();
    let uid = ids[0].trim();
    let gid = ids[1].trim();

    // Something is not quiet right if user requested more than two ID's
    if ids.len() > 2 {
        eprintln!("Maximally two ID's can be requested!");
        process::exit(1);
    }

    while index < opts.len() {
        // Convert ID's requested by user in arguments, to numbers of type Some(u32)
        // If user wants to skip changing UID, use None()
        let user =
            // If user set uid to "n", just use previous ownership ID
            if uid == "n" {
                None
            }
            else {
                match uid.parse::<u32>() {
                    Err(e) => {
                        eprintln!("Could not parse UID because of an error: {:?}", e.kind());
                        process::exit(1);
                    }
                    Ok(parsed_uid) => {
                        Some(parsed_uid)
                    }
                }
            };
        let group =
            if ids.len() == 1 || gid == "n" && ids.len() == 2  {
                None
            }
            else {
                match gid.parse::<u32>() {
                    Err(e) => {
                        eprintln!("{}: Could not check current GID because of an error: {:?}", &opts[index], e.kind());
                        process::exit(1);
                    }
                    Ok(parsed_gid) =>{
                        Some(parsed_gid)
                    }
                }
            };
        if verbose {
            print!("Setting ownership mode: ");
            if let Some(u) = user { print!("{u}") };
            if let Some(g) = group { print!("{g}") };
            println!()
        }
        if !PathBuf::from(&opts[index]).is_dir() || PathBuf::from(&opts[index]).is_dir() & !rec {
            changeown(&opts[index], &user, &group, &verbose, &link);
        }
        else if PathBuf::from(&opts[index]).is_dir() & rec  {
            changeown(&opts[index], &user, &group, &verbose, &link);
            browsedir(&PathBuf::from(&opts[index]), &user, &group, &rec, &verbose, &link);
        }

        index += 1;
    }
}

fn changeown(path:&str, user:&Option<u32>, group:&Option<u32>, verbose:&bool, link:&bool) {
    let command = if *link {
        os::unix::fs::chown(path, *user, *group)
    }
    else {
        os::unix::fs::lchown(path, *user, *group)
    };
    match command {
        Err(e) => eprintln!("{path}: Failed to change ownership: {:?}!", e.kind()),
        Ok(_) => {
            if *verbose {
                println!("{path}: Set successfully.")
            }
        },
    };
}

fn browsedir(path:&Path, user:&Option<u32>, group:&Option<u32>, rec:&bool, verbose:&bool, link:&bool) {
    // List where all found files will be stored
    let result = dir::browse(path);

    // Add new elements to 'result'
    for r in &result {
        changeown(r.to_str().unwrap(), user, group, verbose, link);
        if rec & r.is_dir() {
            changeown(r.to_str().unwrap(), user, group, verbose, link);
            browsedir(r, user, group, rec, verbose, link)
        }
    }
}
